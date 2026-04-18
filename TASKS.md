# Tasks

## Completed Tasks

### Task 1: Implement Full Publish Flow
- Status: Done
- Extended publish to commit Git changes, generate announcement with metadata, persist PoW, use stored keys.

### Task 2: Add Local Storage
- Status: Done
- Added sled-based storage for keys and policies.

### Task 3: Improve Discovery
- Status: Done
- Persist announcements in sled db.

### Task 4: Manifest Validation
- Status: Done
- Added validate method to SkillManifest.

### Task 5: Execution Sandboxing
- Status: Done
- Added basic subprocess execution.

### Task 6: Add Git Clone for Install
- Status: Done
- Install now clones Git repo from announcement git_url.

### Task 7: Add Verify Command Implementation
- Status: Done
- Verify checks signature and PoW from stored records.

### Task 8: Add Central Policy Support
- Status: Done
- Added central policy storage and evaluation in install/execute.

### Task 9: Add Tests for New Features
- Status: Done
- Added tests for manifest validation and storage.

### Task 10: Update Docs
- Status: Done
- Updated protocol, policy, data-model docs and README.

## Next Tasks

### Task 11: Add Reputation System Stub
- Title: Stub reputation summaries in announcements
- Purpose: Prepare for future reputation
- Files: src/models.rs, src/main.rs
- Prerequisites: Announcements working
- Definition of Done: Reputation fields added, default values

### Task 12: Add Pinning Support
- Title: Implement pin records for content availability
- Purpose: Ensure skills remain available
- Files: src/models.rs, src/discovery.rs
- Prerequisites: Discovery working
- Definition of Done: Pin command to mark skills as pinned

### Task 13: Improve Error Handling
- Title: Add better error messages and logging
- Purpose: User-friendly feedback
- Files: src/main.rs, src/cli.rs
- Prerequisites: Basic commands working
- Definition of Done: Errors logged with context

### Task 14: Add Benchmarking Stub
- Title: Stub benchmark fields in manifest
- Purpose: Prepare for performance testing
- Files: src/models.rs
- Prerequisites: Manifest working
- Definition of Done: Benchmarks field added

### Task 15: Update Examples
- Title: Create example with full flow
- Purpose: Demonstrate usage
- Files: examples/
- Prerequisites: All commands working
- Definition of Done: Example script or README

### Task 16: Add Web UI Stub
- Title: Basic HTML interface for skill listing
- Purpose: Enable web access
- Files: src/web.rs
- Prerequisites: Discovery working
- Definition of Done: Serve HTML page with skills

### Task 17: Add P2P Discovery Stub
- Title: Stub libp2p for future network
- Purpose: Prepare for decentralized discovery
- Files: src/p2p.rs
- Prerequisites: Discovery working
- Definition of Done: Basic libp2p setup

### Task 18: Implement Basic Reputation System
- Title: Add reviews and ratings to skills
- Purpose: Build trust through user feedback
- Files: src/models.rs, src/cli.rs, src/discovery.rs
- Prerequisites: Discovery working
- Definition of Done: Review command and storage

### Task 19: Add Patch Support
- Title: Allow skill updates via patches
- Purpose: Enable collaborative improvements
- Files: src/models.rs, src/git.rs
- Prerequisites: Git integration working
- Definition of Done: Patch command to apply changes

### Task 20: Performance Benchmarking
- Status: Done
- Added Criterion benchmarks for crypto (SHA256, sign, verify, PoW) and storage (save/load policy, advertise) in benches/.

## V1 Tasks

### Task 21: Implement Basic Reputation System
- Status: Done
- Added review command, ReviewRecord, and reputation calculation in announcements.

### Task 22: Add Collaborative Editing UI
- Title: Enhance web UI for patch submission and reviews
- Purpose: Enable collaborative editing via web
- Files: src/web.rs
- Prerequisites: Web UI working
- Definition of Done: Forms for patches and reviews in web UI

### Task 23: Implement Semantic Dedupe Hooks
- Status: Done
- Added src/dedupe.rs with Jaccard similarity on names/descriptions, and find_similar method in discovery.

### Task 24: Implement Basic P2P Discovery
- Status: Done
- Added src/p2p.rs with gossipsub for announcements and reviews, integrated with DiscoveryStore via channel.

### Task 25: Integrate P2P with Storage and Trust Propagation
- Status: Done
- Added DHT (kad) to P2P for scalable discovery, credibility scoring for reviewers based on review count, integrated with storage and P2P broadcasting.