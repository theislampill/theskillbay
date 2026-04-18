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
- Policy: Local sovereign, central additive.
- Execution: Only if policy allows.

## Lifecycle
Create -> Publish (PoW + sign) -> Discover -> Verify -> Policy eval -> Install -> Execute.

## Harbour vs theskillbay
theskillbay: Network/protocol. Harbour: Client/agent addon.

## V0 Scope
CLI prototype, in-memory discovery, basic PoW, policy, Git integration.

## Deferred Scope
Full DHT, web UI, advanced reputation, semantic dedupe, consensus.