# 🦀 Game of Life — Rust

Conway's Game of Life running in the terminal, built in Rust in one day as part of Epitech's **P³** (learning-focused punishment).

---

## 📋 Rules

The simulation runs on 2 simple rules:
- A **living cell** survives if it has **2 or 3** living neighbors
- A **dead cell** comes back to life if it has **exactly 3** living neighbors

Everything else dies. From these 2 rules, complex patterns emerge on their own.

---

## ⚙️ Requirements

- macOS / Linux
- [Rust](https://www.rust-lang.org/tools/install) (via `rustup`)

---

## 🚀 Installation & Run

**1. Install Rust** (if not already installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Clone the repo**
```bash
git clone https://github.com/Slacknsss/P-3_Simon_GameOfLife.git
cd P-3_Simon_GameOfLife
```

**3. Run**
```bash
cargo run
```

That's it. No extra setup needed.

---

## 🧠 What I learned

- Rust basics: types, ownership, mutability
- 2D Vec manipulation
- How to structure a simple game loop
- Terminal rendering with ANSI escape codes

---

*Built at Epitech Paris — P³ project*
