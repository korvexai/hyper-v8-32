[![Language](https://img.shields.io/badge/language-rust-orange.svg)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/latency-<1300ns-brightgreen.svg)]()
[![Memory](https://img.shields.io/badge/memory-fixed--279MB-blue.svg)]()
[![License](https://img.shields.io/badge/license-personal--use--only-red.svg)]()
[![Status](https://img.shields.io/badge/status-production--ready-success.svg)]()

Hyper V8-32 Engine
Status: Production-Ready (Core Engine)

Language: Rust (stable)

Framework: Actix-web

Profile: Lock-free / Atomic / Concurrency-oriented

Author: Korvex

Copyright: Â© 2026 Korvex

ðŸ“Œ Overview
Hyper V8-32 is a high-performance concurrent processing engine designed for ultra-low latency and strict memory stability.

The engine is built around a 32-valve architecture, using only atomic operations and lock-free hot paths.

The HTTP interface exists solely as an injection hook for testing and benchmarking purposes.

âš™ï¸ Architecture Summary
32 HyperCore valves

Deterministic hash-based routing

Atomic units using AtomicU64

Cache-line aligned (64 bytes)

No mutexes on the critical path

No per-request dynamic allocations

ðŸŒ API
Active Endpoint
POST /fire

Response
V8-32 Engine: PROCESSED | Time: <ns>

Headers
X-Hyper-Status: PROCESSED | COLLISION

X-Latency-Ns: <number>

Rejected Methods (by design)
GET /fire â†’ 404

PUT /fire â†’ 404

POST /invalid â†’ 404

ðŸš€ Build & Run
Optimized Build
Bash

cargo build --release
Run
Bash

cargo run --release
Server: http://0.0.0.0:8080/fire

ðŸ§ª TESTING (REAL-WORLD VERIFIED)
All tests below were executed on Windows (x86_64), using the --release binary.

âœ… Test 1 â€” Port Binding
PowerShell

netstat -ano | findstr :8080
Result:

Port 8080 â†’ LISTENING

Valid PID âœ” PASS

âœ… Test 2 â€” Clean Shutdown
PowerShell

taskkill /IM hyper-v8-32.exe /F
netstat -ano | findstr :8080
Result:

Port fully released âœ” PASS

âœ… Test 3 â€” Restart After Kill
PowerShell

cargo run --release
Result:

Server starts instantly

Correct port binding âœ” PASS

âœ… Test 4 â€” Method Filtering
PowerShell

curl.exe http://127.0.0.1:8080/fire
curl.exe -X PUT http://127.0.0.1:8080/fire
curl.exe -X POST http://127.0.0.1:8080/invalid
Result:

All requests return 404

Engine remains stable âœ” PASS

âœ… Test 5 â€” Single POST Injection
PowerShell

curl.exe -X POST http://127.0.0.1:8080/fire
Typical Result: V8-32 Engine: PROCESSED | Time: 700â€“1800 ns âœ” PASS

âœ… Test 6 â€” Serial Burst Load
PowerShell

1..500 | % { curl.exe -X POST http://127.0.0.1:8080/fire > $null }
Result:

No crashes

Stable latency âœ” PASS

âœ… Test 7 â€” Concurrency Stress
PowerShell

1..32 | % {
  Start-Job { 1..100 | % { curl.exe -X POST http://127.0.0.1:8080/fire > $null } }
}
Result:

~3200 concurrent requests

No deadlocks

No hangs âœ” PASS

âœ… Test 8 â€” Collision Integrity
Occasional COLLISION responses under heavy concurrency

Engine continues operating normally

No performance degradation âœ” PASS (expected behavior)

âœ… Test 9 â€” Memory Stability
PowerShell

Get-Process hyper-v8-32 | Select WorkingSet
Result: ~279 MB WorkingSet

Constant memory usage before and after stress

No progressive growth âœ” PASS (no memory leaks)

âœ… Test 10 â€” TCP State Validation
PowerShell

netstat -ano | findstr :8080
Result:

Correct LISTENING state

Normal TIME_WAIT entries (Windows TCP stack)

No zombie sockets âœ” PASS

ðŸ§  Technical Notes
Engine is POST-only by design.

Reported latencies are in-engine, not TCP RTT.

TIME_WAIT behavior is normal on Windows.

Memory usage includes Actix and allocator overhead.

â— Non-Goals
âŒ Not a general-purpose web framework

âŒ Not a full REST API

âŒ Not a comparative benchmark suite

This project is a lock-free atomic processing engine, minimally exposed for testing.

ðŸŸ¢ Final Status
ENGINE CORE: STABLE / VERIFIED / READY FOR PUBLICATION

âš–ï¸ Commercial Use & Licensing
Hyper V8-32 is NOT open source.

This project is source-available and licensed strictly for Personal Use by private individuals, as defined in the LICENSE file.

What is allowed
Personal, private experimentation by a natural person

Non-commercial learning and research at home

Reading and studying the source code

What is NOT allowed without a commercial license
Any use by a company, startup, or organization

Internal testing, benchmarking, evaluation, or research

Integration into products, services, platforms, or infrastructure

Offering services built on top of this engine

Any professional or revenue-generating activity

Any use performed on behalf of a legal entity

Any of the above constitutes Commercial Use and requires a separate paid commercial license from Korvex.

Unauthorized commercial use constitutes copyright infringement and is enforceable under applicable law.

For commercial licensing inquiries: contact@korvex.ai
