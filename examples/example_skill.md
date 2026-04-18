# Example Skill

This is an example skill repository.

## manifest.json
```json
{
  "version": "0.1.0",
  "author": "example",
  "capabilities": ["code_generation"],
  "dependencies": [],
  "entry_point": "main.rs",
  "tests": ["test.rs"]
}
```

## main.rs
```rust
fn main() {
    println!("Hello from skill!");
}
```