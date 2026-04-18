# Data Model

## Skill
- id: UUID
- name, description
- git_repo_path
- manifest: SkillManifest

## SkillManifest
- version, author
- capabilities: Vec<String>
- dependencies: Vec<String>
- entry_point: String
- tests: Vec<String>

## CapabilitySignature
- skill_id, content_hash
- signature, public_key

## PublishRecord
- skill_id
- announcement: SignedAnnouncement
- pow: ProofOfWorkRecord

## ProofOfWorkRecord
- content_hash, nonce, difficulty

## SignedAnnouncement
- skill_id, metadata: HashMap
- signature, public_key

## LocalPolicy
- allowed_publishers, blocked_skills
- min_pow_difficulty, require_central_approval

## InstallDecision
- skill_id, allowed, reason

## ExecutionDecision
- skill_id, allowed, reason

## PinRecord
- skill_id, content_hash, pinner

## ReviewRecord
- skill_id, rating (1-5), comment, timestamp

## PatchRecord
- skill_id, patch_hash, author, description, pow

## SubmittedPatch
- skill_id, description, diff, timestamp

## P2PMessage
- Enum: Announcement(SignedAnnouncement) | Review(ReviewRecord)

## CredibilityRecord
- reviewer_id, score (0-1), total_reviews

## Hashing
- Manifest + code hashed with SHA-256

## Signing
- Content hash signed with Ed25519

## Network Visibility
- Announcements, pins, reputation summaries

## Announcement Metadata
- name: Skill name
- description: Description
- author: Author
- git_url: Git repository URL for cloning

## Storage
- Keys: Stored in sled "keys" tree
- Policies: Local and central in "policies" tree
- Announcements: In "announcements" tree
- PublishRecords: In "publish_records" tree