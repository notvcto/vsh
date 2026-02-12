# VSH - Vic's Shell (Implementation)

This is the Rust implementation of VSH.

## Project Structure

```
vsh/
├── Cargo.toml              # Project dependencies and metadata
├── src/
│   ├── main.rs             # Entry point + REPL
│   ├── lib.rs              # Library root
│   ├── parser/             # Command parsing
│   │   ├── mod.rs          # Parser public interface
│   │   ├── tokenizer.rs    # Tokenization logic
│   │   ├── syntax.rs       # Syntax detection
│   │   └── translator.rs   # Argument extraction
│   ├── executor/           # Command execution
│   │   ├── mod.rs          # Executor public interface
│   │   └── builtin.rs      # Built-in commands
│   ├── error.rs            # Error types
│   └── config.rs           # Configuration handling
├── tests/
│   └── integration_test.rs # Integration tests
├── examples/
│   └── backup.vsh          # Example script
└── .github/
    └── workflows/
        └── ci.yml          # CI/CD pipeline
```

## Building

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run with debug output
cargo run -- --debug

# Execute a single command
cargo run -- -c "copy file.txt to backup/"
```

## Running

```bash
# Start REPL
cargo run

# Then try commands:
vsh$ copy file.txt to backup/
vsh$ cp file.txt backup/
vsh$ copy source=file.txt destination=backup/
vsh$ list
vsh$ help
vsh$ exit
```

## Implemented Features (MVP)

✅ **Parser**

- Tokenization with quote handling
- Syntax detection (terse, verbose, named)
- Intent identification
- Argument extraction

✅ **Commands**

- `copy` / `cp` - Copy files (all three syntaxes)
- `move` / `mv` - Move files
- `remove` / `rm` / `delete` - Remove files (with confirmation)
- `list` / `ls` - List directory contents
- `cd` / `change-directory` - Change directory

✅ **Features**

- REPL with history
- Colored output
- Error messages with suggestions
- Config file support (.vshrc)
- Help system

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_copy_command_terse

# Run with output
cargo test -- --nocapture
```

## Next Steps

See the main [VSH_SPECIFICATION.md](../VSH_SPECIFICATION.md) for the full roadmap.

**Phase 2: Intelligence Layer**

- User profiling
- Adaptive auto-completion
- Smart error suggestions

**Phase 3: Scripting**

- Variables and control flow
- Functions
- .vsh script execution

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute!
