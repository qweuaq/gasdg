# Am-Am VPN

Modern desktop VPN client built with **Tauri + React + Rust**, powered by **Xray-core**.

## Architecture

```
┌──────────────────────────────────┐
│   UI Layer (React + TypeScript)  │
│  components / hooks / services   │
├──────────────────────────────────┤
│         Tauri IPC Bridge         │
├──────────────────────────────────┤
│    Backend Layer (Rust)          │
│  commands / state / storage      │
├──────────────────────────────────┤
│    Core Engine (Xray-core)       │
│  config / process / proxy        │
└──────────────────────────────────┘
```

## 📥 Download Installers

Pre-built installers are available from the [Releases](../../releases) page:

| Platform | Format | File |
|----------|--------|------|
| **Windows** | EXE installer (NSIS) | `Am-Am-VPN_x.x.x_x64-setup.exe` |
| **Windows** | MSI installer | `Am-Am-VPN_x.x.x_x64_en-US.msi` |
| **macOS** (Apple Silicon) | DMG | `Am-Am-VPN_x.x.x_aarch64.dmg` |
| **macOS** (Intel) | DMG | `Am-Am-VPN_x.x.x_x64.dmg` |
| **Linux** | AppImage | `Am-Am-VPN_x.x.x_amd64.AppImage` |
| **Linux** | deb package | `Am-Am-VPN_x.x.x_amd64.deb` |

> No `npm install` or build tools required — just download and run!

## 🚀 Getting Started (Development)

### Prerequisites

- **Node.js** 18+ — [download](https://nodejs.org/)
- **Rust** 1.70+ — [install](https://rustup.rs/)
- **Tauri prerequisites** — [platform-specific setup](https://tauri.app/v1/guides/getting-started/prerequisites)

### Development

```bash
cd am-am-vpn
npm install
npm run tauri dev
```

### Build Installers Locally

```bash
cd am-am-vpn
npm install
npm run tauri build
```

Installers will be generated in `am-am-vpn/src-tauri/target/release/bundle/`:
- **Windows**: `bundle/nsis/*.exe` and `bundle/msi/*.msi`
- **macOS**: `bundle/dmg/*.dmg`
- **Linux**: `bundle/appimage/*.AppImage` and `bundle/deb/*.deb`

### CI/CD

The project includes a GitHub Actions workflow (`.github/workflows/build.yml`) that automatically builds installers for all platforms on every push to `main` or when a version tag (`v*`) is pushed. Tagged releases automatically create a GitHub Release with all installer artifacts.

## 📱 Features

- **Subscription import** — paste a subscription URL and fetch server list
- **Protocol support** — VMess, VLESS, Trojan, Shadowsocks
- **Base64 / JSON subscription parsing**
- **Server list** with latency testing
- **One-click connect / disconnect** with animated status
- **Auto-select fastest server**
- **System proxy** (HTTP + SOCKS5) configuration
- **TUN mode** support
- **DNS routing** and leak protection
- **Encrypted config storage** (AES-256-GCM)
- **Real-time logs**
- **Auto-subscription refresh**

## 📁 Project Structure

```
am-am-vpn/
├── index.html                    — Vite entry point
├── package.json                  — Frontend dependencies
├── vite.config.ts                — Vite configuration
├── tsconfig.json                 — TypeScript config
├── src/                          — React UI
│   ├── main.tsx                  — App entry
│   ├── App.tsx                   — Root component
│   ├── App.css                   — Styles
│   ├── components/
│   │   ├── SubscriptionInput.tsx — URL input
│   │   ├── ServerList.tsx        — Server list with latency
│   │   ├── ConnectionButton.tsx  — Connect/disconnect
│   │   ├── StatusBar.tsx         — Connection status
│   │   ├── LogViewer.tsx         — Log panel
│   │   ├── SettingsPanel.tsx     — Settings editor
│   │   └── Toast.tsx             — Toast notifications
│   ├── hooks/
│   │   ├── useConnection.ts      — Connection state hook
│   │   └── useServers.ts         — Server/subscription hook
│   ├── services/
│   │   └── api.ts                — Tauri invoke wrappers
│   └── types/
│       └── index.ts              — Shared TypeScript types
└── src-tauri/                    — Rust backend
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── build.rs
    └── src/
        ├── main.rs               — Entry point
        ├── lib.rs                — Tauri app setup
        ├── state.rs              — Shared app state
        ├── errors.rs             — Error types (thiserror)
        ├── commands/             — Tauri IPC handlers
        │   ├── connection.rs     — connect / disconnect / failover
        │   ├── subscription.rs   — add / refresh / remove
        │   └── server.rs         — list / latency test
        ├── core/                 — Xray-core integration
        │   ├── config.rs         — JSON config generation
        │   ├── download.rs       — Auto-download from GitHub
        │   ├── process.rs        — Process lifecycle
        │   └── xray.rs           — High-level engine
        ├── subscription/         — URL parsing
        │   ├── parser.rs         — Fetch & parse
        │   └── protocols.rs      — VMess/VLESS/Trojan/SS
        ├── proxy/
        │   └── system_proxy.rs   — OS proxy settings
        ├── storage/
        │   └── encrypted.rs      — AES-256-GCM storage
        └── models/
            └── server.rs         — Data structures
```

## 🛠 Technologies

| Layer    | Technology        |
| -------- | ----------------- |
| UI       | React + TypeScript |
| Desktop  | Tauri v2          |
| Backend  | Rust              |
| Core     | Xray-core         |
| Build    | Vite              |

## 🔒 Security

- Core process runs sandboxed with `kill_on_drop`
- Configuration data encrypted with AES-256-GCM
- Encryption key derived per-installation from hostname + data directory
- Subscription URLs stored in encrypted storage
- CSP headers configured in Tauri
- Context isolation between UI and backend

## 🔌 Extensibility

The modular architecture allows:
- Adding **sing-box** as an alternative core engine
- Plugin system via the command interface
- External API for automation
