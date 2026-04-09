
# Dioxus Tutorial

## Usage

```bash
# Create project
dx new NAME
# 1. Choose Template: Bare-Bones
# 2. Choose Fullstack: false
# 3. Choose Router: false
# 4. Choose Tailwindcss: false
# 5. Chooose Prompts for LLMs: false
# 6. Choose Platform Default: Desktop

# Run project
cd NAME && dx serve
# NOTE: DON'T use `cargo run`

# Check project
cd NAME && cargo check
```

## Insight

- Methods:

```plaintext
Method 1 — Inline JS Rendering
Method 2 — JSON Object Serialization
Method 3 — Flat Buffer String
Method 4 — Base64 Binary Buffer
Method 5 — Uint8Array Literal (pseudo zero-copy)
Tier 2 — Zero-copy / Low-overhead transport
  6. WASM Shared Memory
  7. Binary IPC (Tauri)
Tier 3 — GPU Rendering
  8. WebGL
  9. WebGPU
  10. wgpu (Rust native GPU)
```
