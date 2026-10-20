//! Small JSON ABI. Inputs are copied through tracked, bounded allocations; an
//! arbitrary pointer/length from the host is never dereferenced.
use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
pub const MAX_REQUEST_BYTES: usize = 64 * 1024;
#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "camelCase", deny_unknown_fields)]
pub enum Request {
    Init {
        seed: u32,
        ships: [ShipId; 2],
        #[serde(default, rename = "aiDifficulty")]
        ai_difficulty: Option<String>,
    },
    Step {
        dt: f64,
        inputs: [InputState; 2],
    },
    // A unit variant silently ignores additional fields in serde's tagged enum.
    // The empty struct keeps the same wire shape and enforces deny_unknown_fields.
    Snapshot {},
}
#[derive(Default)]
pub struct Engine {
    pub world: Option<World>,
}
impl Engine {
    pub fn request(&mut self, input: &[u8]) -> serde_json::Value {
        let result = (|| -> Result<serde_json::Value, String> {
            if input.len() > MAX_REQUEST_BYTES {
                return Err("request exceeds 64 KiB limit".into());
            }
            let req: Request =
                serde_json::from_slice(input).map_err(|e| format!("invalid request: {e}"))?;
            match req {
                Request::Init {
                    seed,
                    ships,
                    ai_difficulty,
                } => {
                    if ai_difficulty
                        .as_deref()
                        .is_some_and(|s| !matches!(s, "easy" | "medium" | "hard"))
                    {
                        return Err("aiDifficulty must be easy, medium or hard".into());
                    }
                    self.world = Some(World::new(seed, ships, ai_difficulty));
                }
                Request::Step { dt, inputs } => {
                    self.world
                        .as_mut()
                        .ok_or("initialize the engine first")?
                        .step(dt, inputs)?;
                }
                Request::Snapshot {} => {}
            }
            let world = self.world.as_ref().ok_or("initialize the engine first")?;
            Ok(serde_json::json!({"ok":true,"snapshot":world}))
        })();
        result.unwrap_or_else(|error| serde_json::json!({"ok":false,"error":error}))
    }
}
#[derive(Default)]
struct AbiState {
    engine: Engine,
    allocations: BTreeMap<usize, Box<[u8]>>,
    result: Vec<u8>,
}
thread_local! {static STATE:RefCell<AbiState>=RefCell::new(AbiState::default());}
#[no_mangle]
pub extern "C" fn cosmo_alloc(len: usize) -> usize {
    if len == 0 || len > MAX_REQUEST_BYTES {
        return 0;
    }
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.allocations.len() >= 16 {
            return 0;
        }
        let mut bytes = vec![0; len].into_boxed_slice();
        let ptr = bytes.as_mut_ptr() as usize;
        s.allocations.insert(ptr, bytes);
        ptr
    })
}
#[no_mangle]
pub extern "C" fn cosmo_dealloc(ptr: usize, len: usize) {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.allocations.get(&ptr).is_some_and(|b| b.len() == len) {
            s.allocations.remove(&ptr);
        }
    })
}
#[no_mangle]
pub extern "C" fn cosmo_request(ptr: usize, len: usize) -> usize {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        let input = s
            .allocations
            .get(&ptr)
            .filter(|b| b.len() == len)
            .map(|b| b.to_vec());
        let response = match input {
            Some(input) => s.engine.request(&input),
            None => serde_json::json!({"ok":false,"error":"invalid input allocation"}),
        };
        s.result = serde_json::to_vec(&response).expect("response is serializable");
        s.result.as_ptr() as usize
    })
}
#[no_mangle]
pub extern "C" fn cosmo_result_len() -> usize {
    STATE.with(|s| s.borrow().result.len())
}
#[no_mangle]
pub extern "C" fn cosmo_abi_version() -> u32 {
    1
}
