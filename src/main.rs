// © 2026 Korvex. Personal-use only. Commercial use prohibited.
// Project: Hyper V8-32 | Profile: Production-Ready-Final
// Status: Zero-Delirium | Optimized Atomic ID | Forensic IP Active

mod security;
mod tracking;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::fs;
use sha2::{Sha256, Digest};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

use crate::security::*;
use crate::tracking::TRACKING_HASH;

// Global atomic counter pentru generare ID-uri unice cu overhead minim
static REQ_COUNTER: AtomicU64 = AtomicU64::new(1);

// ================================================================
// 0. HARD-LOCK SECURITY CHECK (Pre-Startup)
// ================================================================
fn verify_security() -> bool {
    let hwid = machine_uid::get().unwrap_or_else(|_| "UNKNOWN_ID".to_string());
    let tier = "GOLD"; 
    let secret_salt = "KORVEX_MASTER_SECRET_KEY_2026";
    let license_path = "license.key";

    if !std::path::Path::new(license_path).exists() {
        println!("\n❌ EROARE CRITICĂ: Licența lipsește!");
        println!("HWID-ul dumneavoastră: {}", hwid);
        return false;
    }

    let stored_key = fs::read_to_string(license_path).unwrap_or_default().trim().to_string();
    let mut hasher = Sha256::new();
    hasher.update(format!("{}-{}-{}", hwid, tier, secret_salt));
    let expected_key = format!("{:x}", hasher.finalize());

    // --- LOGICA DE DEBUG ADAUGATĂ ---
    if stored_key != expected_key {
        println!("\n❌ EROARE: Licență invalidă!");
        return false;
    }

    true
}

// ================================================================
// 1. QUANTUM CELL – ATOMIC UNIT ALIGNED TO CACHE LINE
// ================================================================
#[repr(align(64))]
struct QuantumCell {
    state: AtomicU64,
}

impl QuantumCell {
    const fn new() -> Self {
        Self { state: AtomicU64::new(0) }
    }
}

// ================================================================
// 2. HYPERCORE – REPRESENTS A PROCESSING VALVE
// ================================================================
struct HyperCore {
    grid: Vec<QuantumCell>,
    mask: usize, 
}

impl HyperCore {
    fn new(cap_limit: usize) -> Self {
        let cap = cap_limit.min(131_072).next_power_of_two();
        let grid = (0..cap).map(|_| QuantumCell::new()).collect();
        Self {
            grid,
            mask: cap - 1,
        }
    }

    #[inline(always)]
    fn try_process(&self, cell_idx: usize, payload: u64) -> bool {
        let idx = cell_idx & self.mask;
        let cell = &self.grid[idx];

        if cell.state.load(Ordering::Acquire) != 0 {
            return false;
        }

        if cell.state.compare_exchange(0, (payload << 1) | 1, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            let _work = payload.wrapping_add(0x42);
            cell.state.store(0, Ordering::Release);
            true
        } else {
            false
        }
    }
}

// ================================================================
// 3. SUPREME ENGINE – 8 PISTONS / 32 VALVES ARCHITECTURE
// ================================================================
struct SupremeEngine {
    valves: Vec<HyperCore>,
}

impl SupremeEngine {
    fn new(cells_per_valve: usize) -> Self {
        let mut valves = Vec::with_capacity(32);
        for _ in 0..32 {
            valves.push(HyperCore::new(cells_per_valve));
        }
        Self { valves }
    }

    #[inline(always)]
    fn inject(&self, request_id: u64) -> bool {
        let core_info = crate::security::SecurityContext {
            identity_key: TRACKING_HASH,
            license_key: LicenseKey { valid: true }, 
            abuse_key: AbuseKey { level: 0 },
        };

        let wm = watermark(request_id, core_info.identity_key);
        let lb = license_bias(core_info.license_key.valid); 
        let ab = abuse_bias(core_info.abuse_key.level);

        let hash = request_id
            .wrapping_mul(0x9E3779B1 ^ wm)
            .wrapping_add(lb) ^ ab;

        let valve_idx = (hash as usize) & 31;
        let cell_idx = (hash as usize) >> 5; 

        if let Some(valve) = self.valves.get(valve_idx) {
            valve.try_process(cell_idx, request_id)
        } else {
            false
        }
    }
}

// ================================================================
// 4. PUBLIC API (PROTECTED INJECTION)
// ================================================================
async fn hook(engine: web::Data<Arc<SupremeEngine>>) -> impl Responder {
    let start = Instant::now();
    
    let counter = REQ_COUNTER.fetch_add(1, Ordering::Relaxed);
    let ts_low = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let req_id = counter ^ ts_low;

    let success = engine.inject(req_id);
    let latency = start.elapsed().as_nanos();

    let status = if success { "PROCESSED" } else { "COLLISION" };

    HttpResponse::Ok()
        .insert_header(("X-Hyper-Status", status))
        .insert_header(("X-Latency-Ns", latency.to_string()))
        .body(format!("V8-32 Engine: {} | Time: {} ns", status, latency))
}

// ================================================================
// 5. ENGINE STARTUP
// ================================================================
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 🛡️ VERIFICARE SECURITATE HARD-LOCK
    if !verify_security() {
        std::process::exit(1);
    }

    println!("🏁 HYPER V8-32 ENGINE [ULTIMATE BUILD] – Korvex IP Active");
    println!("✅ Security Verified: Commercial License Active");

    let engine = Arc::new(SupremeEngine::new(131_072));

    println!("📡 Endpoint active at: http://0.0.0.0:8080/fire");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(engine.clone()))
            .route("/fire", web::post().to(hook))
    })
    .workers(32) 
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
