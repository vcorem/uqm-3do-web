//! Private two-peer relay. The browser's wire protocol is shared with the Node
//! reference implementation; game packets are never parsed or modified here.
//!
//! Framing is deliberately explicit: no automatic pong writer may bypass the
//! control-frame quota or bounded outgoing queue. RFC 6455 validation happens
//! before payload allocation, and fragmented messages have a separate count cap.
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use sha1::{Digest, Sha1};
use std::{
    collections::{HashMap, HashSet},
    env, io,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream},
    sync::{mpsc, watch, Semaphore},
    time::{sleep_until, timeout, Instant},
};

const ISOLATION: &str = "Cross-Origin-Opener-Policy: same-origin\r\nCross-Origin-Embedder-Policy: require-corp\r\nCross-Origin-Resource-Policy: same-origin\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\n";
const OTHER_LEFT: &str = "The other player disconnected.";
const MAX_HEADERS: usize = 8192;
const MAX_FRAGMENTS: usize = 128;
const MAX_QUEUE_FRAMES: usize = 1024;

#[derive(Clone)]
struct Config {
    host: String,
    port: u16,
    origins: Option<HashSet<String>>,
    dist: Option<PathBuf>,
    max_payload: usize,
    max_buffered: usize,
    max_rooms: usize,
    max_ids: usize,
    max_ip: usize,
    waiting: Duration,
    lifetime: Duration,
    heartbeat: Duration,
    idle: Duration,
    latency: Vec<Duration>,
    test_events: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 9999,
            origins: None,
            dist: Some(PathBuf::from("dist")),
            max_payload: 65536,
            max_buffered: 262144,
            max_rooms: 128,
            max_ids: 10000,
            max_ip: 16,
            waiting: Duration::from_millis(120000),
            lifetime: Duration::from_millis(14400000),
            heartbeat: Duration::from_millis(15000),
            idle: Duration::from_millis(900000),
            latency: vec![Duration::ZERO],
            test_events: false,
        }
    }
}
fn origin_is_exact(value: &str) -> bool {
    url::Url::parse(value).is_ok_and(|url| {
        matches!(url.scheme(), "http" | "https") && url.origin().ascii_serialization() == value
    })
}
fn bounded(
    options: &Value,
    name: &str,
    fallback: usize,
    min: usize,
    max: usize,
) -> Result<usize, String> {
    match options.get(name) {
        None => Ok(fallback),
        Some(value) => value
            .as_u64()
            .and_then(|n| usize::try_from(n).ok())
            .filter(|n| *n >= min && *n <= max)
            .ok_or_else(|| format!("Invalid {name}")),
    }
}
impl Config {
    fn test_options(&mut self, options: Value) -> Result<(), String> {
        let map = options
            .as_object()
            .ok_or("Test options must be an object")?;
        let names = [
            "maxPayload",
            "maxBufferedBytes",
            "maxRooms",
            "maxRoomIds",
            "maxPerIp",
            "waitingMs",
            "lifetimeMs",
            "heartbeatMs",
            "idleMs",
            "latencyPatternMs",
            "distDir",
        ];
        if map.keys().any(|name| !names.contains(&name.as_str())) {
            return Err("Unknown test option".into());
        }
        self.max_payload = bounded(&options, "maxPayload", 65536, 2048, 65536)?;
        self.max_buffered = bounded(
            &options,
            "maxBufferedBytes",
            262144,
            self.max_payload,
            1048576,
        )?;
        self.max_rooms = bounded(&options, "maxRooms", 128, 1, 1024)?;
        self.max_ids = bounded(&options, "maxRoomIds", 10000, self.max_rooms, 1000000)?;
        self.max_ip = bounded(&options, "maxPerIp", 16, 2, 2048)?;
        self.waiting =
            Duration::from_millis(bounded(&options, "waitingMs", 120000, 10, 3600000)? as u64);
        self.lifetime =
            Duration::from_millis(bounded(&options, "lifetimeMs", 14400000, 10, 86400000)? as u64);
        self.heartbeat =
            Duration::from_millis(bounded(&options, "heartbeatMs", 15000, 10, 120000)? as u64);
        self.idle =
            Duration::from_millis(bounded(&options, "idleMs", 900000, 10, 86400000)? as u64);
        if let Some(value) = options.get("latencyPatternMs") {
            let array = value
                .as_array()
                .filter(|a| !a.is_empty() && a.len() <= 100)
                .ok_or("Invalid latencyPatternMs")?;
            self.latency = array
                .iter()
                .map(|v| {
                    v.as_u64()
                        .filter(|n| *n <= 2000)
                        .map(Duration::from_millis)
                        .ok_or("Invalid latencyPatternMs")
                })
                .collect::<Result<Vec<_>, _>>()?;
        }
        if let Some(value) = options.get("distDir") {
            if value == &Value::Bool(false) {
                self.dist = None;
            } else {
                return Err("Test distDir must be false; use --dist for paths".into());
            }
        }
        Ok(())
    }
    fn parse() -> Result<Option<Self>, String> {
        let mut config = Self::default();
        let mut args = env::args().skip(1);
        while let Some(flag) = args.next() {
            if flag == "--help" {
                println!("Usage: uqm-relay [--host 127.0.0.1] [--port 9999] [--origin https://game.example] [--dist DIR]\n\n--origin can be repeated. Test harness only: --test-events --test-options JSON");
                return Ok(None);
            }
            if flag == "--test-events" {
                config.test_events = true;
                continue;
            }
            if !["--host", "--port", "--origin", "--dist", "--test-options"]
                .contains(&flag.as_str())
            {
                return Err(format!("Unknown option: {flag}"));
            }
            let value = args
                .next()
                .ok_or_else(|| format!("Incomplete option: {flag}"))?;
            match flag.as_str() {
                "--host" => config.host = value,
                "--port" => config.port = value.parse().map_err(|_| "Invalid port")?,
                "--origin" => {
                    if !origin_is_exact(&value) {
                        return Err("Allowed origins must be exact http(s) origins without trailing slashes".into());
                    }
                    config
                        .origins
                        .get_or_insert_with(HashSet::new)
                        .insert(value);
                }
                "--dist" => config.dist = Some(PathBuf::from(value)),
                "--test-options" => config
                    .test_options(serde_json::from_str(&value).map_err(|_| "Invalid test JSON")?)?,
                _ => unreachable!(),
            }
        }
        Ok(Some(config))
    }
}

#[derive(Clone)]
struct Closing {
    notice: Option<Value>,
    code: u16,
    reason: String,
}
struct Outgoing {
    opcode: u8,
    data: Vec<u8>,
    due: Instant,
    charge: usize,
}
struct Peer {
    id: usize,
    ip: IpAddr,
    tx: mpsc::Sender<Outgoing>,
    controls: mpsc::Sender<Outgoing>,
    closing: watch::Sender<Option<Closing>>,
    closed: AtomicBool,
    alive: AtomicBool,
    bytes: AtomicUsize,
    frames: AtomicUsize,
    peak_bytes: AtomicUsize,
}
impl Peer {
    fn enqueue(&self, opcode: u8, data: Vec<u8>, due: Instant, limit: usize) -> bool {
        if self.closed.load(Ordering::Acquire) {
            return false;
        }
        let charge = data.len() + if opcode >= 8 { 2 } else { 0 };
        // Reservation covers channel, delayed/in-flight frame and socket write.
        if self
            .bytes
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |bytes| {
                bytes.checked_add(charge).filter(|n| *n <= limit)
            })
            .is_err()
        {
            return false;
        }
        let bytes = self.bytes.load(Ordering::Acquire);
        self.peak_bytes.fetch_max(bytes, Ordering::Relaxed);
        if self.frames.fetch_add(1, Ordering::AcqRel) >= MAX_QUEUE_FRAMES {
            self.frames.fetch_sub(1, Ordering::AcqRel);
            self.bytes.fetch_sub(charge, Ordering::AcqRel);
            return false;
        }
        let channel = if opcode >= 8 {
            &self.controls
        } else {
            &self.tx
        };
        if channel
            .try_send(Outgoing {
                opcode,
                data,
                due,
                charge,
            })
            .is_err()
        {
            self.frames.fetch_sub(1, Ordering::AcqRel);
            self.bytes.fetch_sub(charge, Ordering::AcqRel);
            return false;
        }
        true
    }
    fn json(&self, value: Value, limit: usize) -> bool {
        self.enqueue(
            1,
            serde_json::to_vec(&value).expect("JSON notification"),
            Instant::now(),
            limit,
        )
    }
    fn close(&self, notice: Option<Value>, code: u16, reason: &str) {
        if self
            .closed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.closing.send_replace(Some(Closing {
                notice,
                code,
                reason: reason.into(),
            }));
        }
    }
}
struct Room {
    host: Arc<Peer>,
    guest: Option<Arc<Peer>>,
    created: Instant,
    last_data: Instant,
}
#[derive(Default)]
struct Registry {
    rooms: HashMap<String, Room>,
    used: HashSet<String>,
    peers: HashMap<usize, Arc<Peer>>,
    ips: HashMap<IpAddr, usize>,
    next_id: usize,
    peak_queue_bytes: usize,
    stopping: bool,
}
struct App {
    config: Config,
    state: Mutex<Registry>,
}
impl App {
    fn stats(&self, state: &Registry) {
        if self.config.test_events {
            let peak = state
                .peers
                .values()
                .fold(state.peak_queue_bytes, |peak, peer| {
                    peak.max(peer.peak_bytes.load(Ordering::Relaxed))
                });
            println!(
                "{}",
                json!({"event":"stats", "rooms":state.rooms.len(),
                "connections":state.peers.len(), "usedRoomIds":state.used.len(), "peakQueuedBytes":peak})
            );
        }
    }
    fn end_room_locked(
        &self,
        state: &mut Registry,
        room_id: &str,
        departing: Option<usize>,
        message: &str,
    ) {
        if let Some(room) = state.rooms.remove(room_id) {
            for peer in [Some(room.host), room.guest].into_iter().flatten() {
                if departing != Some(peer.id) {
                    peer.close(
                        Some(json!({"type":"peer-left", "message":message})),
                        1001,
                        "Room closed",
                    );
                }
            }
            self.stats(state);
        }
    }
    fn reject(&self, peer: &Arc<Peer>, room_id: Option<&str>, message: &str, code: u16) {
        peer.close(
            Some(json!({"type":"error", "message":message})),
            code,
            message,
        );
        if let Some(id) = room_id {
            self.end_room_locked(
                &mut self.state.lock().unwrap(),
                id,
                Some(peer.id),
                OTHER_LEFT,
            );
        }
    }
    fn attach(&self, peer: &Arc<Peer>, target: &str) -> Option<(String, bool)> {
        let Some((id, host)) = room_path(target) else {
            self.reject(peer, None, "Invalid room URL or role.", 1008);
            return None;
        };
        let mut state = self.state.lock().unwrap();
        if state.stopping || peer.closed.load(Ordering::Acquire) {
            peer.close(None, 1001, "Server is stopping.");
            return None;
        }
        let error = if host {
            if state.used.contains(id) {
                Some("Room already exists or has expired. Create a new room.")
            } else if state.rooms.len() >= self.config.max_rooms
                || state.used.len() >= self.config.max_ids
            {
                Some("Room limit reached.")
            } else {
                state.used.insert(id.into());
                state.rooms.insert(
                    id.into(),
                    Room {
                        host: peer.clone(),
                        guest: None,
                        created: Instant::now(),
                        last_data: Instant::now(),
                    },
                );
                None
            }
        } else {
            match state.rooms.get_mut(id) {
                None => Some("Room does not exist or has expired."),
                Some(room) if room.host.closed.load(Ordering::Acquire) => {
                    Some("Room does not exist or has expired.")
                }
                Some(room) if room.guest.is_some() => Some("Room already has two players."),
                Some(room) => {
                    room.guest = Some(peer.clone());
                    room.last_data = Instant::now();
                    None
                }
            }
        };
        if let Some(message) = error {
            drop(state);
            self.reject(peer, None, message, 1008);
            return None;
        }
        peer.json(json!({"type":"waiting"}), self.config.max_buffered);
        let room = state.rooms.get(id).unwrap();
        if let Some(guest) = &room.guest {
            room.host.json(
                json!({"type":"ready", "role":"host"}),
                self.config.max_buffered,
            );
            guest.json(
                json!({"type":"ready", "role":"guest"}),
                self.config.max_buffered,
            );
        }
        self.stats(&state);
        Some((id.into(), host))
    }
    fn cleanup(&self, peer: &Arc<Peer>, room_id: Option<&str>) {
        let mut state = self.state.lock().unwrap();
        if let Some(id) = room_id {
            self.end_room_locked(&mut state, id, Some(peer.id), OTHER_LEFT);
        }
        state.peak_queue_bytes = state
            .peak_queue_bytes
            .max(peer.peak_bytes.load(Ordering::Relaxed));
        if state.peers.remove(&peer.id).is_some() {
            if let Some(count) = state.ips.get_mut(&peer.ip) {
                *count -= 1;
                if *count == 0 {
                    state.ips.remove(&peer.ip);
                }
            }
        }
        self.stats(&state);
    }
    fn tick(&self, heartbeat: bool) {
        let now = Instant::now();
        let mut state = self.state.lock().unwrap();
        let expired: Vec<_> = state
            .rooms
            .iter()
            .filter_map(|(id, room)| {
                if now.duration_since(room.created) >= self.config.lifetime {
                    Some((id.clone(), 0))
                } else if room.guest.is_none()
                    && now.duration_since(room.created) >= self.config.waiting
                {
                    Some((id.clone(), 1))
                } else if room.guest.is_some()
                    && now.duration_since(room.last_data) > self.config.idle
                {
                    Some((id.clone(), 2))
                } else {
                    None
                }
            })
            .collect();
        for (id, kind) in expired {
            if kind == 1 {
                let host = state.rooms.get(&id).unwrap().host.clone();
                host.close(
                    Some(
                        json!({"type":"error", "message":"Nobody joined before the room expired."}),
                    ),
                    1001,
                    "Room closed",
                );
                self.end_room_locked(&mut state, &id, Some(host.id), OTHER_LEFT);
            } else {
                self.end_room_locked(
                    &mut state,
                    &id,
                    None,
                    if kind == 0 {
                        "Room time limit reached."
                    } else {
                        "Room was idle for too long."
                    },
                );
            }
        }
        if heartbeat {
            let peers: Vec<_> = state.peers.values().cloned().collect();
            for peer in peers {
                if peer.closed.load(Ordering::Acquire) {
                    continue;
                }
                if !peer.alive.swap(false, Ordering::AcqRel)
                    || !peer.enqueue(9, Vec::new(), now, self.config.max_buffered)
                {
                    peer.close(None, 1001, "Room closed");
                    let id = state
                        .rooms
                        .iter()
                        .find(|(_, room)| {
                            room.host.id == peer.id
                                || room.guest.as_ref().is_some_and(|p| p.id == peer.id)
                        })
                        .map(|(id, _)| id.clone());
                    if let Some(id) = id {
                        self.end_room_locked(&mut state, &id, Some(peer.id), OTHER_LEFT);
                    }
                }
            }
        }
    }
    fn stop(&self) {
        let mut state = self.state.lock().unwrap();
        state.stopping = true;
        let ids: Vec<_> = state.rooms.keys().cloned().collect();
        for id in ids {
            self.end_room_locked(&mut state, &id, None, "Server is stopping.");
        }
        for peer in state.peers.values() {
            peer.close(None, 1001, "Server is stopping.");
        }
    }
}
fn room_path(target: &str) -> Option<(&str, bool)> {
    let tail = target.strip_prefix("/netplay/")?;
    let (id, role) = tail.split_once("?role=")?;
    // Keep the game/edition prefix in active and retired room keys. None of
    // these engines may share streams, even when the random token is equal.
    let token = id
        .strip_prefix("hd/")
        .or_else(|| id.strip_prefix("cosmoclash/"))
        .unwrap_or(id);
    if token.len() != 32
        || !token
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return None;
    }
    match role {
        "host" => Some((id, true)),
        "guest" => Some((id, false)),
        _ => None,
    }
}

struct Request {
    method: String,
    target: String,
    headers: HashMap<String, String>,
}
async fn read_request<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<Request> {
    // One byte at a time keeps any coalesced first WebSocket frame in BufReader.
    let mut bytes = Vec::with_capacity(1024);
    while bytes.len() < MAX_HEADERS {
        bytes.push(reader.read_u8().await?);
        if bytes.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    if !bytes.ends_with(b"\r\n\r\n") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Headers too large",
        ));
    }
    let value = std::str::from_utf8(&bytes).map_err(|_| io::ErrorKind::InvalidData)?;
    let mut lines = value.split("\r\n");
    let first: Vec<_> = lines.next().unwrap_or("").split(' ').collect();
    if first.len() != 3
        || first[2] != "HTTP/1.1"
        || !first[1].starts_with('/')
        || first[1].bytes().any(|b| b <= 32 || b == 127)
    {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let mut headers = HashMap::new();
    for line in lines.take_while(|line| !line.is_empty()) {
        let (name, value) = line.split_once(':').ok_or(io::ErrorKind::InvalidData)?;
        if name.is_empty()
            || !name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
            || value.bytes().any(|b| (b < 32 && b != 9) || b == 127)
        {
            return Err(io::ErrorKind::InvalidData.into());
        }
        let name = name.to_ascii_lowercase();
        // Reject ambiguous security/framing headers instead of choosing one.
        if headers.insert(name, value.trim().to_string()).is_some() {
            return Err(io::ErrorKind::InvalidData.into());
        }
    }
    if !headers.contains_key("host")
        || headers.contains_key("transfer-encoding")
        || headers.get("content-length").is_some_and(|v| v != "0")
    {
        return Err(io::ErrorKind::InvalidData.into());
    }
    Ok(Request {
        method: first[0].into(),
        target: first[1].into(),
        headers,
    })
}
async fn response<W: AsyncWrite + Unpin>(
    writer: &mut W,
    status: &str,
    body: &[u8],
    extra: &str,
    head: bool,
) -> io::Result<()> {
    writer.write_all(format!("HTTP/1.1 {status}\r\nConnection: close\r\nContent-Length: {}\r\n{ISOLATION}{extra}\r\n", body.len()).as_bytes()).await?;
    if !head {
        writer.write_all(body).await?;
    }
    writer.shutdown().await
}
async fn deny<W: AsyncWrite + Unpin>(writer: &mut W, status: &str, message: &str) {
    let body = serde_json::to_vec(&json!({"type":"error", "message":message})).unwrap();
    let _ = timeout(
        Duration::from_secs(1),
        response(
            writer,
            status,
            &body,
            "Content-Type: application/json\r\n",
            false,
        ),
    )
    .await;
}
fn decode_path(raw: &str) -> Option<String> {
    let mut out = Vec::with_capacity(raw.len());
    let bytes = raw.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = |b: u8| (b as char).to_digit(16).map(|n| n as u8);
            out.push(hex(*bytes.get(i + 1)?)? * 16 + hex(*bytes.get(i + 2)?)?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}
fn mime(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        "glb" => "model/gltf-binary",
        "ogg" => "audio/ogg",
        "wav" => "audio/wav",
        "json" => "application/json",
        "css" => "text/css; charset=utf-8",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
async fn static_file<W: AsyncWrite + Unpin>(
    writer: &mut W,
    request: &Request,
    dist: Option<&Path>,
) -> io::Result<()> {
    let head = request.method == "HEAD";
    if request.method != "GET" && !head {
        return response(
            writer,
            "405 Method Not Allowed",
            b"",
            "Allow: GET, HEAD\r\n",
            false,
        )
        .await;
    }
    let resolve = async {
        let dist = dist.ok_or(io::ErrorKind::NotFound)?;
        let decoded = decode_path(request.target.split('?').next().unwrap_or(""))
            .ok_or(io::ErrorKind::NotFound)?;
        if !decoded.starts_with('/')
            || decoded.contains(['\\', '\0'])
            || decoded.split('/').any(|s| s.starts_with('.'))
        {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
        let root = tokio::fs::canonicalize(dist).await?;
        let path = tokio::fs::canonicalize(root.join(if decoded == "/" {
            "index.html"
        } else {
            &decoded[1..]
        }))
        .await?;
        if !path.starts_with(&root) || path == root {
            return Err(io::ErrorKind::NotFound.into());
        }
        // Reject FIFOs/devices before open: opening a FIFO for reading can
        // otherwise block a filesystem worker indefinitely, including shutdown.
        if !tokio::fs::metadata(&path).await?.is_file() {
            return Err(io::ErrorKind::NotFound.into());
        }
        let mut options = tokio::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        // The document root can change between metadata and open. Nonblocking
        // open protects against a FIFO swap; NOFOLLOW rejects a leaf symlink
        // swapped into the already-canonicalized path. Check the opened handle
        // again below. These flags do not change regular-file reads.
        options.custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW);
        let file = options.open(&path).await?;
        let info = file.metadata().await?;
        if !info.is_file() {
            return Err(io::ErrorKind::NotFound.into());
        }
        Ok::<_, io::Error>((file, info.len(), path))
    }
    .await;
    let Ok((mut file, length, path)) = resolve else {
        return response(writer, "404 Not Found", b"Not found", "", head).await;
    };
    writer.write_all(format!("HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: {length}\r\nContent-Type: {}\r\n{ISOLATION}\r\n", mime(&path)).as_bytes()).await?;
    if !head {
        // Large HD bundles must work on slow links. Bound inactivity for each
        // chunk, never the total duration of a progressing download.
        let mut buffer = [0u8; 65536];
        loop {
            let count = timeout(Duration::from_secs(30), file.read(&mut buffer)).await??;
            if count == 0 {
                break;
            }
            timeout(Duration::from_secs(30), writer.write_all(&buffer[..count])).await??;
        }
    }
    writer.shutdown().await
}

#[derive(Debug)]
enum FrameError {
    Io,
    Protocol,
    TooLarge,
    InvalidText,
}
impl From<io::Error> for FrameError {
    fn from(_: io::Error) -> Self {
        Self::Io
    }
}
struct Frame {
    opcode: u8,
    final_frame: bool,
    data: Vec<u8>,
}
async fn read_frame<R: AsyncRead + Unpin>(
    reader: &mut R,
    remaining: usize,
) -> Result<Frame, FrameError> {
    let first = reader.read_u8().await?;
    let second = reader.read_u8().await?;
    let opcode = first & 15;
    let final_frame = first & 128 != 0;
    if first & 112 != 0 || second & 128 == 0 || !matches!(opcode, 0 | 1 | 2 | 8 | 9 | 10) {
        return Err(FrameError::Protocol);
    }
    let short = second & 127;
    let length = match short {
        126 => {
            let n = reader.read_u16().await? as u64;
            if n < 126 {
                return Err(FrameError::Protocol);
            }
            n
        }
        127 => {
            let n = reader.read_u64().await?;
            if n < 65536 || n >> 63 != 0 {
                return Err(FrameError::Protocol);
            }
            n
        }
        n => n as u64,
    };
    if opcode >= 8 && (!final_frame || length > 125) {
        return Err(FrameError::Protocol);
    }
    if opcode < 8 && length > remaining as u64 {
        return Err(FrameError::TooLarge);
    }
    let mut mask = [0; 4];
    reader.read_exact(&mut mask).await?;
    // Maximum allocation is configured <=64 KiB, or 125 for a control frame.
    let mut data = vec![0; length as usize];
    reader.read_exact(&mut data).await?;
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= mask[i & 3];
    }
    if opcode == 8 {
        if data.len() == 1 {
            return Err(FrameError::Protocol);
        }
        if data.len() >= 2 {
            let code = u16::from_be_bytes([data[0], data[1]]);
            if !matches!(code, 1000..=1003 | 1007..=1014 | 3000..=4999) {
                return Err(FrameError::Protocol);
            }
            if std::str::from_utf8(&data[2..]).is_err() {
                return Err(FrameError::InvalidText);
            }
        }
    }
    Ok(Frame {
        opcode,
        final_frame,
        data,
    })
}
async fn write_frame<W: AsyncWrite + Unpin>(
    writer: &mut W,
    opcode: u8,
    data: &[u8],
) -> io::Result<()> {
    let mut header = [0u8; 10];
    header[0] = 128 | opcode;
    let size = if data.len() < 126 {
        header[1] = data.len() as u8;
        2
    } else if data.len() <= u16::MAX as usize {
        header[1] = 126;
        header[2..4].copy_from_slice(&(data.len() as u16).to_be_bytes());
        4
    } else {
        header[1] = 127;
        header[2..10].copy_from_slice(&(data.len() as u64).to_be_bytes());
        10
    };
    writer.write_all(&header[..size]).await?;
    writer.write_all(data).await
}
async fn write_closing(writer: &mut OwnedWriteHalf, closing: Closing) {
    let _ = timeout(Duration::from_secs(1), async {
        if let Some(notice) = closing.notice {
            write_frame(writer, 1, &serde_json::to_vec(&notice).unwrap()).await?;
        }
        let mut bytes = if closing.code == 1005 {
            Vec::new() // Echo a client's empty close frame without a status code.
        } else {
            closing.code.to_be_bytes().to_vec()
        };
        // Current reasons are ASCII; keep a valid UTF-8 boundary for future ones.
        for ch in closing.reason.chars() {
            if bytes.len() + ch.len_utf8() > 125 {
                break;
            }
            let mut buffer = [0; 4];
            bytes.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes());
        }
        write_frame(writer, 8, &bytes).await?;
        writer.shutdown().await
    })
    .await;
}
async fn writer_loop(
    mut writer: OwnedWriteHalf,
    peer: Arc<Peer>,
    mut rx: mpsc::Receiver<Outgoing>,
    mut controls: mpsc::Receiver<Outgoing>,
) {
    let mut closing = peer.closing.subscribe();
    let mut delayed = None;
    loop {
        let decision = closing.borrow().clone();
        if let Some(value) = decision {
            write_closing(&mut writer, value).await;
            break;
        }
        let message = if let Some(message) = delayed.take() {
            message
        } else {
            tokio::select! {
                biased;
                _ = closing.changed() => continue,
                message = controls.recv() => { let Some(message) = message else { break; }; message },
                message = rx.recv() => { let Some(message) = message else { break; }; message },
            }
        };
        // Control frames bypass artificial data delay while sharing its strict
        // byte/frame accounting, matching the reference relay's heartbeat.
        let message = if message.opcode < 8 {
            tokio::select! {
                biased;
                _ = closing.changed() => continue,
                control = controls.recv() => {
                    let Some(control) = control else { break; };
                    delayed = Some(message);
                    control
                },
                _ = sleep_until(message.due) => message,
            }
        } else {
            message
        };
        // Once a frame header is on the wire, never cancel midway and
        // append a close frame into its payload. Finish it or drop TCP.
        let result = timeout(
            Duration::from_secs(1),
            write_frame(&mut writer, message.opcode, &message.data),
        )
        .await;
        peer.bytes.fetch_sub(message.charge, Ordering::AcqRel);
        peer.frames.fetch_sub(1, Ordering::AcqRel);
        if !matches!(result, Ok(Ok(()))) {
            peer.close(None, 1001, "Room closed");
            break;
        }
    }
    // Dropping receiver discards every queued game packet after teardown.
}

struct Rate {
    start: Instant,
    bytes: usize,
    messages: usize,
}
impl Rate {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            bytes: 0,
            messages: 0,
        }
    }
    fn add(&mut self, bytes: usize, max_bytes: usize, max_messages: usize) -> bool {
        let now = Instant::now();
        if now.duration_since(self.start) >= Duration::from_secs(1) {
            self.start = now;
            self.bytes = 0;
            self.messages = 0;
        }
        self.bytes += bytes;
        self.messages += 1;
        self.bytes <= max_bytes && self.messages <= max_messages
    }
}
async fn reader_loop<R: AsyncRead + Unpin>(
    reader: &mut R,
    app: &App,
    peer: &Arc<Peer>,
    room_id: &str,
    host: bool,
) {
    let mut closing = peer.closing.subscribe();
    let mut binary_rate = Rate::new();
    let mut control_rate = Rate::new();
    let mut fragment_opcode = None;
    let mut fragments = 0;
    let mut payload = Vec::new();
    let mut sequence = 0;
    let mut last_due = Instant::now();
    loop {
        if peer.closed.load(Ordering::Acquire) {
            break;
        }
        let result = tokio::select! {
            biased;
            _ = closing.changed() => break,
            value = read_frame(reader, app.config.max_payload - payload.len()) => value,
        };
        let frame = match result {
            Ok(frame) => frame,
            Err(error) => {
                let code = match error {
                    FrameError::TooLarge => 1009,
                    FrameError::InvalidText => 1007,
                    FrameError::Protocol => 1002,
                    FrameError::Io => 1001,
                };
                peer.close(None, code, "Invalid WebSocket frame");
                break;
            }
        };
        if frame.opcode >= 8 {
            if frame.opcode == 8 {
                if frame.data.is_empty() {
                    peer.close(None, 1005, "");
                } else {
                    let code = u16::from_be_bytes([frame.data[0], frame.data[1]]);
                    // read_frame already checked code and reason UTF-8.
                    peer.close(None, code, std::str::from_utf8(&frame.data[2..]).unwrap());
                }
                break;
            }
            let max_controls =
                32.max(1000_u128.div_ceil(app.config.heartbeat.as_millis()) as usize + 4);
            if !control_rate.add(frame.data.len() + 2, 4096, max_controls) {
                app.reject(peer, Some(room_id), "Control frame rate exceeded.", 1008);
                break;
            }
            if frame.opcode == 10 {
                peer.alive.store(true, Ordering::Release);
            } else if !peer.enqueue(10, frame.data, Instant::now(), app.config.max_buffered) {
                app.reject(peer, Some(room_id), "Peer is too slow.", 1008);
                break;
            }
            continue;
        }
        if frame.opcode == 0 {
            if fragment_opcode.is_none() {
                peer.close(None, 1002, "Unexpected continuation");
                break;
            }
        } else if fragment_opcode.is_some() {
            peer.close(None, 1002, "Interrupted fragmented message");
            break;
        } else {
            fragment_opcode = Some(frame.opcode);
        }
        fragments += 1;
        if fragments > MAX_FRAGMENTS {
            peer.close(None, 1009, "Too many fragments");
            break;
        }
        payload.extend_from_slice(&frame.data);
        if !frame.final_frame {
            continue;
        }
        let opcode = fragment_opcode.take().unwrap();
        fragments = 0;
        if opcode == 1 && std::str::from_utf8(&payload).is_err() {
            peer.close(None, 1007, "Invalid UTF-8");
            break;
        }
        let data = std::mem::take(&mut payload);
        let mut state = app.state.lock().unwrap();
        let Some(room) = state.rooms.get_mut(room_id) else {
            break;
        };
        let Some(guest) = &room.guest else {
            drop(state);
            app.reject(
                peer,
                Some(room_id),
                "Wait for the other player before sending game data.",
                1008,
            );
            break;
        };
        if opcode != 2 || data.is_empty() {
            drop(state);
            app.reject(
                peer,
                Some(room_id),
                "Only nonempty binary game data is accepted.",
                1008,
            );
            break;
        }
        if !binary_rate.add(data.len(), 1048576, 1000) {
            drop(state);
            app.reject(peer, Some(room_id), "Game data rate exceeded.", 1008);
            break;
        }
        let destination = if host { guest } else { &room.host };
        let due = (Instant::now() + app.config.latency[sequence % app.config.latency.len()])
            .max(last_due);
        sequence = sequence.wrapping_add(1);
        last_due = due;
        if !destination.enqueue(2, data, due, app.config.max_buffered) {
            drop(state);
            app.reject(peer, Some(room_id), "Peer is too slow.", 1008);
            break;
        }
        room.last_data = Instant::now();
    }
}

async fn connection(stream: TcpStream, address: SocketAddr, app: Arc<App>) {
    let _ = stream.set_nodelay(true);
    let (read, mut write) = stream.into_split();
    let mut reader = BufReader::with_capacity(MAX_HEADERS, read);
    let request = match timeout(Duration::from_secs(10), read_request(&mut reader)).await {
        Ok(Ok(request)) => request,
        _ => {
            deny(&mut write, "400 Bad Request", "Invalid HTTP request.").await;
            return;
        }
    };
    let upgrade = request
        .headers
        .get("upgrade")
        .is_some_and(|v| v.eq_ignore_ascii_case("websocket"));
    if !upgrade {
        let _ = static_file(&mut write, &request, app.config.dist.as_deref()).await;
        return;
    }
    let Some(origin) = request
        .headers
        .get("origin")
        .filter(|value| origin_is_exact(value))
    else {
        deny(&mut write, "403 Forbidden", "Origin required.").await;
        return;
    };
    let own_origin = format!("http://{}", request.headers["host"]);
    if !app
        .config
        .origins
        .as_ref()
        .map_or(origin == &own_origin, |origins| origins.contains(origin))
    {
        deny(&mut write, "403 Forbidden", "Origin is not allowed.").await;
        return;
    }
    if request.headers.contains_key("sec-websocket-protocol") {
        deny(
            &mut write,
            "400 Bad Request",
            "Subprotocols are unsupported.",
        )
        .await;
        return;
    }
    let Some(key) = request.headers.get("sec-websocket-key") else {
        deny(&mut write, "400 Bad Request", "WebSocket key required.").await;
        return;
    };
    let valid_key = STANDARD
        .decode(key)
        .is_ok_and(|bytes| bytes.len() == 16 && STANDARD.encode(bytes) == *key);
    let valid_connection = request.headers.get("connection").is_some_and(|s| {
        s.split(',')
            .any(|part| part.trim().eq_ignore_ascii_case("upgrade"))
    });
    if request.method != "GET"
        || request
            .headers
            .get("sec-websocket-version")
            .map(String::as_str)
            != Some("13")
        || !valid_key
        || !valid_connection
    {
        deny(
            &mut write,
            "400 Bad Request",
            "Invalid WebSocket handshake.",
        )
        .await;
        return;
    }
    let (tx, rx) = mpsc::channel(MAX_QUEUE_FRAMES);
    let (controls, controls_rx) = mpsc::channel(64);
    let (closing, _) = watch::channel(None);
    let peer = {
        let mut state = app.state.lock().unwrap();
        if state.stopping
            || state.peers.len() >= app.config.max_rooms * 2
            || state.ips.get(&address.ip()).copied().unwrap_or(0) >= app.config.max_ip
        {
            None
        } else {
            state.next_id += 1;
            let peer = Arc::new(Peer {
                id: state.next_id,
                ip: address.ip(),
                tx,
                controls,
                closing,
                closed: AtomicBool::new(false),
                alive: AtomicBool::new(true),
                bytes: AtomicUsize::new(0),
                frames: AtomicUsize::new(0),
                peak_bytes: AtomicUsize::new(0),
            });
            *state.ips.entry(address.ip()).or_default() += 1;
            state.peers.insert(peer.id, peer.clone());
            app.stats(&state);
            Some(peer)
        }
    };
    let Some(peer) = peer else {
        deny(
            &mut write,
            "503 Service Unavailable",
            "Connection limit reached.",
        )
        .await;
        return;
    };
    let mut digest = Sha1::new();
    digest.update(key);
    digest.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let handshake = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", STANDARD.encode(digest.finalize()));
    if !matches!(
        timeout(
            Duration::from_secs(1),
            write.write_all(handshake.as_bytes())
        )
        .await,
        Ok(Ok(()))
    ) {
        app.cleanup(&peer, None);
        return;
    }
    let writer = tokio::spawn(writer_loop(write, peer.clone(), rx, controls_rx));
    let room = app.attach(&peer, &request.target);
    if let Some((id, host)) = &room {
        reader_loop(&mut reader, &app, &peer, id, *host).await;
    }
    peer.close(None, 1001, "Room closed");
    // Retire the room now, but keep its connection/IP reservation until the
    // final write finishes so slow closing sockets cannot bypass those caps.
    if let Some((id, _)) = &room {
        app.end_room_locked(
            &mut app.state.lock().unwrap(),
            id,
            Some(peer.id),
            OTHER_LEFT,
        );
    }
    let _ = writer.await;
    app.cleanup(&peer, room.as_ref().map(|(id, _)| id.as_str()));
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("SIGTERM handler");
        tokio::select! { _ = tokio::signal::ctrl_c() => {}, _ = terminate.recv() => {} }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
#[tokio::main]
async fn main() {
    let config = match Config::parse() {
        Ok(Some(config)) => config,
        Ok(None) => return,
        Err(error) => {
            eprintln!("uqm-relay: {error}");
            std::process::exit(2);
        }
    };
    let listener = match TcpListener::bind((config.host.as_str(), config.port)).await {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("uqm-relay: {error}");
            std::process::exit(1);
        }
    };
    let address = listener.local_addr().unwrap();
    if config.test_events {
        println!(
            "{}",
            json!({"event":"listening", "address":address.ip().to_string(), "port":address.port()})
        );
    } else {
        println!("UQM game and private Melee relay (Rust): http://{address}/");
    }
    let permits = Arc::new(Semaphore::new(config.max_rooms * 2 + 64));
    let app = Arc::new(App {
        config,
        state: Mutex::new(Registry::default()),
    });
    let housekeeping_app = app.clone();
    let housekeeping = tokio::spawn(async move {
        let period = housekeeping_app
            .config
            .waiting
            .min(housekeeping_app.config.lifetime)
            .min(housekeeping_app.config.heartbeat)
            .min(housekeeping_app.config.idle)
            .min(Duration::from_secs(1));
        let mut clock = tokio::time::interval_at(Instant::now() + period, period);
        let mut heartbeat = Instant::now();
        loop {
            clock.tick().await;
            let due = heartbeat.elapsed() >= housekeeping_app.config.heartbeat;
            if due {
                heartbeat = Instant::now();
            }
            housekeeping_app.tick(due);
        }
    });
    let mut tasks = tokio::task::JoinSet::new();
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);
    let mut accept_after = Instant::now();
    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            Some(_) = tasks.join_next(), if !tasks.is_empty() => {},
            accepted = async { sleep_until(accept_after).await; listener.accept().await } => {
                let (stream, address) = match accepted {
                    Ok(connection) => connection,
                    Err(_) => {
                        // Descriptor exhaustion can leave the listener readable.
                        // Back off while still polling shutdown/task completion.
                        accept_after = Instant::now() + Duration::from_millis(100);
                        continue;
                    }
                };
                let Ok(permit) = permits.clone().try_acquire_owned() else { drop(stream); continue; };
                let app = app.clone();
                tasks.spawn(async move { let _permit = permit; connection(stream, address, app).await; });
            }
        }
    }
    drop(listener);
    housekeeping.abort();
    app.stop();
    let _ = timeout(Duration::from_secs(2), async {
        while tasks.join_next().await.is_some() {}
    })
    .await;
    tasks.abort_all();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stopped_or_closed_connections_cannot_consume_room_ids() {
        for stopping in [false, true] {
            let app = App {
                config: Config::default(),
                state: Mutex::new(Registry {
                    stopping,
                    ..Registry::default()
                }),
            };
            let (tx, _) = mpsc::channel(1);
            let (controls, _) = mpsc::channel(1);
            let (closing, _) = watch::channel(None);
            let peer = Arc::new(Peer {
                id: 1,
                ip: "127.0.0.1".parse().unwrap(),
                tx,
                controls,
                closing,
                closed: AtomicBool::new(!stopping),
                alive: AtomicBool::new(true),
                bytes: AtomicUsize::new(0),
                frames: AtomicUsize::new(0),
                peak_bytes: AtomicUsize::new(0),
            });
            assert!(app
                .attach(&peer, "/netplay/0123456789abcdef0123456789abcdef?role=host")
                .is_none());
            let state = app.state.lock().unwrap();
            assert!(state.rooms.is_empty());
            assert!(state.used.is_empty());
        }
    }
    #[test]
    fn strict_paths_and_origins() {
        assert_eq!(
            room_path("/netplay/0123456789abcdef0123456789abcdef?role=host"),
            Some(("0123456789abcdef0123456789abcdef", true))
        );
        assert_eq!(
            room_path("/netplay/hd/0123456789abcdef0123456789abcdef?role=host"),
            Some(("hd/0123456789abcdef0123456789abcdef", true))
        );
        assert_eq!(
            room_path("/netplay/hd/0123456789abcdef0123456789abcdef?role=guest"),
            Some(("hd/0123456789abcdef0123456789abcdef", false))
        );
        for role in ["host", "guest"] {
            assert_eq!(
                room_path(&format!(
                    "/netplay/cosmoclash/0123456789abcdef0123456789abcdef?role={role}"
                )),
                Some((
                    "cosmoclash/0123456789abcdef0123456789abcdef",
                    role == "host"
                ))
            );
        }
        for bad in [
            "/netplay/ABCDEF0123456789abcdef0123456789?role=host",
            "/netplay/0123456789abcdef0123456789abcdef?role=host&x=1",
            "/netplay/0123456789abcdef0123456789abcdef?role=guest/",
            "/netplay/hd/ABCDEF0123456789abcdef0123456789?role=host",
            "/netplay/hd/0123456789abcdef0123456789abcdef?role=host&x=1",
            "/netplay/hd/0123456789abcdef0123456789abcdef?role=guest/",
            "/netplay/hd/0123456789abcdef0123456789abcdef",
            "/netplay/hd/hd/0123456789abcdef0123456789abcdef?role=host",
            "/netplay/original/0123456789abcdef0123456789abcdef?role=host",
            "/netplay/3do/0123456789abcdef0123456789abcdef?role=host",
            "/netplay/HD/0123456789abcdef0123456789abcdef?role=host",
            "/netplay/%68d/0123456789abcdef0123456789abcdef?role=host",
            "/netplay/hd%2f0123456789abcdef0123456789abcdef?role=host",
        ] {
            assert!(room_path(bad).is_none());
        }
        assert!(origin_is_exact("https://game.example"));
        for bad in [
            "https://game.example/",
            "null",
            "https://game.example:443",
            "https://user:pw@game.example",
            "file:///tmp",
            "HTTPS://game.example",
        ] {
            assert!(!origin_is_exact(bad));
        }
        assert_eq!(decode_path("/%2e%2e/x"), Some("/../x".into()));
        assert!(decode_path("/%zz").is_none());
        assert!(decode_path("/%ff").is_none());
    }
    #[test]
    fn config_bounds() {
        for value in [
            json!({"maxRooms":0}),
            json!({"maxPayload":65537}),
            json!({"heartbeatMs":0}),
            json!({"latencyPatternMs":[]}),
            json!({"latencyPatternMs":[2001]}),
            json!({"maxRooms":1.1}),
            json!({"unknown":1}),
        ] {
            assert!(Config::default().test_options(value).is_err());
        }
        let mut config = Config::default();
        config
            .test_options(json!({"maxRooms":1,"maxRoomIds":1,"latencyPatternMs":[40,0,20]}))
            .unwrap();
        assert_eq!(config.max_rooms, 1);
    }
    #[tokio::test]
    async fn frame_validation_precedes_allocation() {
        for bytes in [
            vec![0xc2, 0x80],
            vec![0x82, 0],
            vec![0x83, 0x80],
            vec![0x09, 0x80],
            vec![0x89, 0xfe, 0, 126],
            vec![0x82, 0xfe, 0, 1],
            vec![0x82, 0xff, 128, 0, 0, 0, 0, 0, 0, 0],
        ] {
            assert!(matches!(
                read_frame(&mut bytes.as_slice(), 65536).await,
                Err(FrameError::Protocol)
            ));
        }
        let oversized = [0x82, 0xff, 0, 0, 0, 0, 0, 1, 0, 1];
        assert!(matches!(
            read_frame(&mut oversized.as_slice(), 65536).await,
            Err(FrameError::TooLarge)
        ));
    }
}
