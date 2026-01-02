## Project Structure

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
