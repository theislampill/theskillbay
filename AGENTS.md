# Agents Guide

## Project Purpose
Build a decentralized skill sharing system for agents, with Git-native skills, signed provenance, PoW publishing, and policy execution.

## Current Architecture Summary
See ARCHITECTURE.md. Trust boundaries: Discovery, Trust, Execution. Git canonical.

## V0 Allowed
- CLI prototype
- In-memory stubs
- Basic crypto/policy
- Local discovery simulation

## V0 Must Not Do
- Full P2P network
- Web UI
- Advanced reputation
- Semantic dedupe
- Consensus

## Coding Conventions
- Rust with async
- JSON for data
- SHA-256, Ed25519
- Clap for CLI

## Test Requirements
- Unit tests for crypto, policy
- Integration tests for flows

## How to Propose Change
- Update canonical docs first
- Add tests
- Small patches

## Module Choice
- Extend existing modules
- New module only if clear separation

## Anti-Drift Rules
- Inspect ARCHITECTURE.md before changes
- Do not collapse layers
- Keep Git canonical
- Local policy sovereign
- Central additive
- No hidden agent memory
- Prefer inspectable mechanisms

## Canonical Files
- ARCHITECTURE.md for architecture
- ROADMAP.md for scope
- SECURITY.md for trust

## Do Not Re-Argue
- Compartmentalized trust
- Git as artifact layer
- PoW as gate, not trust
- Small primitives over frameworks

## Next Tasks
See TASKS.md