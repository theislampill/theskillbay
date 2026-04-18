# Protocol

## Publish Flow
1. Create skill manifest
2. Validate manifest
3. Commit changes to Git
4. Hash manifest + code
5. Sign hash with stored key
6. Generate PoW nonce
7. Create signed announcement with metadata (name, author, git_url)
8. Store PublishRecord and advertise

## Verify Flow
1. Discover announcement
2. Check signature on metadata
3. Check PoW from PublishRecord
4. Validate manifest hash

## Install Flow
1. Discover skill
2. Verify
3. Evaluate local + central policies
4. If allowed, clone Git repo

## Execute Flow
1. Check local + central execution policy
2. If allowed, run entry_point in subprocess

## Patch Flow
1. Commit changes to skill repo
2. Hash patch description
3. Generate PoW nonce
4. Create PatchRecord
5. Advertise patch for review/merge

## Execute Flow
1. Check execution policy
2. Run in sandbox if allowed