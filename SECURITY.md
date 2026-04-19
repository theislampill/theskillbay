# Security

## Trust Model
Compartmentalized: Discovery != Trust != Execution.

## Verification
- Content hashes for integrity
- Ed25519 signatures for provenance
- PoW for anti-spam
- Policy checks before execution

## Local Sovereignty
Local policy is absolute; central is additive.

## Current Execution Model

**V0 Limitation**: Skills execute as subprocesses with no isolation. This is suitable for development/testing but NOT for production use with untrusted skills.

- Commands run with the same privileges as the harbour process
- No resource limits or sandboxing
- Central policy checks for execution are deferred (publisher identity not resolved at execution time)

**Future**: Sandboxed execution with:
- Process isolation (Firecracker, gVisor, or similar)
- Resource limits (CPU, memory, disk)
- Network restrictions
- File system isolation
- Full policy enforcement including central publisher checks