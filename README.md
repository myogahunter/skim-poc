# skim-poc

Replica of skim-rs/skim's PR review workflow.

Demonstrates build.rs code execution on pull_request_target:
- `generate-files` job checks out fork PR code
- Runs `cargo run` which compiles and executes attacker code
- GITHUB_TOKEN (contents:write) and app token in scope

Used for authorized security research only.
