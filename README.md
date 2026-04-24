# BlueCore Browser

BlueCore is a next-generation, modular browser project built with Rust and React.

## ⚖️ License
This project is licensed under the **Apache License 2.0**.

## 🏢 Ownership
BlueCore is a part of the **Simon Project**.

## 🚀 Getting Started

### Prerequisites
- **Node.js**: (Current LTS recommended)
- **Rust**: (Latest stable version)
- **System Dependencies (Linux)**:
  
  **Debian/Ubuntu**:
  ```bash
  sudo apt-get update
  sudo apt-get install -y libsoup2.4-dev libjavascriptcoregtk-4.0-dev libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

  **Arch Linux**:
  ```bash
  # 1. Install official dependencies
  sudo pacman -S --needed base-devel libsoup webkit2gtk-4.1 pkgconf
  
  # 2. Fast Fix (Symlink 4.1 to 4.0) - Avoids 4+ hour AUR build
  # (Simulates the expected 4.0 environment using 4.1)
  sudo ln -sf /usr/lib/pkgconfig/webkit2gtk-4.1.pc /usr/lib/pkgconfig/webkit2gtk-4.0.pc
  sudo ln -sf /usr/lib/pkgconfig/javascriptcoregtk-4.1.pc /usr/lib/pkgconfig/javascriptcoregtk-4.0.pc
  sudo ln -sf /usr/lib/libwebkit2gtk-4.1.so /usr/lib/libwebkit2gtk-4.0.so
  sudo ln -sf /usr/lib/libjavascriptcoregtk-4.1.so /usr/lib/libjavascriptcoregtk-4.0.so
  ```

### How to Run

1. **Install Dependencies**:
   ```bash
   npm install
   ```

2. **Run in Development Mode**:
   ```bash
   npm run tauri dev
   ```

3. **Build for Production**:
   ```bash
   npm run tauri build
   ```

## 🧪 Experimental: Chromium Engine

BlueCore includes an optional, dormant Chromium engine. To activate it:

1. **Activation via Config**:
   Set `enabled: true` in `config/engines.json`.

2. **Activation via Trigger**:
   Alternatively, ensure `docs/enable_chromium.md` exists and contains:
   `TOKEN: BLC-ENABLE-CHROME-V1`

3. **Switching Engines**:
   Use the UI or the Internal API:
   ```typescript
   BlueCoreAPI.engine.switch('chromium');
   ```

---
© 2026 Simon Project. All rights reserved.
