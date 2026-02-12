# VSH - Vic's Shell
## Complete Project Specification v0.1

---

## 1. Vision Statement

**VSH (Vic's Shell)** is a next-generation shell that maintains the full power of traditional shells while dramatically improving ergonomics, discoverability, and accessibility for users at all skill levels.

### Core Philosophy
- **Cognitive Flexibility**: Multiple syntax styles for the same command - your brain, your choice
- **Progressive Disclosure**: Beginners use verbose syntax; power users stay terse
- **Adaptive Intelligence**: The shell learns your style and adapts to you
- **No Compromise**: Full shell power with zero degradation from bash/zsh
- **For Everyone**: Beginners, power users, and everyone in between

---

## 2. Design Principles

1. **Multiple Valid Syntaxes** - Commands work in terse, verbose, and named-parameter forms
2. **Intelligent Adaptation** - Shell profiles user behavior and adapts suggestions
3. **Readable Scripts** - .vsh files should read like pseudocode
4. **Helpful Errors** - Errors suggest fixes and explain problems in plain English
5. **Backwards Compatible** - Run .sh scripts natively with bash compatibility mode
6. **Offline-First** - All intelligence runs locally, no cloud dependencies
7. **Fast & Reliable** - Built in Rust for safety and performance

---

## 3. Core Features

### 3.1 Multi-Syntax Command Support

Every command supports multiple syntax variants:

**Example: File Copy**
```bash
# Traditional (bash-compatible)
cp file.txt backup/

# Verbose natural language
copy file.txt to backup/

# Named parameters
copy source=file.txt destination=backup/

# Mixed styles (all valid)
copy file.txt destination=backup/
cp source=file.txt to backup/
```

**Example: Find Files**
```bash
# Traditional
find . -type f -name "*.txt"

# Verbose
find files with-extension txt in current-directory

# Named parameters
find type=file name="*.txt" path=.

# Natural hybrid
find files named "*.txt" in .
```

### 3.2 Intelligent User Profiling

VSH builds a profile of each user based on:
- **Command history patterns** (verbose vs terse usage ratio)
- **Error recovery patterns** (what helps them succeed)
- **Syntax preference trends** (converging on a style)
- **Typing patterns** (command length, completion usage)
- **Setup questionnaire** (explicit skill level declaration)

Profile is stored in `~/.vsh/profile.json` and continuously updated.

### 3.3 Smart Error Handling

When errors occur, VSH provides:

```bash
$ copy file.txt to nonexistent/
✗ Error: Destination directory does not exist

Suggestion: Did you mean to create it first?
  → mkdir nonexistent && copy file.txt to nonexistent/

Or did you mean one of these?
  → copy file.txt to Documents/
  → copy file.txt to backup/

[?] Need help? Type 'help copy' for more information
```

Features:
- **Typo correction** with suggestions
- **Context-aware hints** based on directory structure
- **Plain English explanations** of what went wrong
- **Example commands** that would work
- **Help system integration** for learning

### 3.4 Adaptive Auto-Completion

Tab completion that:
- **Learns your preference** (verbose vs terse)
- **Offers all valid syntaxes** when uncertain
- **Prioritizes your style** as confidence grows
- **Shows examples inline** for unfamiliar commands

```bash
$ cop<TAB>
→ copy          (verbose style - you use this 80% of the time)
  cp            (terse style)
  copy <file>   (show example)
```

### 3.5 Script Compatibility

**Run .sh files natively:**
```bash
$ ./legacy-script.sh
[VSH: Running in bash compatibility mode]
```

**Write .vsh files with enhanced syntax:**
```vsh
#!/usr/bin/env vsh

# Variables - both styles work
backup_dir = "~/backups"
set log_file = "/var/log/backup.log"

# Conditionals
if directory-exists $backup_dir then
    echo "Backup directory ready"
else
    mkdir $backup_dir
    echo "Created backup directory"
end

# Loops - readable syntax
for file in *.txt do
    copy $file to $backup_dir
    echo "Backed up: $file" | append-to $log_file
end

# Functions
function compress-backup
    tar -czf backup.tar.gz $backup_dir
    echo "Backup compressed"
end

compress-backup
```

### 3.6 Pipe Flexibility

Multiple pipe operators for cognitive flexibility:

```bash
# Traditional
cat file.txt | grep "pattern" | sort

# Arrow style
cat file.txt -> grep "pattern" -> sort

# Natural language
cat file.txt then grep "pattern" then sort

# All valid and interchangeable
```

---

## 4. Architecture

### 4.1 High-Level Components

```
┌─────────────────────────────────────────────┐
│            VSH Shell (main)                 │
├─────────────────────────────────────────────┤
│  ┌────────────┐  ┌──────────────┐          │
│  │   Parser   │→ │  Intent      │          │
│  │  Engine    │  │  Analyzer    │          │
│  └────────────┘  └──────────────┘          │
│         ↓               ↓                   │
│  ┌────────────────────────────┐            │
│  │   Command Translator       │            │
│  │  (Syntax → Canonical AST)  │            │
│  └────────────────────────────┘            │
│         ↓                                   │
│  ┌────────────────────────────┐            │
│  │   Executor                 │            │
│  │  (Runs canonical commands) │            │
│  └────────────────────────────┘            │
├─────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │ Profile  │  │  Config  │  │  Compat  │ │
│  │ Manager  │  │  System  │  │  Layer   │ │
│  └──────────┘  └──────────┘  └──────────┘ │
└─────────────────────────────────────────────┘
```

### 4.2 Parser Engine

**Multi-Stage Pipeline:**

1. **Tokenization**
   - Split input into tokens (commands, args, operators)
   - Handle quotes, escapes, variables

2. **Syntax Detection**
   - Identify which syntax variant is being used
   - Pattern match against known command structures

3. **Intent Analysis**
   - Determine what the user is trying to do
   - Extract semantic meaning from syntax

4. **Canonical Translation**
   - Convert any valid syntax to internal AST
   - Normalize to consistent representation

5. **Validation**
   - Check arguments, paths, permissions
   - Provide intelligent errors if invalid

**Example Translation:**

```rust
// Input variants
"copy file.txt to backup/"
"cp file.txt backup/"
"copy source=file.txt destination=backup/"

// All translate to canonical AST:
Command {
    intent: Copy,
    args: {
        source: Path("file.txt"),
        destination: Path("backup/"),
    }
}
```

### 4.3 User Profiling System

**Profile Structure:**
```rust
struct UserProfile {
    // Syntax preferences
    verbose_ratio: f32,           // 0.0 = always terse, 1.0 = always verbose
    preferred_syntax: SyntaxStyle, // Dominant style
    
    // Behavior metrics
    avg_command_length: usize,
    completion_usage_rate: f32,
    error_rate: f32,
    
    // Learning indicators
    commands_executed: u64,
    days_active: u32,
    skill_estimate: SkillLevel,   // Beginner, Intermediate, Advanced
    
    // Adaptation state
    confidence: f32,               // How confident we are in profile
    last_updated: DateTime,
}

enum SyntaxStyle {
    Terse,          // cp, mv, rm
    Verbose,        // copy, move, remove
    Named,          // copy source=x dest=y
    Mixed,          // User uses multiple styles
}

enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    PowerUser,
}
```

**Profiling Algorithm:**

```rust
fn update_profile(profile: &mut UserProfile, command: &Command) {
    // Track syntax style
    let syntax = detect_syntax_style(command);
    profile.update_syntax_preference(syntax);
    
    // Track success/failure
    if command.succeeded() {
        profile.error_rate = profile.error_rate * 0.95;
    } else {
        profile.error_rate = profile.error_rate * 1.05;
    }
    
    // Estimate skill level
    profile.skill_estimate = estimate_skill(profile);
    
    // Increase confidence over time
    profile.confidence = min(1.0, profile.confidence + 0.001);
    profile.commands_executed += 1;
}
```

### 4.4 Configuration System

**Configuration File: `~/.vshrc`**

```vsh
# VSH Configuration File

# Syntax preferences (optional - shell will learn automatically)
set default-syntax = "verbose"  # or "terse" or "named"

# Error handling
set show-suggestions = true
set show-examples = true
set explain-errors = true
set typo-correction = true
set suggestion-count = 3

# Compatibility
set bash-mode = "auto"          # "auto", "always", "never"
set enhance-bash-scripts = true # Better errors for .sh files
set run-sh-in-subprocess = false

# Auto-completion
set completion-style = "adaptive" # "adaptive", "all-options", "preferred-only"
set completion-delay = 100       # ms before showing completions

# Profiling
set enable-profiling = true
set profile-path = "~/.vsh/profile.json"

# History
set history-size = 10000
set history-file = "~/.vsh/history"

# Appearance
set prompt = "vsh %d $ "
set color-scheme = "default"
set show-git-status = true

# Advanced
set parser-strictness = "lenient" # "strict", "lenient"
set cache-parsed-scripts = true
```

### 4.5 Bash Compatibility Layer

**Modes:**

1. **Auto Mode** (default)
   - Detect .sh files → bash compatibility
   - Detect .vsh files → VSH syntax
   - Interactive shell → VSH syntax with bash fallback

2. **Always Mode**
   - Everything runs in bash compatibility
   - VSH features disabled

3. **Never Mode**
   - Pure VSH mode
   - .sh files may have issues

**Implementation:**
```rust
enum CompatMode {
    Auto,
    Always,
    Never,
}

fn should_use_bash_compat(file: &Path, mode: CompatMode) -> bool {
    match mode {
        CompatMode::Always => true,
        CompatMode::Never => false,
        CompatMode::Auto => {
            file.extension() == Some("sh") || 
            file.starts_with("#!/bin/bash") ||
            file.starts_with("#!/bin/sh")
        }
    }
}
```

---

## 5. Command Syntax Specification

### 5.1 Syntax Variants

Every command supports three main styles:

#### Terse (Bash-Compatible)
```bash
cp source dest
mv old new
rm file
ls -la
grep pattern file
```

#### Verbose (Natural Language)
```bash
copy source to dest
move old to new
remove file
list all-details
grep pattern in file
```

#### Named Parameters
```bash
copy source=file dest=backup/
move from=old to=new
remove file=document.txt
list show=all-details
grep pattern="error" file=log.txt
```

### 5.2 Core Commands

**File Operations:**
```bash
# Copy
cp file.txt backup/
copy file.txt to backup/
copy source=file.txt destination=backup/

# Move
mv old.txt new.txt
move old.txt to new.txt
move from=old.txt to=new.txt

# Remove
rm file.txt
remove file.txt
delete file.txt
remove file=file.txt

# Create directory
mkdir newdir
create-directory newdir
make directory=newdir
```

**File Search:**
```bash
# Find files
find . -name "*.txt"
find files named "*.txt"
find files with-name "*.txt" in .
find path=. name="*.txt" type=file

# Grep
grep "pattern" file.txt
search "pattern" in file.txt
grep pattern="pattern" file=file.txt
```

**Directory Navigation:**
```bash
# Change directory
cd /path/to/dir
change-directory /path/to/dir
goto /path/to/dir
cd path=/path/to/dir

# List
ls -la
list all-details
list show=all details=true
```

### 5.3 Variables

**Declaration:**
```bash
# All valid
name = "value"
set name = "value"
let name = "value"
var name = "value"
```

**Usage:**
```bash
# All valid
$name
${name}
{name}
```

**Types (Optional):**
```bash
# Strings
name = "John"
set message = "Hello World"

# Numbers
count = 42
set price = 19.99

# Lists
files = ["a.txt", "b.txt", "c.txt"]
set numbers = [1, 2, 3, 4, 5]

# Type inference by default, explicit typing optional
set count: number = 42
set name: string = "Alice"
```

### 5.4 Control Flow

**Conditionals:**
```bash
# If-then-else
if condition then
    commands
else
    other commands
end

# One-line
if condition then command

# Conditions
if file-exists "test.txt" then echo "Found!"
if $count > 10 then echo "Large"
if directory-exists $path then cd $path
```

**Loops:**
```bash
# For loop
for item in list do
    echo $item
end

# While loop
while condition do
    commands
end

# Iterate files
for file in *.txt do
    process $file
end

# Iterate numbers
for i in 1..10 do
    echo $i
end
```

**Functions:**
```bash
# Function definition
function backup-files
    for file in *.txt do
        copy $file to ~/backup/
    end
end

# With parameters
function greet name
    echo "Hello, $name!"
end

# With return values
function add x y
    return $(($x + $y))
end

# Call
backup-files
greet "Alice"
result = add 5 3
```

### 5.5 Pipes and Redirection

**Pipes:**
```bash
# All equivalent
cat file.txt | grep pattern | sort
cat file.txt -> grep pattern -> sort
cat file.txt then grep pattern then sort
```

**Redirection:**
```bash
# Output
command > file.txt
command output-to file.txt
command redirect-output file.txt

# Append
command >> file.txt
command append-to file.txt

# Input
command < input.txt
command input-from input.txt
```

---

## 6. Error Handling & Help System

### 6.1 Error Messages

**Structure:**
```
✗ Error: [What went wrong]

[Plain English explanation]

Suggestion: [How to fix it]
  → [Suggested command]

[Alternative suggestions if applicable]

[?] Need help? Type 'help [command]' for more information
```

**Examples:**

```bash
$ copy nonexistent.txt backup/
✗ Error: Source file does not exist

The file 'nonexistent.txt' could not be found in the current directory.

Suggestion: Check the filename or path
  → list files containing "nonexistent"
  
Did you mean one of these?
  → copy document.txt to backup/
  → copy notes.txt to backup/

[?] Need help? Type 'help copy' for more information
```

```bash
$ remove important-file.txt
⚠ Warning: This will permanently delete 'important-file.txt'

This action cannot be undone.

Suggestion: Create a backup first
  → copy important-file.txt to backup/ && remove important-file.txt

Continue? [y/N]: 
```

### 6.2 Help System

**Integrated help:**
```bash
$ help copy

COPY - Copy files or directories

SYNTAX VARIANTS:
  copy <source> to <destination>
  cp <source> <destination>
  copy source=<path> destination=<path>

EXAMPLES:
  copy file.txt to backup/
  copy *.txt to documents/
  cp -r folder/ backup/

OPTIONS:
  --recursive, -r     Copy directories recursively
  --verbose, -v       Show detailed output
  --force, -f         Overwrite without asking

SEE ALSO: move, remove, link
```

**Context-sensitive help:**
```bash
$ copy <TAB>
[Shows files in current directory]

$ copy file.txt <TAB>
→ to [destination]
  into [directory]
  destination=

$ copy file.txt to <TAB>
[Shows directories]
```

---

## 7. Development Roadmap

### Phase 1: MVP (Proof of Concept)
**Goal:** Demonstrate core parsing and multi-syntax support

**Features:**
- Basic parser for 3-5 core commands (copy, move, remove, list)
- Support for terse and verbose syntax
- Simple command execution
- Basic error messages
- .vshrc configuration file

**Deliverable:** Working shell that can execute simple file operations

**Timeline:** 4-6 weeks

---

### Phase 2: Intelligence Layer
**Goal:** Add adaptive profiling and smart completion

**Features:**
- User profiling system
- Command history tracking
- Adaptive auto-completion
- Improved error messages with suggestions
- Learning algorithm implementation

**Deliverable:** Shell that learns user preferences

**Timeline:** 6-8 weeks

---

### Phase 3: Script Support
**Goal:** Full scripting capabilities with .vsh files

**Features:**
- Variables and data types
- Control flow (if/for/while)
- Functions
- Script execution engine
- Bash compatibility mode for .sh files

**Deliverable:** Full scripting language

**Timeline:** 8-10 weeks

---

### Phase 4: Advanced Features
**Goal:** Polish and power-user features

**Features:**
- Advanced pipe operators
- Named parameters for all commands
- Comprehensive help system
- Plugin/extension system
- Performance optimizations
- Full test coverage

**Deliverable:** Production-ready shell

**Timeline:** 10-12 weeks

---

### Phase 5: Ecosystem
**Goal:** Tools, documentation, and community

**Features:**
- VSH package manager
- Standard library of .vsh scripts
- Comprehensive documentation
- Tutorial system
- Syntax highlighting for editors
- Migration tools from bash/zsh

**Deliverable:** Complete ecosystem

**Timeline:** Ongoing

---

## 8. Technical Stack

### 8.1 Core Technologies

**Language:** Rust (stable)

**Key Crates:**
- `clap` - CLI argument parsing
- `rustyline` - Interactive line editing and history
- `nom` or `pest` - Parser combinators
- `serde` - Serialization (for config/profile)
- `tokio` - Async runtime (for background tasks)
- `colored` - Terminal colors
- `shellexpand` - Path expansion
- `dirs` - Platform-specific directories

### 8.2 Architecture Crates

```toml
[dependencies]
# Parsing
nom = "7.1"
pest = "2.7"

# CLI & REPL
clap = { version = "4.4", features = ["derive"] }
rustyline = "13.0"
colored = "2.1"

# Data & Config
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# System interaction
nix = "0.27"
libc = "0.2"

# Utilities
anyhow = "1.0"
thiserror = "1.0"
log = "0.4"
env_logger = "0.11"

# Async (optional, for future features)
tokio = { version = "1.35", features = ["full"] }
```

### 8.3 Project Structure

```
vsh/
├── Cargo.toml
├── README.md
├── LICENSE
├── .github/
│   └── workflows/
│       └── ci.yml
├── src/
│   ├── main.rs                 # Entry point
│   ├── lib.rs                  # Library root
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── tokenizer.rs
│   │   ├── syntax_detector.rs
│   │   ├── intent_analyzer.rs
│   │   └── translator.rs
│   ├── executor/
│   │   ├── mod.rs
│   │   ├── command.rs
│   │   ├── builtin.rs
│   │   └── external.rs
│   ├── profile/
│   │   ├── mod.rs
│   │   ├── profiler.rs
│   │   └── analyzer.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── vshrc.rs
│   ├── completion/
│   │   ├── mod.rs
│   │   └── adaptive.rs
│   ├── error/
│   │   ├── mod.rs
│   │   └── suggestions.rs
│   ├── compat/
│   │   ├── mod.rs
│   │   └── bash.rs
│   └── repl/
│       ├── mod.rs
│       └── interactive.rs
├── tests/
│   ├── integration/
│   └── fixtures/
├── examples/
│   ├── basic.vsh
│   └── advanced.vsh
└── docs/
    ├── SYNTAX.md
    ├── COMMANDS.md
    └── CONTRIBUTING.md
```

---

## 9. File Formats

### 9.1 .vsh Script Files

**Shebang:**
```bash
#!/usr/bin/env vsh
```

**Example:**
```vsh
#!/usr/bin/env vsh
# Example VSH script

# Variables
backup_dir = "~/backups"
log_file = "/var/log/backup.log"

# Function
function create-backup source
    if not directory-exists $backup_dir then
        create-directory $backup_dir
    end
    
    copy $source to $backup_dir
    echo "Backed up: $source" >> $log_file
end

# Main logic
for file in *.txt do
    create-backup $file
end

echo "Backup complete!"
```

### 9.2 .vshrc Configuration

```vsh
# VSH Configuration

# Appearance
set prompt = "vsh %d $ "
set color-scheme = "monokai"

# Behavior
set default-syntax = "verbose"
set completion-style = "adaptive"

# Features
set show-suggestions = true
set enable-profiling = true

# Aliases
alias ll = "list all-details"
alias backup = "copy to ~/backups/"

# Custom functions
function quick-backup
    copy $1 to ~/backups/
end
```

### 9.3 Profile Storage

**File:** `~/.vsh/profile.json`

```json
{
  "version": "0.1.0",
  "created": "2026-02-11T00:00:00Z",
  "last_updated": "2026-02-11T12:30:00Z",
  "syntax_preferences": {
    "verbose_ratio": 0.75,
    "preferred_syntax": "Verbose",
    "terse_count": 50,
    "verbose_count": 150,
    "named_count": 25
  },
  "behavior_metrics": {
    "avg_command_length": 25,
    "completion_usage_rate": 0.60,
    "error_rate": 0.05,
    "commands_executed": 1250,
    "days_active": 30
  },
  "skill_estimate": "Intermediate",
  "confidence": 0.85
}
```

---

## 10. Success Metrics

### 10.1 User Experience Goals

- **Beginner onboarding:** New users execute successful commands within 2 minutes
- **Error recovery:** 80%+ of errors include actionable suggestions
- **Syntax flexibility:** Users can mix syntaxes without confusion
- **Learning curve:** Intermediate proficiency within 1 week of daily use

### 10.2 Technical Goals

- **Performance:** Command execution < 50ms overhead vs bash
- **Parsing accuracy:** 99%+ correct intent detection
- **Profile accuracy:** 85%+ syntax prediction accuracy after 100 commands
- **Compatibility:** 95%+ bash scripts run without modification

### 10.3 Community Goals

- **Adoption:** 1000+ active users within 6 months
- **Contributions:** 10+ external contributors
- **Documentation:** 100% command coverage in help system
- **Scripts:** Library of 50+ example .vsh scripts

---

## 11. Community & Governance

**VSH is a fully community-driven project.** The shell is for everyone, so it should be built BY everyone.

### 11.1 Core Philosophy

**Why Community-Driven?**
- A shell for all skill levels needs input from all skill levels
- Diverse users = diverse perspectives = better design
- Sustainability beyond any single maintainer
- Transparency builds trust
- Collective wisdom > individual opinion

**Principles:**
- **Open from Day One** - All development happens in public
- **Inclusive by Default** - All skill levels welcome to contribute
- **Transparent Decisions** - Process and rationale documented
- **Credit Everyone** - Recognition for all contributions
- **Iterate Together** - Community votes on major decisions

### 11.2 Governance Model

**Decision-Making Structure:**

```
┌─────────────────────────────────────────┐
│         Community (Everyone)            │
│  - Votes on major decisions             │
│  - Proposes features via RFCs           │
│  - Reviews and discusses                │
└────────────┬────────────────────────────┘
             │
┌────────────▼────────────────────────────┐
│      Core Team (3-7 members)            │
│  - Day-to-day decisions                 │
│  - Review PRs                           │
│  - Maintain quality standards           │
│  - Elected by community (6-month terms) │
└────────────┬────────────────────────────┘
             │
┌────────────▼────────────────────────────┐
│         Founder (Vic)                   │
│  - Vision keeper                        │
│  - Tie-breaker on deadlocks             │
│  - Can step back once mature            │
└─────────────────────────────────────────┘
```

**Major Decisions (Require Community Vote):**
- Syntax changes affecting existing commands
- Breaking changes to .vsh file format
- Default configuration changes
- Adding/removing core features
- Governance structure changes

**Minor Decisions (Core Team):**
- Bug fixes
- Performance improvements
- New optional features (can be toggled off)
- Documentation updates
- Non-breaking enhancements

**RFC Process:**
1. Anyone submits RFC (Request for Comments) via GitHub
2. 2-week discussion period
3. Core team summarizes feedback
4. Community vote if needed (>50% approval)
5. Implementation by volunteers

### 11.3 Contribution Pathways

**Everyone Can Contribute - Regardless of Skill Level**

**For Beginners:**
- 🐛 **Report bugs** - Help us find issues
- 📝 **Improve docs** - Fix typos, clarify instructions
- 💬 **Answer questions** - Help others in discussions
- 🧪 **Test features** - Try new releases, report feedback
- 🎨 **Design examples** - Create .vsh script examples

**For Intermediate:**
- ✨ **Add commands** - Implement new built-in commands
- 🔧 **Fix bugs** - Tackle good-first-issue labels
- 📚 **Write tutorials** - Help others learn VSH
- 🎯 **Triage issues** - Help organize and prioritize
- 🌐 **Translate** - Localize error messages

**For Advanced:**
- ⚡ **Core features** - Parser, profiler, executor improvements
- 🏗️ **Architecture** - Design and implement major systems
- 🔬 **Performance** - Optimize hot paths
- 🛡️ **Security** - Review and harden code
- 🎓 **Mentorship** - Guide new contributors

### 11.4 Communication Channels

**GitHub (Primary Hub):**
- **Discussions** - General chat, questions, ideas
- **Issues** - Bug reports, feature requests
- **Pull Requests** - Code contributions
- **RFCs** - Major proposals in `/rfcs` directory
- **Wiki** - Community-maintained documentation

**Discord Server:**
- `#general` - Casual chat
- `#help` - Support for users
- `#development` - Technical discussions
- `#design` - Syntax and UX debates
- `#rfc-discussion` - Live RFC debates
- `#showcase` - Share .vsh scripts

**Monthly Community Calls:**
- Open to everyone
- Demo new features
- Discuss roadmap
- Live Q&A
- Recorded and posted on YouTube

**Social Media:**
- Twitter/X for announcements
- Reddit r/vsh for discussions
- Hacker News for launches

### 11.5 Recognition & Credit

**All Contributors Recognized:**
- `CONTRIBUTORS.md` - Everyone who contributes
- Release notes credit all contributors
- "Contributor" role in Discord
- Special recognition for major features

**Hall of Fame:**
- Top monthly contributors
- Most helpful community members
- Best .vsh script submissions
- Outstanding bug reporters

**Maintainer Election:**
- Every 6 months, community votes for core team
- Nominations based on contributions
- 3-7 members (odd number for tie-breaking)
- Anyone can run if they've contributed

### 11.6 Code of Conduct

**Core Values:**
- **Be Kind** - Assume good intentions
- **Be Inclusive** - Welcome all backgrounds and skill levels
- **Be Constructive** - Critique ideas, not people
- **Be Patient** - Everyone's learning
- **Be Respectful** - Disagreement is fine, disrespect is not

**Enforcement:**
- Warnings for first offense
- Temp ban for repeated issues
- Permanent ban for severe violations
- Core team makes final decisions

### 11.7 Development Workflow

**Open Development:**
```bash
# Everything happens in public
1. Discussion in GitHub Issues/Discussions
2. RFC for major changes
3. Implementation in feature branch
4. PR with tests and documentation
5. Review by core team + community
6. Merge when approved
7. Credit in release notes
```

**Quality Standards:**
- Tests required for new features
- Documentation required for user-facing changes
- Code review by at least 2 people
- CI must pass (formatting, tests, lints)
- No merge until consensus reached

**Release Cycle:**
- **Patch releases** (0.1.x) - Bug fixes, every 2 weeks
- **Minor releases** (0.x.0) - New features, every 2 months
- **Major releases** (x.0.0) - Breaking changes, when ready

### 11.8 Funding & Sustainability

**Initial Bootstrap:**
- Free and open source (MIT/Apache 2.0)
- No commercial backing required
- Volunteer-driven development

**Future Options (If Needed):**
- GitHub Sponsors for infrastructure costs
- OpenCollective for transparent finances
- Corporate sponsorships (no influence on decisions)
- Optional premium features (cloud sync, etc.) - revenue shared with contributors

**Principles:**
- Core shell always free
- No paywalls for essential features
- Community controls direction, not sponsors
- Transparent finances

### 11.9 Bootstrapping the Community

**Phase 1: Initial Launch (Month 1)**
- Release spec publicly on GitHub
- Create Discord server
- Post on Hacker News, Reddit, Twitter
- Invite initial contributors
- Set up CI/CD and documentation

**Phase 2: First Contributors (Months 2-3)**
- Label good-first-issues
- Mentor new contributors
- Hold first community call
- Establish RFC process
- Get to 10+ active contributors

**Phase 3: Community Growth (Months 4-6)**
- First community vote on major decision
- Elect initial core team (3-5 people)
- Launch contributor recognition program
- Hit 50+ contributors
- Stable release candidate

**Phase 4: Maturity (Months 7-12)**
- Community self-sustaining
- Regular release cadence
- Active Discord with daily activity
- 100+ contributors
- First 1.0 release

### 11.10 Community-Driven Features

**Examples of Community Input:**

**Syntax Design:**
- Multiple variants exist BECAUSE different people think differently
- Community votes on which aliases to include
- Users propose new syntax patterns

**Error Messages:**
- Beginners report which errors are confusing
- Community suggests better wording
- A/B testing with real users

**Platform Support:**
- Windows users drive Windows compatibility
- macOS users handle macOS quirks
- Linux distro maintainers package for their distros

**Localization:**
- Native speakers translate error messages
- Cultural context for examples
- Region-specific defaults

---

## 12. Open Questions & Future Considerations

### 12.1 To Resolve (Community Input Needed!)

1. **Parser implementation:** `nom` vs `pest` vs custom? → Let's vote!
2. **Profile privacy:** How to handle sensitive command history?
3. **Update mechanism:** Auto-updates vs manual?
4. **Telemetry:** Anonymous usage stats (opt-in)? What data?
5. **Cross-platform:** Windows support strategy?
6. **Default prompt:** What should `vsh $ ` look like?
7. **License:** MIT vs Apache 2.0 vs dual?

### 12.2 Future Features (Community Will Decide)

- **AI Integration:** Natural language command generation (opt-in)
- **Cloud Sync:** Profile sync across machines
- **Team Profiles:** Shared syntax standards for teams
- **Plugin System:** Community extensions
- **Visual Mode:** GUI overlay for beginners
- **Voice Commands:** Speech-to-command translation
- **Package Manager:** Install .vsh scripts from community repo

---

## 13. Getting Started (For Contributors)

### Prerequisites
- Rust 1.70+ installed
- Basic shell scripting knowledge
- Familiarity with parsers (optional but helpful)

### Quick Start
```bash
# Clone repo
git clone https://github.com/vic/vsh.git
cd vsh

# Build
cargo build

# Run tests
cargo test

# Run VSH
cargo run

# Install locally
cargo install --path .
```

### Contributing
See `docs/CONTRIBUTING.md` for:
- Code style guidelines
- Testing requirements
- PR process
- Command implementation guide

---

## 14. Vision from the Founder

**From Vic:**

> I created VSH because shells shouldn't be gatekeepers. They should be gateways.
> 
> I've watched too many brilliant people bounce off the command line because `cp` doesn't make sense, because `find . -type f -name "*.txt" -exec grep -l "pattern" {} \;` looks like line noise, because one misplaced quote breaks everything.
> 
> But I've also watched power users accomplish incredible things with the same tools.
> 
> VSH shouldn't make you choose. Beginners get readable commands. Power users get efficiency. Everyone gets a shell that adapts to THEM, not the other way around.
> 
> But here's the thing - I can't build this alone. A shell for everyone NEEDS everyone.
> 
> This is our project. Community-driven from day one. Your ideas, your code, your vision.
> 
> Let's build something that makes the command line accessible to millions, without dumbing it down for anyone.
> 
> Let's build VSH.
> 
> — Vic

---

## 15. License & Ownership

**License:** MIT License (chosen by community vote)

**Copyright:** © 2026 VSH Contributors

**Why MIT?**
- Maximum freedom for users
- Compatible with commercial and open source projects
- Simple and well-understood
- Encourages adoption and contribution

**Trademark:** "VSH" and "Vic's Shell" are community-owned. No single entity controls the brand.

**Governance:** See Section 11 - Community & Governance

**Contributors Retain Copyright:** All contributors retain copyright to their contributions, licensed under MIT to the project.

---

## 16. Credits & Inspiration

**Created by:** Victor Soto (Founder & Vision Keeper)

**Built by:** The VSH Community (see CONTRIBUTORS.md)

**Inspired by:**
- **Fish Shell** - Intelligent auto-completion and helpful suggestions
- **Nushell** - Structured data and modern design
- **PowerShell** - Verbose, discoverable command syntax
- **Bash/Zsh** - Decades of proven shell design
- **Elvish** - Innovative parsing and pipeline ideas
- **Oil Shell** - Better language design for shell scripts

**Special Thanks:**
- Everyone who reported their frustrations with traditional shells
- The open source community for creating the tools we build on
- Early contributors and testers who shape VSH's direction

---

## 17. Call to Action

**Join Us!**

VSH is just getting started, and we need YOU:

🌟 **Star the repo** - Show support  
💬 **Join Discord** - Connect with the community  
🐛 **Report bugs** - Help us improve  
💡 **Propose features** - Share your ideas  
🔧 **Write code** - Build the future  
📝 **Improve docs** - Make it better for everyone  
🎓 **Mentor others** - Help newcomers contribute

**Ready to contribute?**
1. Read `CONTRIBUTING.md`
2. Check out `good-first-issue` labels
3. Join Discord `#development` channel
4. Say hi and ask questions!

**No experience required.** We'll help you get started.

**Links:**
- GitHub: `https://github.com/vic/vsh` (coming soon!)
- Discord: `https://discord.gg/vsh` (coming soon!)
- Website: `https://vsh.dev` (coming soon!)

---

**Together, we're building a shell for everyone.**

**Welcome to VSH.** 🚀

---

---

## 18. Community Bootstrap Plan

**How We Launch VSH as a Community Project from Day One**

### Week 1: Foundation
- [ ] Create GitHub repo: `vic/vsh` (personal repo - free!)
- [ ] Publish this spec as first commit
- [ ] Set up issue templates (bug, feature, RFC)
- [ ] Create `CONTRIBUTING.md` with beginner-friendly guide
- [ ] Create `CODE_OF_CONDUCT.md`
- [ ] Enable GitHub Discussions
- [ ] Create Discord server with channels
- [ ] Set up basic CI (GitHub Actions - free for public repos)
- [ ] Add topic tags: `shell`, `rust`, `cli`, `command-line`, `terminal`

**Note:** Starting with personal repo keeps it free. Can transfer to organization later if needed (GitHub makes this easy).

### Week 2: Community Seeding
- [ ] Post on Hacker News: "VSH - A Shell for Everyone, Built by Everyone"
- [ ] Post on Reddit: r/rust, r/commandline, r/opensource
- [ ] Tweet announcement with spec link
- [ ] Create project website (GitHub Pages is fine)
- [ ] Write blog post: "Why VSH is Community-Driven from Day One"
- [ ] Invite 5-10 initial interested people to be founding contributors

### Week 3: First Contributions
- [ ] Create 10 "good-first-issue" labels across different areas:
  - Documentation improvements
  - Example .vsh scripts
  - Project setup (Cargo.toml, structure)
  - Simple command implementations
  - Test framework setup
- [ ] Host first community call (30 min)
  - Introduce vision
  - Answer questions
  - Discuss initial architecture decisions
- [ ] Start first RFC: "Parser Library Selection"

### Week 4: Momentum Building
- [ ] Merge first 5 PRs from community
- [ ] Update CONTRIBUTORS.md
- [ ] Showcase contributor work on Discord
- [ ] Second community call - demo basic prototype
- [ ] Vote on first major decision (parser library)
- [ ] Set up release automation

### Month 2-3: Community Governance
- [ ] Establish core team nomination process
- [ ] Document decision-making process in GOVERNANCE.md
- [ ] Create RFC template
- [ ] First community vote on syntax decision
- [ ] Monthly release cadence established
- [ ] 20+ active contributors

### Success Metrics (First 3 Months)
- 100+ GitHub stars
- 20+ contributors
- 50+ merged PRs
- 5+ active core team members
- Daily activity on Discord
- Working MVP released

---

## Appendix A: Community First Decisions

```
Input: "copy file.txt to backup/"
Tokens: ["copy", "file.txt", "to", "backup/"]
Intent: FileCopy
Args: { source: "file.txt", dest: "backup/" }
Canonical: Command::Copy { src: Path("file.txt"), dst: Path("backup/") }

Input: "cp file.txt backup/"
Tokens: ["cp", "file.txt", "backup/"]
Intent: FileCopy
Args: { source: "file.txt", dest: "backup/" }
Canonical: Command::Copy { src: Path("file.txt"), dst: Path("backup/") }

Input: "copy source=file.txt destination=backup/"
Tokens: ["copy", "source=file.txt", "destination=backup/"]
Intent: FileCopy
Args: { source: "file.txt", dest: "backup/" }
Canonical: Command::Copy { src: Path("file.txt"), dst: Path("backup/") }
```

---

## Appendix B: Parser Pseudo-Code

```rust
// High-level parser flow

fn parse(input: &str) -> Result<Command, ParseError> {
    // 1. Tokenize
    let tokens = tokenize(input)?;
    
    // 2. Detect syntax style
    let syntax = detect_syntax(&tokens);
    
    // 3. Identify intent
    let intent = analyze_intent(&tokens, syntax)?;
    
    // 4. Extract arguments
    let args = extract_args(&tokens, syntax, intent)?;
    
    // 5. Validate
    validate(&args, intent)?;
    
    // 6. Build canonical command
    Ok(build_command(intent, args))
}

fn detect_syntax(tokens: &[Token]) -> SyntaxStyle {
    // Check for named parameters (key=value)
    if tokens.iter().any(|t| t.contains('=')) {
        return SyntaxStyle::Named;
    }
    
    // Check for verbose keywords (to, from, with, etc.)
    if tokens.iter().any(|t| is_connector_word(t)) {
        return SyntaxStyle::Verbose;
    }
    
    // Default to terse
    SyntaxStyle::Terse
}

fn analyze_intent(tokens: &[Token], style: SyntaxStyle) -> Result<Intent> {
    let command = &tokens[0];
    
    // Try terse aliases first
    if let Some(intent) = TERSE_ALIASES.get(command) {
        return Ok(intent.clone());
    }
    
    // Try verbose keywords
    if let Some(intent) = VERBOSE_KEYWORDS.get(command) {
        return Ok(intent.clone());
    }
    
    // Unknown command
    Err(ParseError::UnknownCommand(command.clone()))
}
```

---

**End of Specification v0.1**

*This is a living document. Suggestions and improvements welcome!*
