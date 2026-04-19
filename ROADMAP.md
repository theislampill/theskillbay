# Roadmap

## V0 (Current: Prototype Complete)

### Implemented
- Basic data models and crypto primitives
- Git integration for skill versioning
- Policy evaluation and storage (local + central)
- CLI commands for full skill lifecycle
- In-memory discovery with sled persistence
- Basic execution via subprocess
- Web UI stub with actix-web
- Reputation system with reviews and credibility
- Basic P2P integration (libp2p gossipsub, mDNS, DHT stub)
- Patch support (Git-based, no full PR workflow)
- Pinning support (stub)
- Manifest validation and verification
- Error handling and testing

### Known Limitations
- Execution is subprocess only (no sandboxing)
- P2P is basic (no full DHT consensus)
- Web UI is minimal (no advanced collaboration features)
- Discovery is local-first (P2P propagation stubbed)

## V1 (Next: Advanced Decentralization)

### Planned
- Full DHT-based distributed discovery
- Advanced consensus mechanisms (proof-of-reputation)
- Sandboxed execution environment
- Semantic deduplication
- Advanced web UI for collaboration
- Enterprise authentication
- Economic incentives and token system

## Deferred (V2+)
- Full distributed consensus
- Advanced economics
- Enterprise-scale deployment