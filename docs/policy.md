# Policy

## Local Policy
- Sovereign: Always checked first
- Blocks/allows by skill ID or publisher
- Sets min PoW difficulty
- Stored persistently in sled

## Central Policy
- Additive: Checked if local allows
- Requires approval for install/execute
- Bans publishers
- Optional, stored separately

## Evaluation
- Install: Local then central
- Execute: Local then central
- Deny if any fails
- Decisions logged with reasons