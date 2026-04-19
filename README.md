# theskillbay

A Git-native self-evolving skill bay for agents: decentralised skill discovery, collaboration, and sync with signed provenance, PoW-gated publishing, policy-controlled execution, and compartmentalised trust.

## V0 Scope

This is a V0 prototype implementing:
- CLI for skill creation, publishing, discovery, and execution
- Git integration for skill versioning
- Cryptographic signing and PoW for anti-spam
- Local and central policy evaluation (central checks deferred for execution)
- Basic in-memory discovery (P2P stubbed)
- Simple subprocess execution (no sandboxing)
- Web UI stub (basic server, no advanced features)
- Reputation system with reviews and credibility scoring

**Not implemented in V0:**
- Full P2P network with DHT
- Advanced consensus mechanisms
- Sandboxed execution
- Semantic deduplication
- Enterprise authentication
- Advanced economics/incentives

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/harbour --help
```

### Core Commands

- `init <path>`: Initialize workspace
- `create-skill <name> <description> <path>`: Create a new skill
- `publish <path>`: Publish a skill with Git commit, sign, PoW
- `discover <query>`: Find skills (in-memory)
- `verify <skill_id>`: Check signature and PoW
- `install <skill_id> <path>`: Clone and install skill if policy allows
- `execute <skill_id> [args]`: Run skill if policy allows
- `show-policy-decision <skill_id>`: Check execution policy
- `set-policy <blocked> <min_pow>`: Configure local policy
- `set-central-policy <banned>`: Configure central policy

### Advanced Commands (Partial Implementation)

- `patch <path> <description>`: Create a patch (Git-based, no PR)
- `pin <skill_id>`: Pin a skill for availability (stub)
- `web`: Start basic web UI for skill discovery
- `review <skill_id> <rating> <comment>`: Review a skill
- `p2p`: Start P2P node (stubbed network behavior in V0)

## Verification

After building:

```bash
# Run tests
cargo test

# Create a test skill
mkdir -p test_skill
./target/release/harbour create-skill "test" "A test skill" test_skill/

# Publish it
./target/release/harbour publish test_skill/

# Discover it
./target/release/harbour discover "test"

# Verify it
./target/release/harbour verify <skill_id>

# Install it
./target/release/harbour install <skill_id> install_path/

# Execute it (requires skill to have executable entry_point)
./target/release/harbour execute <skill_id> arg1 arg2
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for details.

## Security

See [SECURITY.md](SECURITY.md) for trust model and current limitations.
