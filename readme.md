# 🐚 VSH – Vic's Shell

**The beginner-friendly, power-user-capable shell.**  
VSH is a modern Linux shell built to be as intuitive as it is extensible.  
Designed for people who know absolutely nothing about the terminal—but powerful enough for those who do.

---

## ✨ Mission

> To make the Linux shell accessible, safe, and enjoyable for everyone—without sacrificing power.

VSH is the bridge between new users and seasoned professionals. It speaks your language, teaches as you go, and grows with you. Whether you’re listing files or building CLI pipelines, VSH makes every command feel natural.

---

## 🧠 Philosophy

- **Human-first, not command-first** – Natural language commands just work.
- **Learning by doing** – VSH teaches you as you use it.
- **Safe by default** – Prevents destructive commands unless you really mean it.
- **Hackable by design** – Plugins, aliases, themes, and full control.
- **Two modes, one shell** – Choose natural or classic syntax.

---

## 🗣️ Syntax Overview

VSH supports **two styles of syntax**:

- **Natural Mode** _(beginner-friendly, default)_
- **Classic Mode** _(POSIX-style, power user)_

### 🌿 Natural Examples

```bash
list files in /home
copy file.txt to backup/
search "error" in logs/*.log
delete temp/* with confirmation
```

### ⚙️ Classic Examples

```bash
ls /home
cp file.txt backup/
grep "error" logs/*.log
rm -i temp/*
```

### 🔁 Hybrid-Friendly

You can freely mix both:

```bash
list files in . | grep ".md"
```

---

## 🚀 Key Features

- ✅ Natural language command parsing
- ✅ Classic shell support
- ✅ Plugin system (Rust, Python, Shell)
- ✅ Smart autocompletion + suggestions
- ✅ Explains commands before or after execution
- ✅ Safety prompts for dangerous commands
- ✅ Built-in tutorial: `vsh tutor`
- ✅ Custom themes and prompts (`.vshrc`)

---

## 🗂 Directory Structure

```
vsh/
├── src/            # Core Rust code
├── plugins/        # First-party + community plugins
├── themes/         # Prompt themes (TOML)
├── tests/          # Unit + integration tests
├── examples/       # Demo scripts and usage
├── docs/           # GitHub Pages content
├── .vshrc          # Default config file
├── Cargo.toml      # Rust project config
├── README.md       # This file
```

---

## 🧭 Roadmap

### ✅ v0.1 - MVP Shell

- [x] Natural + classic parser
- [x] Core command execution (`list`, `copy`, `delete`, etc.)
- [x] History + variables
- [x] Config system (`.vshrc`)
- [x] Theme support + prompt rendering

### 🔜 v0.2 - Learning Mode

- [ ] `explain <cmd>` for command feedback
- [ ] `why did this fail?` for smart stderr parsing
- [ ] Interactive tutorial system

### 🔮 Future

- [ ] Fuzzy autocomplete
- [ ] Full plugin manager (`vsh plugin install`)
- [ ] WASM/GUI demo shell
- [ ] Git-aware prompt
- [ ] Shell macros + command recorder
- [ ] Shell time travel (`history -replay`)

---

## 🙌 Contributing

Want to help build the next great Linux shell? We welcome contributions of all kinds:

- Bug reports & feature ideas → [Issues](https://github.com/yourname/vsh/issues)
- Code contributions → Fork, commit, and open a PR
- Plugin authors → Create plugins in `plugins/`
- Theme designers → Submit `.toml` files to `themes/`
- Docs & tutorials → Help others learn VSH faster

See [`CONTRIBUTING.md`](docs/contributing.md) for details.

---

## 📝 License

[MIT License](LICENSE)

---

## 🌐 Links

- 📘 [Full Documentation](https://yourusername.github.io/vsh)
- 📦 [Crates.io](https://crates.io/crates/vsh) _(coming soon)_
- 🐦 [@vicdev](https://twitter.com/yourhandle) for updates

---

> _“If you can talk, you can shell.” — VSH_
