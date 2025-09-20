# 📁 rfind: A Fast, Modern `find` in Rust

## 📚 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Usage](#usage)
- [Examples](#examples)

---

## 🧭 Overview

**rfind** is a high-performance command-line tool for locating files and directories on your system. Built from the ground up in Rust, it leverages the language's strengths—speed, memory safety, and concurrency—to outperform traditional alternatives.

By using low-level APIs and concurrent processing, rfind avoids the overhead of external binaries and fully utilizes modern multi-core processors to traverse your filesystem with exceptional speed.

This project showcases Rust's power in building robust, native tools for real-world use cases.

---

## ✨ Features

- ⚡ **High-Speed Search**: Quickly locate files and directories by name.
- 🧩 **Type Filtering**: Limit your search to specific file types (`file` or `dir`).
- 🔐 **Permission Filtering**: Search for files matching specific permission sets.
- 🔍 **Case-Insensitive Search**: Perform searches regardless of character case.

---

## 🚀 Usage

A command-line tool for finding files and directories in a filesystem

```bash

Usage: rfind [OPTIONS]

Options:
  -p, --path <PATH>
          The path to the directory to start searching from. Defaults to the current directory
          
          [default: .]

  -t, --types <TYPES>
          Filter the search by file type. Can be used multiple times to search for different types

          Possible values:
          - file: Search for files
          - dir:  Search for directories
          
          [default: dir file]

  -n, --name <NAME>
          The name of the file or directory to search for

  -c, --case-sensitive
          Perform a case-sensitive search (default: true)

      --perm <PERM>
          Filter the search by file permissions (octal format)

  -v, --verbose
          Show error messages when directories can't be read

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

```bash
./target/debug/rfind --name output --path ./target/debug

```

Output:

Agrs: Path: "./target/debug" Name: output Types: {Dir, File} Perm: None
./target/debug/build/output
./target/debug/build/output/output
./target/debug/build/proc-macro2-c4fca423565ace57/output

```bash
./target/debug/rfind --name output --path ./target/debug

```

Output:

Agrs: Path: "./target/debug" Name: output Types: {Dir, File} Perm: None
./target/debug/build/output
./target/debug/build/output/output
./target/debug/build/proc-macro2-c4fca423565ace57/output


```bash
./target/debug/rfind --name ssltransport* --path / --types file --perm 644
```
Output:

Agrs: Path: "/" Name: ssltransport* Types: {File} Perm: Some(420) Case-Sensitive: true
/usr/lib/python3/dist-packages/urllib3/util/ssltransport.py
/usr/lib/python3/dist-packages/urllib3/util/__pycache__/ssltransport.cpython-312.pyc
[ ERROR ] Failed to read directory: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
