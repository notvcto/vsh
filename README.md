# VSH - Vic's Shell

**A shell for everyone, built by everyone.**

---

## 🎯 What is VSH?

VSH is a next-generation command-line shell that maintains full power while making the terminal accessible to beginners and more productive for everyone.

### The Problem
Traditional shells are incredible tools, but they're needlessly cryptic:
- `find . -type f -name "*.txt" -exec grep -l "pattern" {} \;` ← What does this even do?
- Commands like `cp`, `mv`, `rm` aren't intuitive
- One misplaced quote breaks everything
- Error messages are hostile
- The learning curve keeps brilliant people away from the command line

### The Solution
VSH offers **cognitive flexibility** - the same command works in multiple syntaxes:

```bash
# Power user (terse)
cp file.txt backup/

# Beginner (verbose)
copy file.txt to backup/

# Explicit (named parameters)
copy source=file.txt destination=backup/

# All three work! Pick what makes sense to YOUR brain.
```

---

## ✨ Key Features

### 🧠 **Adaptive Intelligence**
VSH learns your preferences and adapts:
- Auto-completion matches your style
- Suggestions improve over time
- Errors become more helpful as it learns what confuses you

### 📝 **Multiple Syntax Styles**
Every command supports terse, verbose, and named-parameter syntax. Use what feels natural.

### 💡 **Intelligent Errors**
```bash
$ copy nonexistent.txt backup/
✗ Error: Source file does not exist

The file 'nonexistent.txt' could not be found.

Suggestion: Did you mean one of these?
  → copy document.txt to backup/
  → copy notes.txt to backup/

[?] Need help? Type 'help copy'
```

### 🔄 **Full Bash Compatibility**
Run your existing `.sh` scripts without changes. VSH automatically detects and handles them.

### 📜 **Readable Scripts**
Write `.vsh` scripts that read like pseudocode:

```vsh
#!/usr/bin/env vsh

backup_dir = "~/backups"

if not directory-exists $backup_dir then
    create-directory $backup_dir
end

for file in *.txt do
    copy $file to $backup_dir
    echo "Backed up: $file"
end
```

---

## 🚀 Status

**Current:** Specification Phase  
**Next:** Community Building & MVP Development  
**Timeline:** First release target - Q3 2026

We're building this in the open from day one. Join us!

---

## 🤝 Community-Driven

VSH isn't built by one person - it's built by everyone who uses it.

**Why?**
- A shell for all skill levels needs input from all skill levels
- Diverse perspectives = better design
- Sustainable beyond any single maintainer
- Your voice matters

**How to Contribute:**
- 🌟 Star this repo
- 💬 Join our Discord (coming soon)
- 📖 Read [CONTRIBUTING.md](CONTRIBUTING.md)
- 🐛 Report bugs and suggest features
- 💻 Write code
- 📝 Improve documentation
- 🎓 Help other users

**No experience required.** We mentor new contributors!

---

## 📚 Documentation

- [**VSH_SPECIFICATION.md**](VSH_SPECIFICATION.md) - Complete technical specification
- [**CONTRIBUTING.md**](CONTRIBUTING.md) - How to contribute (beginner-friendly!)
- [**GOVERNANCE.md**](GOVERNANCE.md) - How decisions are made
- [**CODE_OF_CONDUCT.md**](CODE_OF_CONDUCT.md) - Our community standards

---

## 🏗️ Project Structure

```
vsh/
├── README.md                  # You are here
├── VSH_SPECIFICATION.md       # Complete spec
├── CONTRIBUTING.md            # Contribution guide
├── GOVERNANCE.md              # Decision-making process
├── CODE_OF_CONDUCT.md         # Community standards
├── Cargo.toml                 # Rust project (coming soon)
├── src/                       # Source code (coming soon)
├── tests/                     # Test suite (coming soon)
├── docs/                      # Documentation (coming soon)
│   ├── rfcs/                  # RFCs for major decisions
│   └── commands/              # Command reference
└── examples/                  # Example .vsh scripts (coming soon)
```

---

## 🎯 Design Philosophy

1. **Cognitive Flexibility** - Support how different brains think
2. **Progressive Disclosure** - Simple for beginners, powerful for experts
3. **Helpful by Default** - Errors teach, don't frustrate
4. **Community-Driven** - Built by everyone, for everyone
5. **No Compromises** - Full shell power, zero degradation

---

## 🛣️ Roadmap

### Phase 1: MVP (Q2 2026)
- Core parser for 5-10 commands
- Multi-syntax support (terse, verbose, named)
- Basic error handling
- Configuration file support

### Phase 2: Intelligence (Q3 2026)
- User profiling system
- Adaptive auto-completion
- Smart error suggestions
- Learning algorithm

### Phase 3: Scripting (Q4 2026)
- Full .vsh scripting support
- Variables and control flow
- Functions
- Bash compatibility mode

### Phase 4: Polish (Q1 2027)
- Performance optimization
- Comprehensive help system
- Plugin architecture
- Production-ready release

### Phase 5: Ecosystem (2027+)
- Package manager
- Standard library
- Editor integration
- Migration tools

[See full roadmap in VSH_SPECIFICATION.md](VSH_SPECIFICATION.md#7-development-roadmap)

---

## 💻 Quick Start (Coming Soon)

```bash
# Install (future)
cargo install vsh

# Or build from source
git clone https://github.com/vsh-shell/vsh.git
cd vsh
cargo build --release
cargo install --path .

# Run
vsh
```

---

## 🎓 Examples

### File Operations
```bash
# Copy files
copy file.txt to backup/
cp file.txt backup/
copy source=file.txt destination=backup/

# Move files
move old.txt to new.txt
mv old.txt new.txt
move from=old.txt to=new.txt

# Remove files
remove file.txt
rm file.txt
delete file=file.txt
```

### Finding Files
```bash
# Traditional
find . -name "*.txt"

# Verbose
find files named "*.txt"

# Named parameters
find path=. name="*.txt" type=file
```

### Pipes
```bash
# All equivalent
cat file.txt | grep pattern | sort
cat file.txt -> grep pattern -> sort
cat file.txt then grep pattern then sort
```

### Scripts
```vsh
#!/usr/bin/env vsh

# Variables
name = "World"

# Conditionals
if file-exists "data.txt" then
    echo "Processing data..."
    process-data "data.txt"
else
    echo "No data file found"
end

# Loops
for file in *.txt do
    echo "Processing: $file"
    copy $file to backup/
end

# Functions
function greet name
    echo "Hello, $name!"
end

greet $name
```

---

## ❓ FAQ

### Is VSH a fork of bash/zsh?
No, it's built from scratch in Rust, but it can run bash scripts.

### Will my bash scripts work?
Yes! VSH detects `.sh` files and runs them in compatibility mode.

### Is this dumbed-down for beginners?
Not at all! It's the same power as bash/zsh, just more accessible.

### Can power users still be efficient?
Absolutely! Terse syntax works exactly like traditional shells.

### Why not just learn bash?
You should! VSH doesn't replace learning; it makes it easier.

### How is this different from Fish/Nushell?
- **Fish:** Great UX, but different syntax breaks muscle memory
- **Nushell:** Innovative structured data, but steep learning curve
- **VSH:** Traditional + modern, multiple syntaxes, learns your style

### When will it be ready?
First usable release: Q3 2026. Production-ready: Q1 2027.

### How can I help?
[Read CONTRIBUTING.md](CONTRIBUTING.md) and join the community!

---

## 🌟 Core Team

**Founder:** Victor Soto
**Core Team:** TBD (First election: June 2026)  
**Contributors:** See [CONTRIBUTORS.md](CONTRIBUTORS.md)

---

## 📜 License

**MIT License**

Copyright © 2026 VSH Contributors

See [LICENSE](LICENSE) for details.

---

## 🔗 Links

- **Website:** (coming soon)
- **GitHub:** https://github.com/notvcto/vsh
- **Discord:** (coming soon)
- **Twitter:** (coming soon)
- **Email:** (coming soon)

---

## 🙏 Acknowledgments

VSH is inspired by the incredible work of:
- Fish Shell team
- Nushell team
- PowerShell team
- Bash, Zsh, and all the shells that came before
- Every developer who's ever been frustrated by `find . -type f -name "*.txt" -exec grep -l "pattern" {} \;`

---

## 💬 Join the Conversation

**Have questions?** **Ideas?** **Want to help?**

- 💡 **Ideas:** GitHub Discussions
- 🐛 **Bugs:** GitHub Issues
- 💬 **Chat:** Discord (coming soon)
- 📧 **Email:** (coming soon)

---

<div align="center">

### **A shell for everyone, built by everyone.**

⭐ Star this repo to show support  
🔔 Watch for updates  
🍴 Fork to contribute  

**Let's make the command line accessible to millions.**

**Welcome to VSH.** 🚀

</div>
