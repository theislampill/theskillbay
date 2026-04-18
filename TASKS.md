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

## Next Tasks

### Task 6: Add Git Clone for Install
- Title: Replace copy with actual Git clone in install
- Purpose: Proper skill installation from Git
- Files: src/main.rs, src/git.rs
- Prerequisites: Git integration
- Definition of Done: Install clones Git repo

### Task 7: Add Verify Command Implementation
- Title: Implement verify to check sig and PoW
- Purpose: Full verification flow
- Files: src/main.rs, src/crypto.rs
- Prerequisites: Crypto functions
- Definition of Done: Verify command validates discovered skills

### Task 8: Add Central Policy Support
- Title: Load and evaluate central policy
- Purpose: Additive policy
- Files: src/policy.rs, src/storage.rs
- Prerequisites: Local policy working
- Definition of Done: Central policy checked if required

### Task 9: Add Tests for New Features
- Title: Unit tests for storage, validation, execution
- Purpose: Ensure reliability
- Files: tests/
- Prerequisites: Features implemented
- Definition of Done: Tests pass

### Task 10: Update Docs
- Title: Update docs to reflect new features
- Purpose: Keep docs current
- Files: docs/, README.md
- Prerequisites: Features done
- Definition of Done: Docs updated