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