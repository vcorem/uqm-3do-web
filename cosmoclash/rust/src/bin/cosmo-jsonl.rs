//! Native test driver; fixture loading and AI oracles are intentionally absent
//! from the production WASM request surface.
use std::io::{self, BufRead, Write};
fn main() {
    let mut engine = cosmoclash_core::abi::Engine::default();
    let stdin = io::stdin();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let response = match line {
            Ok(line) => {
                let parsed: Option<serde_json::Value> = serde_json::from_str(&line).ok();
                match parsed.as_ref().and_then(|p| p["op"].as_str()) {
                    Some("ai_oracle") => {
                        match cosmoclash_core::ai::oracle::run(parsed.as_ref().unwrap()) {
                            Ok(v) => v,
                            Err(e) => serde_json::json!({"ok":false,"error":e}),
                        }
                    }
                    Some("testLoad") => match serde_json::from_value::<cosmoclash_core::World>(
                        parsed.as_ref().unwrap()["snapshot"].clone(),
                    ) {
                        Ok(world) => {
                            engine.world = Some(world);
                            engine.request(br#"{"op":"snapshot"}"#)
                        }
                        Err(e) => serde_json::json!({"ok":false,"error":e.to_string()}),
                    },
                    _ => engine.request(line.as_bytes()),
                }
            }
            Err(e) => serde_json::json!({"ok":false,"error":e.to_string()}),
        };
        serde_json::to_writer(&mut out, &response).unwrap();
        writeln!(out).unwrap();
        out.flush().unwrap();
    }
}
