# Protocol

## Publish Flow
1. Create skill manifest
2. Hash content
3. Sign hash
4. Generate PoW nonce
5. Create signed announcement
6. Advertise on discovery

## Verify Flow
1. Check signature
2. Check PoW
3. Validate manifest

## Install Flow
1. Discover skill
2. Verify
3. Evaluate policies
4. Clone Git repo if approved

## Execute Flow
1. Check execution policy
2. Run in sandbox if allowed