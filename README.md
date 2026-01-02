# 📂 tidy — Rust File Organizer

A simple, fast, and modular CLI tool written in Rust to organize files into folders based on their file extensions.

## 📦 Project Structure

```text
tidy/
├── src/
│   ├── main.rs             # CLI entry point (uses clap)
│   ├── lib.rs              # Core library interface
│   ├── organizer/          # Folder organization logic
│   │   ├── mod.rs          # organize_folder function
│   │   ├── file_types.rs   # File extension mapping
│   │   └── mover.rs        # File moving helpers
│   └── utils.rs            # (Optional) Utility helpers
├── testfolder/             # A sample directory for testing
├── Cargo.toml              # Rust project manifest
└── README.md
```

---

## 🚀 Features

- Automatically organizes files into folders based on their extensions.
- Accepts a **target directory as an argument.**
- Defaults to the **current working directory** if no argument is provided.
- Cross-platform support (Linux, Windows, Mac).

---

## 🔽 Download (Prebuilt Binaries)

👉 [Download the latest release](https://github.com/Ik-cyber/tidy/releases/tag/v0.1.0).

### Available for:

    ✅ Linux: tidy-v0.1.0-linux.zip

## 🛠️ Build from Source (Optional)

### 🔧 On Linux / macOS

```bash
git clone https://github.com/Ik-cyber/tidy.git
cd tidy
cargo build --release
sudo cp target/release/tidy /usr/local/bin/
```

## 🔧 On Windows (PowerShell)

```bash
git clone https://github.com/Ik-cyber/tidy.git
cd tidy
cargo build --release
.\target\release\tidy.exe .\testfolder
```

# ⚙️ Usage

## Run with a Target Directory:

```bash
tidy ./testfolder
```

# ✔️ This will organize files inside the testfolder.

## Run without Arguments (Organizes Current Directory):

```bash
tidy
```

# 📝 Notes

- Organizes files into folders like Images, Documents, TextFiles, or Other.

- Automatically creates target folders if they don’t exist.

- Ignores directories — only organizes files.

- Easily test with the testfolder/ directory.- Supports nested directories.

