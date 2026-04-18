# theskillbay

A Git-native self-evolving skill bay for agents: decentralised skill discovery, collaboration, and sync with signed provenance, PoW-gated publishing, policy-controlled execution, and compartmentalised trust.

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/harbour --help
```

Commands:
- init: Initialize workspace
- create-skill: Create a new skill
- publish: Publish a skill with Git commit, sign, PoW
- patch: Create a patch for collaborative evolution
- discover: Find skills
- verify: Check signature and PoW
- install: Clone and install skill if policy allows
- execute: Run skill if policy allows
- show-policy-decision: Check execution policy
- set-policy: Configure local policy
- set-central-policy: Configure central policy
- pin: Pin a skill for availability
- web: Start web UI for skill discovery

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for details.

## Vision

See [VISION.md](VISION.md) for the long-term vision.
