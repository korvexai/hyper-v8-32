# Runbook

## Normal behavior

- Requests may return `Collision`
- Latency increases under contention
- Throughput is best-effort
- No request backlog is kept

## This is NOT a bug

- High collision rate under load
- Uneven request distribution
- Throughput drops during spikes

## Health definition

The engine is healthy if:
- The process is running
- It responds to requests
- It does not block or crash

Rejections are part of normal operation.
