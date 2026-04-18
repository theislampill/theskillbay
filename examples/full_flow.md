# Full Flow Example

This example demonstrates the complete skill lifecycle in theskillbay.

## 1. Initialize Workspace

```bash
harbour init ./workspace
cd ./workspace
```

## 2. Create a Skill

```bash
harbour create-skill "my-skill" "A test skill" ./my-skill
cd my-skill
# Edit manifest.json and main.rs as needed
```

## 3. Publish the Skill

```bash
harbour publish .
```

This commits changes, signs the content, generates PoW, and advertises the skill.

## 4. Discover Skills

```bash
harbour discover "my-skill"
```

## 5. Verify a Skill

```bash
harbour verify <skill-id>
```

Checks signature and PoW.

## 6. Set Policies (Optional)

```bash
harbour set-policy "" 4  # No blocks, min PoW 4
harbour set-central-policy ""  # No bans
```

## 7. Install a Skill

```bash
harbour install <skill-id>
```

Clones the Git repo if policies allow.

## 8. Execute a Skill

```bash
harbour execute <skill-id> arg1 arg2
```

Runs the skill in a subprocess if policies allow.

## 9. Pin a Skill

```bash
harbour pin <skill-id>
```

Marks the skill as pinned for availability.

## 10. Show Policy Decision

```bash
harbour show-policy-decision <skill-id>
```

Checks if execution is allowed.