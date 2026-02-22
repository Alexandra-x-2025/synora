# CLI Smoke Tests

## Purpose
Quick regression checklist for the current CLI baseline.

## Environment
- OS: Windows PowerShell
- Repo root: `C:\dev\synora`

## 1. Build
```powershell
cargo check
cargo test
```

## 2. Init + Discovery
```powershell
cargo run -- config init --json
cargo run -- software discover scan --json
cargo run -- software list --json --limit 10
```

## 3. Source Lifecycle
```powershell
cargo run -- source suggest --json --limit 20
cargo run -- source list --json --status pending --limit 10
cargo run -- source review-bulk --approve --status pending --domain github.com --limit 3 --json
cargo run -- source apply-approved --json --limit 5
cargo run -- source registry-list --json --status active --limit 10
```

## 4. Registry Ops
```powershell
cargo run -- source registry-disable --json --domain github.com --limit 2
cargo run -- source registry-list --json --status disabled --limit 10
cargo run -- source registry-enable --json --domain github.com --limit 2
cargo run -- source registry-list --json --status active --limit 10
```

## 5. Update Flow
```powershell
cargo run -- update check --json --limit 10
cargo run -- update apply --candidate-id 180 --dry-run --json
cargo run -- update apply --candidate-id 180 --confirm --execution-ticket "ticket-2026-02-22-001" --json
cargo run -- update history --json --candidate-id 180 --limit 10
```

## 6. Failure Simulation
```powershell
cargo run -- update apply --candidate-id 180 --confirm --execution-ticket "ticket-2026-02-22-002" --simulate-failure --json
cargo run -- update apply --candidate-id 180 --confirm --execution-ticket "ticket-2026-02-22-003" --simulate-failure --simulate-rollback-failure --json
cargo run -- update history --json --status failed --candidate-id 180 --limit 10
```

## 6.1 Cleanup Flow
```powershell
cargo run -- cleanup apply --software-id 1 --dry-run --json
cargo run -- cleanup apply --software-id 1 --confirm --execution-ticket "cleanup-ticket-2026-02-22-001" --json
cargo run -- cleanup history --json --software-id 1 --limit 10
cargo run -- cleanup history --json --execution-ticket "cleanup-ticket-2026-02-22-001" --limit 10
cargo run -- cleanup history --json --rollback-status failed --limit 10
cargo run -- cleanup history --json --contains "rollback" --limit 10
cargo run -- cleanup apply --software-id 1 --confirm --json
```

## 7. Validation Errors
```powershell
cargo run -- source list --json --status unknown
cargo run -- source registry-disable --json
cargo run -- update check --json --limit 0
cargo run -- update apply --candidate-id 180 --dry-run --confirm --json
```

## 8. Download Flow
```powershell
cargo run -- repo sync --json
cargo run -- package search --json --limit 5
cargo run -- download start --package-id personal_local.sample --dry-run --json
cargo run -- download show --job-id "<job_id>" --json
cargo run -- download verify --job-id "<job_id>" --json
cargo run -- download start --package-id public_default.sample --dry-run --json
cargo run -- download verify --job-id "<job_id_2>" --simulate-hash-failure --json
cargo run -- download retry --job-id "<job_id_2>" --dry-run --json
cargo run -- download list --json --limit 10
cargo run -- download history --json --status failed --failure-type hash --limit 10
```

## 9. AI Repair Plan (Plan-only)
```powershell
cargo run -- ai repair-plan --software "PowerToys" --issue "crash on launch after update" --json
cargo run -- ai repair-plan --software "UnknownTool" --issue "startup failed with error code 5" --json

# validation errors
cargo run -- ai repair-plan --software "" --issue "x" --json
cargo run -- ai repair-plan --software "PowerToys" --issue "" --json
```

## 10. AI Analyze/Recommend (Plan-only)
```powershell
cargo run -- ai analyze --json
cargo run -- ai recommend --goal "Rust development workstation" --json
cargo run -- ai recommend --goal "video editing workflow" --json

# validation errors
cargo run -- ai recommend --goal "" --json
```

## 11. UI Search (Read-only)
```powershell
cargo run -- ui search --q "PowerToys" --json
cargo run -- ui search --q "rollback" --limit 5 --json

# validation errors
cargo run -- ui search --q "" --json
cargo run -- ui search --q "x" --limit 0 --json
```

## 12. UI Action Run (Simulated)
```powershell
# low/medium risk action can run without confirm
cargo run -- ui action-run --id "software.show:111" --json

# high risk action requires confirm
cargo run -- ui action-run --id "update.history:update-1771779929120392100-0-180" --json
cargo run -- ui action-run --id "update.history:update-1771779929120392100-0-180" --confirm --json

# validation errors
cargo run -- ui action-run --id "" --json
```

## 13. Job Queue (Simulated)
```powershell
cargo run -- job submit --type "download.fetch" --payload '{"package_id":"public_default.sample"}' --json
cargo run -- job submit --type "download.verify" --payload '{"job_id":"download-demo"}' --simulate-failed --json
cargo run -- job list --json --limit 10
cargo run -- job list --json --status failed --limit 10

# retry from failed/deadletter
cargo run -- job retry --id 2 --json

# deadletter ops
cargo run -- job submit --type "download.verify" --payload '{"job_id":"download-dlq"}' --simulate-deadletter --json
cargo run -- job deadletter-list --json --limit 10
cargo run -- job replay-deadletter --limit 5 --json

# validation errors
cargo run -- job submit --type "unknown.type" --payload "{}" --json
cargo run -- job list --json --status unknown
cargo run -- job retry --id 1 --json
```
