# Architecture

## System Purpose
Decentralized skill sharing for agents, with Git as canonical artifact, compartmentalized trust, and policy-controlled execution.

## Core Objects
- Skill: Git repo with code, manifest, tests.
- SkillManifest: Metadata and capabilities.
- CapabilitySignature: Signed hash of content.
- PublishRecord: Announcement + PoW.
- PatchRecord: Changes with PoW.
- ProofOfWorkRecord: Anti-spam nonce.
- SignedAnnouncement: Network advertisement.
- ReputationSummary: Trust scores.
- PinRecord: Availability guarantees.
- LocalPolicy: Sovereign rules.
- CentralPolicy: Additive org rules.
- InstallDecision: Policy result.
- ExecutionDecision: Gate for running.

## Trust Boundaries
- Discovery: Network layer, no trust.
- Trust: Verification of hashes/signatures.
- Execution: Policy-gated.

## Responsibilities
- Network: DHT/gossip for refs, hashes, announcements.
- Policy: Local sovereign (blocked skills, publisher allowlists, central approval requirement), central additive (approved skills, banned publishers).
- Execution: Only if local policy allows; central policy checks deferred for execution in V0.

## Lifecycle
Create -> Publish (PoW + sign) -> Discover -> Verify -> Policy eval -> Install -> Execute.

## Harbour vs theskillbay
theskillbay: Network/protocol. Harbour: Client/agent addon.

## V0 Implementation Status

### Implemented in V0
- **Data Models**: All core structs (Skill, SkillManifest, SignedAnnouncement, etc.)
- **Crypto**: SHA-256 hashing, Ed25519 signing, basic PoW
- **Git Integration**: Repo init, commit, clone operations
- **Policy**: Local and central policy evaluation, storage
- **Discovery**: In-memory storage with sled persistence, basic queries
- **Execution**: Subprocess execution (no sandboxing)
- **CLI**: Full command set for skill lifecycle
- **Storage**: sled-based persistence for policies, announcements, reviews
- **Web UI**: Basic actix-web server (stub interface)
- **P2P**: Basic libp2p integration (gossipsub, mDNS, DHT stub)
- **Reputation**: Review system with credibility scoring

### Deferred / Stubbed in V0
- **Full P2P Network**: DHT for distributed storage, consensus
- **Advanced Consensus**: Proof-of-reputation, decentralized validation
- **Sandboxing**: Strongly isolated execution environment
- **Semantic Deduplication**: ML-based similarity detection
- **Enterprise Auth**: Advanced authentication mechanisms
- **Economics**: Token incentives, advanced reputation models