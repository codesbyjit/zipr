### 🦀 zipr

> ⚡ A blazing-fast, lossless file compressor in Rust — smaller, quicker, smarter than zip.

---

### 📘 Overview

`zipr` is a **modern file compressor** built with **Rust**, offering **superior speed** and **compression ratio** through algorithms like **Huffman**, **LZW**, and **BWT** — powered by Rust’s **parallelism** and **bit-level precision**.

---

### ⚙️ Features

* 🔒 Lossless, bit-perfect compression
* ⚡ Multi-threaded for high performance
* 📦 Multiple algorithms (Huffman / LZW / BWT)
* 📊 Built-in benchmarking
* 🧠 Optimized buffering
* 🦀 100% Rust, safe & fast

---

### 🚀 Install

```bash
cargo install zipr
# or build from source
git clone https://github.com/codesbyjit/zipr.git
cd zipr && cargo build --release
```
---
```
🦀 zipr — Setup Script

Usage:
  ./zipr-setup.sh [COMMAND]

Commands:
  install     Install zipr (no sudo required)
  update      Update to the latest version
  uninstall   Remove zipr completely
  help        Show this help message

Examples:
  ./zipr-setup.sh install
  ./zipr-setup.sh update
  ./zipr-setup.sh uninstall
```

---

#### 🔁 Update Shortcut

```bash
alias zipr-update='bash <(curl -s https://raw.githubusercontent.com/codesbyjit/zipr/main/update.sh)'
```
---

### 🧩 Usage

```bash
zipr compress input.txt output.zipr
zipr extract output.zipr restored.txt
zipr bench input.txt
```

---

### 🧠 Tech Stack

Rust · bitvec · rayon · clap · serde

---

### 🧪 Roadmap

* [ ] Smart algorithm selection
* [ ] Streaming mode (stdin/stdout)
* [ ] GUI (Tauri)


---

### 🪶 License

MIT © 2025 codesbyjit
