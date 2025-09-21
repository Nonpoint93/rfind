# 📁 rfind: A Robust, Auditable `find` in Rust

## 📚 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Why Rust?](#why-rust)

---

## 🧭 Overview

**rfind** is a modern alternative to the classic Unix `find`, written entirely in Rust. It’s designed for developers, sysadmins, and security professionals who need precise, scriptable, and auditable file discovery.

Unlike traditional tools, `rfind` offers semantic flags for privilege auditing, ergonomic CLI design, and strong guarantees around correctness and portability.

This project is part of my personal transition from Java backend development to systems programming in Rust, focusing on clarity, safety, and performance.

---

## ✨ Features

- ⚡ **Fast Recursive Search** — Efficient traversal with robust error handling
- 🧩 **Type Filtering** — Search by file type (`file`, `dir`)
- 🔐 **Permission Matching** — Exact or masked octal permissions (`--perm 755`, `--perm /4000`)
- 🧠 **Privilege Flags** — Detect SUID, SGID, executable-by-others, or root-owned files
- 🔍 **Name Matching** — Glob patterns (`--name '*.sh'`) with optional case sensitivity
- 📣 **Verbose Mode** — Show permission errors and inaccessible paths
- 🧪 **Unit-Tested** — Core filters covered by Rust unit tests

---

## 📦 Installation

```bash
git clone https://github.com/tuusuario/rfind
cd rfind
cargo build --release
```

Binary will be available at ./target/release/rfind.

🚀 Usage

rfind [OPTIONS]

Options:
  -p, --path <PATH>             Directory to start searching from [default: .]
  -t, --types <TYPES>           Filter by type: file, dir [default: file dir]
  -n, --name <NAME>             Glob pattern to match file/directory names
  -c, --case-sensitive          Enable case-sensitive name matching [default: true]
      --perm <PERM>             Filter by permissions (octal or masked with `/`)
      --suid                    Match files with SUID bit
      --sgid                    Match files with SGID bit
      --exec-other              Match files executable by others
      --owned-by-root           Match files owned by UID 0
  -v, --verbose                 Show errors when directories can't be read
  -h, --help                    Print help
  -V, --version                 Print version

🧪 Examples

🔍 Find all .sh scripts executable by others

rfind --name '*.sh' --exec-other

🔐 Find SUID binaries owned by root

rfind --suid --owned-by-root --types file --perm /4000

📁 Find directories named config under /etc

rfind --path /etc --types dir --name config

🧾 Verbose scan of /usr for files with permission 644

rfind --path /usr --types file --perm 644 --verbose


🦀 Why Rust?

After 9 years of backend development in Java, I wanted to build tools that are:

    Safe by default — no nulls, no data races

    Portable — compile once, run anywhere

    Ergonomic — expressive CLI with clap, strong typing, and clear error handling

    Auditable — ideal for security tooling and filesystem analysis

Rust allows me to write code that is fast, correct, and maintainable — and rfind is my way of demonstrating that shift.