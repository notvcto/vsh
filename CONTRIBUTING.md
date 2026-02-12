# Contributing to VSH

**Welcome!** 🎉

VSH is a shell for everyone, built by everyone. No matter your experience level, you can contribute.

---

## 🌟 Why Contribute?

- **Make shells accessible** - Help millions of people learn command line
- **Learn Rust** - Great project to learn or improve your Rust skills
- **Shape the future** - Your ideas directly influence VSH's direction
- **Join a community** - Meet other passionate developers
- **Build your portfolio** - Real, impactful open source work

---

## 🚀 Quick Start (5 minutes)

### 1. Join the Community
- ⭐ Star the repo on GitHub
- 💬 Join our Discord server (coming soon)
- 📖 Read the [VSH Specification](VSH_SPECIFICATION.md)

### 2. Set Up Your Environment
```bash
# Clone the repo
git clone https://github.com/vsh-shell/vsh.git
cd vsh

# Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build VSH
cargo build

# Run tests
cargo test

# Try it out!
cargo run
```

### 3. Find Something to Work On
Check out issues labeled:
- `good-first-issue` - Perfect for newcomers
- `help-wanted` - We need help here!
- `documentation` - Improve docs
- `beginner-friendly` - Low complexity

---

## 🎯 Ways to Contribute

### For Everyone (No Coding Required!)

#### 📝 Improve Documentation
- Fix typos or unclear explanations
- Add examples to command help
- Write tutorials
- Translate error messages

**Example:**
```markdown
Found: "The file does not exist"
Suggest: "Error: Could not find 'file.txt' in the current directory"
```

#### 🐛 Report Bugs
Help us find and fix issues:
1. Check if bug already reported
2. Create new issue with:
   - What you tried
   - What you expected
   - What actually happened
   - Your OS and VSH version

#### 💡 Suggest Features
Have an idea? We want to hear it!
1. Check GitHub Discussions for similar ideas
2. Create a new discussion or RFC
3. Explain the problem it solves
4. Give examples of how it would work

#### 🧪 Test Beta Features
- Try new releases
- Report what works and what doesn't
- Share feedback on new syntax

---

### For Coders

#### ✨ Add a New Command
Pick a command from our [Command Wishlist](https://github.com/vsh-shell/vsh/issues/XXX)

**Example: Adding `create-file` command**
```rust
// 1. Add to parser (src/parser/commands.rs)
"create-file" | "touch" => Intent::CreateFile,

// 2. Implement executor (src/executor/builtin.rs)
fn execute_create_file(args: &Args) -> Result<()> {
    let path = args.get_path("file")?;
    File::create(path)?;
    Ok(())
}

// 3. Add tests (tests/commands/create_file.rs)
#[test]
fn test_create_file() {
    let result = execute("create-file test.txt");
    assert!(result.is_ok());
    assert!(Path::new("test.txt").exists());
}

// 4. Update documentation (docs/commands/CREATE_FILE.md)
```

#### 🔧 Fix a Bug
1. Find a bug in Issues
2. Comment: "I'd like to work on this!"
3. Fork, fix, test, submit PR

#### ⚡ Optimize Performance
- Profile the code
- Find hot paths
- Optimize algorithms
- Benchmark improvements

---

## 📋 Contribution Workflow

### 1. Pick an Issue
Browse [Issues](https://github.com/vsh-shell/vsh/issues) and comment:
> "I'd like to work on this! ETA: [rough estimate]"

### 2. Fork & Branch
```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR-USERNAME/vsh.git
cd vsh
git checkout -b fix-issue-123
```

### 3. Make Changes
- Write clear, commented code
- Follow our style guide (run `cargo fmt`)
- Add tests for new features
- Update docs if needed

### 4. Test
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test

# Test manually
cargo run
```

### 5. Commit
Write clear commit messages:
```bash
git commit -m "Add verbose syntax for remove command

- Implements 'remove file' alongside 'rm file'
- Adds tests for both syntaxes
- Updates command documentation

Closes #123"
```

### 6. Push & Create PR
```bash
git push origin fix-issue-123
```

Then create a Pull Request on GitHub with:
- **Clear title**: "Add verbose syntax for remove command"
- **Description**: What you changed and why
- **Screenshots/demos** if UI-related
- **Link to issue**: "Closes #123"

### 7. Respond to Review
- Core team will review (usually within 48 hours)
- Make requested changes
- Push updates to the same branch
- Comment when ready for re-review

### 8. Celebrate! 🎉
Once merged:
- You'll be added to CONTRIBUTORS.md
- Your name in the release notes
- "Contributor" role on Discord
- Eternal glory!

---

## 🎨 Code Style Guide

### Rust Conventions
```rust
// Use descriptive names
fn parse_verbose_syntax(input: &str) -> Result<Command>  // ✅ Good
fn parse(s: &str) -> Result<Cmd>                          // ❌ Too terse

// Document public APIs
/// Parses a command in verbose syntax format
/// 
/// # Examples
/// ```
/// let cmd = parse_verbose_syntax("copy file.txt to backup/")?;
/// assert_eq!(cmd.intent, Intent::Copy);
/// ```
pub fn parse_verbose_syntax(input: &str) -> Result<Command>

// Use Result for errors, not panics
fn get_file(path: &str) -> Result<File>  // ✅ Good
fn get_file(path: &str) -> File          // ❌ Could panic

// Match VSH's style
match syntax_style {
    SyntaxStyle::Terse => parse_terse(input),
    SyntaxStyle::Verbose => parse_verbose(input),
    SyntaxStyle::Named => parse_named(input),
}
```

### Formatting
Run before committing:
```bash
cargo fmt
cargo clippy
```

### Testing
Every feature needs tests:
```rust
#[test]
fn test_copy_verbose_syntax() {
    let input = "copy file.txt to backup/";
    let cmd = parse(input).unwrap();
    
    assert_eq!(cmd.intent, Intent::Copy);
    assert_eq!(cmd.source, "file.txt");
    assert_eq!(cmd.destination, "backup/");
}
```

---

## 🏗️ Project Structure

```
vsh/
├── src/
│   ├── main.rs           # Entry point
│   ├── parser/           # Command parsing
│   │   ├── tokenizer.rs  # Break input into tokens
│   │   ├── intent.rs     # Determine what user wants
│   │   └── translator.rs # Convert to canonical form
│   ├── executor/         # Run commands
│   │   ├── builtin.rs    # Built-in commands
│   │   └── external.rs   # System commands
│   ├── profile/          # User profiling
│   └── config/           # Configuration
├── tests/                # Integration tests
├── docs/                 # Documentation
└── examples/             # Example .vsh scripts
```

**Where to start:**
- New commands → `src/executor/builtin.rs`
- Syntax support → `src/parser/`
- Tests → `tests/`
- Docs → `docs/`

---

## 🤝 Communication

### Discord Channels
- `#general` - Casual chat
- `#help` - Get unstuck
- `#development` - Technical discussions
- `#design` - Syntax and UX debates
- `#show-and-tell` - Share what you built!

### GitHub
- **Issues** - Bug reports and feature requests
- **Discussions** - Ideas and questions
- **Pull Requests** - Code contributions
- **RFCs** - Major proposals (see RFC process below)

### Community Calls
- **Monthly** - Last Saturday of each month
- **Open to all** - No preparation needed
- **Recorded** - Posted on YouTube
- **Agenda** - Posted in Discord #announcements

---

## 📜 RFC Process (For Major Changes)

**What needs an RFC?**
- New syntax variants
- Breaking changes
- Major architectural decisions
- Changes to core behavior

**Process:**
1. Copy `docs/rfcs/0000-template.md` to `docs/rfcs/0000-your-idea.md`
2. Fill in the template
3. Submit PR to `rfcs/` directory
4. Announce in Discord #rfc-discussion
5. 2-week discussion period
6. Community vote if no consensus
7. Implementation once approved

**Example RFCs:**
- RFC-0001: Parser Library Selection
- RFC-0002: Profiling Data Privacy
- RFC-0003: Windows Support Strategy

---

## ❓ FAQ

### "I'm new to Rust, can I contribute?"
**Absolutely!** Many contributions don't require Rust:
- Documentation
- Bug reports
- Design discussions
- Testing
- Example scripts

And if you want to learn Rust, VSH is a great project! Ask for help in `#help`.

### "I'm new to open source, is this okay?"
**Perfect place to start!** We're beginner-friendly:
- We label easy issues
- We mentor new contributors
- We're patient with questions
- We celebrate small wins

### "My idea was rejected, what now?"
Happens to everyone! Possible reasons:
- Doesn't fit VSH's vision
- Too complex for now
- Better approached differently

**What to do:**
- Ask for clarification
- Refine and re-propose
- Try a different contribution
- No hard feelings!

### "How do I become a core team member?"
Core team members are elected every 6 months:
1. Contribute consistently for a few months
2. Show good judgment and collaboration
3. Get nominated (or nominate yourself)
4. Community votes
5. Accept the role!

### "Can I work on this full-time?"
VSH is volunteer-driven right now. In the future:
- We may have GitHub Sponsors
- Companies might sponsor development
- Premium features might fund core work

But for now, it's a labor of love!

---

## 🎓 Learning Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### Parsers
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Nom Tutorial](https://github.com/Geal/nom/tree/main/doc)
- [Pest Book](https://pest.rs/book/)

### Shells
- [Build Your Own Shell](https://github.com/kamalmarhubi/shell-workshop)
- [How Shells Work](https://www.gnu.org/software/bash/manual/)

---

## 💖 Code of Conduct

**TL;DR: Be kind, be respectful, be constructive.**

We're building VSH to be inclusive. That starts with how we treat each other:

- **Be welcoming** - Everyone was new once
- **Be patient** - Take time to explain and understand
- **Be respectful** - Disagree with ideas, not people
- **Be constructive** - Suggest improvements, don't just criticize
- **Be professional** - This is a public space

**Not okay:**
- Harassment, discrimination, or hate speech
- Personal attacks or insults
- Trolling or deliberately derailing discussions
- Sharing private information without consent

**Enforcement:**
1. Warning for first offense
2. Temporary ban for repeated issues
3. Permanent ban for severe violations

Report issues to: conduct@vsh.dev or to any core team member

---

## 🙏 Thank You!

Every contribution matters:
- The typo fix that helps someone understand
- The bug report that prevents crashes
- The feature idea that makes VSH better
- The encouragement that keeps us going

**You're helping make command line accessible to millions.**

**Welcome to the VSH community!** 🚀

---

## 📞 Need Help?

- 💬 **Discord `#help` channel** (coming soon)
- 📧 **Email**: (coming soon)
- 💭 **GitHub Discussions** - For longer questions
- 🐦 **Twitter**: (coming soon)

**Don't be shy - we're here to help!**

---

**Happy Contributing!** ✨
