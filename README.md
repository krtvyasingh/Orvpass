<div align="center">

# ⚡ ORVPASS CLI & TUI
### Ultra-Fast, Zero-Knowledge Terminal Password & Secrets Management Suite

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust_1.85+-orange.svg)](https://www.rust-lang.org)
[![Security](https://img.shields.io/badge/crypto-Argon2id_%2B_ChaCha20--Poly1305-emerald.svg)](SECURITY.md)
[![Release](https://img.shields.io/badge/release-v5.5.0-indigo.svg)](https://github.com/krtvyasingh/Orvpass/releases/tag/v5.5.0)
[![CI Tests](https://img.shields.io/badge/tests-100%25_passing-success.svg)](https://github.com/krtvyasingh/Orvpass/actions)
[![Startup](https://img.shields.io/badge/startup-%3C1ms-brightgreen.svg)](#-cryptographic-specifications--benchmarks)

<p align="center">
  <b>Sub-1ms Cold Start</b> • <b>Zero GUI Bloat</b> • <b>Interactive TokyoNight TUI</b> • <b>30+ Password Manager Formats</b> • <b>BIP-39 Mnemonic Backup</b>
</p>

</div>

---

## 📖 Table of Contents
- [💡 Why Terminal-Native?](#-why-terminal-native)
- [⚡ Daily User Convenience Suite](#-daily-user-convenience-suite)
- [🚀 Quick Installation](#-quick-installation)
- [🎮 Interactive TokyoNight TUI](#-interactive-tokyonight-tui)
- [🌐 Universal Import & Export Engine (30+ Formats)](#-universal-import--export-engine-30-formats)
- [🛠️ Comprehensive CLI Reference](#-comprehensive-cli-reference)
- [🔒 Cryptographic Specifications & Benchmarks](#-cryptographic-specifications--benchmarks)
- [🐚 Shell Integration & Auto-Aliases](#-shell-integration--auto-aliases)
- [📄 License & Security](#-license--security)

---

## 💡 Why Terminal-Native?

Orvpass v5.5.0 is a **pure, lightning-fast Rust CLI and interactive TUI**. All webview runtimes, Electron-style wrappers, and mobile build bloat have been eliminated in favor of UNIX purity:

- ⚡ **Sub-1ms Cold Startup**: Instantaneous command execution and clipboard piping with zero latency.
- 🛡️ **Zero Attack Surface**: No browser DOM vulnerabilities, no embedded JavaScript runtimes, and zero third-party telemetry.
- 🧼 **Deterministic Memory Safety**: Sensitive secrets exist in RAM only while in use and are wiped via Rust's `ZeroizeOnDrop` trait immediately upon destruction.
- 🔗 **UNIX Pipeline Ergonomics**: Seamlessly pipe secrets into processes (`orvpass run -- npm start`), stdout streams, or scripts without trailing line breaks.

---

## ⚡ Daily User Convenience Suite

Orvpass v5.5.0 includes high-frequency shortcuts designed for daily-driver developer workflows:

| Fast Command | Purpose |
| :--- | :--- |
| `orvpass cp <name>` | Instantly copy password to clipboard (auto-wipes in 15 seconds) |
| `orvpass cpu <name>` | Instantly copy username to clipboard |
| `orvpass cpt <name>` | Instantly copy live 6-digit 2FA TOTP token |
| `orvpass quick-add <title> [user] [pass]` | Single-line instant credential creation (auto-generates pass if omitted) |
| `orvpass open <name>` | Launch credential website directly in your default browser |
| `orvpass pick` | Instant interactive fuzzy selector to copy credentials in 1 keystroke |
| `orvpass health` | 0-100 Vault Health & Security Rating breakdown |
| `orvpass backup` / `restore` | Encrypted snapshot archives with SHA-256 integrity verification |
| `orvpass attach` / `extract` | Encrypt & attach private keys/files directly into credential items |
| `orvpass history <name>` | View credential revision timeline & integrity history |
| `orvpass fav <name>` / `orvpass favorites` | Star/unstar credential or view favorites list |
| `orvpass recent` | View 5 most recently accessed/modified secrets |
| `orvpass duplicate <name>` | Clone existing credential into `<name> (Copy)` |
| `orvpass rename <old> <new>` | Rename vault item with zero data loss |
| `orvpass mnemonic` | Generate 24-word BIP-39 emergency paper backup phrase |
| `orvpass calibrate` | Hardware auto-tune Argon2id parameters |
| `eval "$(orvpass init-shell)"` | Install lightning shell aliases (`op`, `opg`, `opc`, `opcu`, `opct`, `opl`, `opgen`) |

---

## 🚀 Quick Installation

### 🍺 Via Homebrew (macOS & Linux)
```bash
brew tap krtvyasingh/tap
brew install orvpass-cli
```

### ⚡ Via 1-Line Shell Installer
```bash
curl -fsSL https://raw.githubusercontent.com/krtvyasingh/Orvpass/main/install.sh | sh
```

### 🦀 Via Cargo
```bash
cargo install --git https://github.com/krtvyasingh/Orvpass.git orvpass-cli
```

---

## 🎮 Interactive TokyoNight TUI

Launch the full-featured terminal dashboard:
```bash
orvpass
# or
orvpass tui
```

```text
┌── 🛡️  ORVPASS v5.5.0 Enterprise [Argon2id+ChaCha20] ───────────────┬── 🔍 Press '/' to fuzzy search vault credentials... ──┐
│                                                                    │                                                        │
├────────────────────────────────────────────────────────────────────┴────────────────────────────────────────────────────────┤
│ [📦 All Items] | [🔑 Logins] | [📝 Secure Notes] | [💳 Credit Cards] | [⭐ Favorites]                                      │
├──────────────────────────────────────────┬──────────────────────────────────────────────────────────────────────────────────┤
│ ▶ 🔑  GitHub                             │ Title:     GitHub                                                                │
│       developer@orvpass.dev              │ Tags:      [production, infra, cloud]                                            │
│   📝  Server SSH Key                     │ Username:  developer@orvpass.dev   [u to copy]                                   │
│   💳  Corporate Card                     │ Password:  ••••••••••••••••        [c / Enter to copy, p to reveal]              │
│       •••• 4242                          │ 2FA TOTP:  482910 (24s left)       [t to copy]                                   │
│                                          │ URL:       https://github.com      [o to open browser]                           │
├──────────────────────────────────────────┴──────────────────────────────────────────────────────────────────────────────────┤
│  Ready  | [j/k] Nav | [Tab] Category | [/] Search | [Enter/c] Copy | [u] User | [t] TOTP | [o] Open | [s] Star | [?] Help   │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### ⌨️ TUI Keyboard Controls
| Key | Action |
| :--- | :--- |
| `j` / `k` or `↓` / `↑` | Navigate credentials list |
| `Tab` | Cycle category tabs (All Items, Logins, Secure Notes, Cards, Favorites) |
| `/` | Instant fuzzy search across titles, usernames, and tags |
| `Esc` / `Backspace` | Clear search query / Dismiss modal |
| `Enter` / `c` | Instant password copy to clipboard (auto-wipes after 15s) |
| `u` | Copy username to clipboard |
| `t` | Copy live RFC 6238 TOTP 2FA code |
| `o` | Launch item URL in default system browser |
| `s` | Toggle Star (Favorite) status |
| `p` | Mask / Reveal password in inspector pane |
| `?` | Toggle full keyboard shortcuts modal dialog |
| `q` | Quit dashboard |

---

## 🌐 Universal Import & Export Engine (30+ Formats)

Orvpass v5.5.0 supports bidirectional zero-data-loss migration for **every major password manager, web browser, 2FA authenticator, and cloud secret format**:

| Domain | Supported Formats | Import / Export Capabilities |
| :--- | :--- | :--- |
| **Password Managers** | **Bitwarden** (`.json`), **1Password** (`.csv`, `.1pux`, `.1pif`), **KeePass** (`.kdbx`, XML, CSV), **LastPass** (`.csv`), **NordPass** (`.csv`), **Proton Pass** (`.json`), **Dashlane** (`.json`, `.csv`), **Keeper** (`.json`, `.csv`), **Zoho Vault** (`.csv`), **RoboForm** (`.csv`), **Enpass** (`.json`), **SafeInCloud** (`.xml`, `.csv`), **Buttercup** (`.json`), **Passbolt** (`.csv`) | Bidirectional schema normalization, category mapping, notes, custom fields, tags |
| **Web Browsers** | **Google Chrome**, **Mozilla Firefox**, **Microsoft Edge**, **Brave**, **Apple Safari**, **Opera & Opera GX**, **Vivaldi**, **Arc Browser**, **Tor Browser**, **DuckDuckGo Browser** | Native CSV import & export, automated column header mapping, delimiter detection |
| **2FA Authenticators** | **Standard OTPAuth URIs** (`otpauth://totp/...`), **Aegis 2FA** (`.json`), **2FAS Authenticator** (`.json`, `.2fas`), **FreeOTP**, **Authy** | RFC 6238 secret extraction, SHA1/SHA256 support, token labeling |
| **Cloud & DevOps** | **Kubernetes Secrets** (`v1/Secret` YAML), **HashiCorp Vault** (KV v2 JSON), **AWS Secrets Manager**, **GCP Secret Manager**, **Azure Key Vault**, **Infisical**, **Doppler**, **Terraform tfvars** (`.json`), **.env / .env.vault** | Key-value normalization, env uppercase mapping, Base64 encoding |

```bash
# Import external vault from any file
orvpass import bitwarden_export.json
orvpass import nordpass.csv
orvpass import aegis_backup.json

# Export vault to any target format
orvpass export --format bitwarden vault_export.json
orvpass export --format k8s my_secrets.yaml
orvpass export --format tfvars terraform.tfvars.json
orvpass export --format dotenv .env
```

---

## 🛠️ Comprehensive CLI Reference

### ⚡ 1. Daily Convenience & Quick Access
```bash
orvpass cp <name>                     # Instant password copy
orvpass cpu <name>                    # Instant username copy
orvpass cpt <name>                    # Instant 2FA TOTP token copy
orvpass quick-add <name> [user] [pwd] # Fast inline credential add
orvpass open <name>                   # Launch credential URL in browser
orvpass pick                          # Interactive 1-keystroke credential selector
orvpass health                        # 0-100 Vault Health score & diagnostics
orvpass backup [--output path]        # Create encrypted snapshot backup
orvpass restore <file>                # Restore vault from backup
orvpass attach <item> <file>          # Encrypt and attach file to item
orvpass extract <item> <file>         # Decrypt and extract attached file
orvpass history <name>                # View credential revision timeline
orvpass favorites                     # Filter starred items
orvpass fav <name>                    # Star or unstar item
orvpass recent                        # List top 5 recently accessed credentials
orvpass duplicate <name>              # Clone credential
orvpass rename <old> <new>            # Rename credential
orvpass notes                         # List all secure notes
orvpass cards                         # List all payment cards
orvpass mnemonic                      # BIP-39 24-word paper backup phrase
orvpass calibrate                     # Argon2id hardware benchmark tuner
orvpass which-shell                   # Shell detection & configuration guide
```

### 🔐 2. Core Vault Management
```bash
orvpass list [--json] [-c category]   # View vault contents (beautiful Unicode table or JSON)
orvpass get <name> [-p] [-u] [-t]     # Retrieve specific credentials
orvpass add [name]                    # Full interactive wizard
orvpass remove <name>                 # Delete vault item
orvpass search <query>                # Fuzzy search across vault
orvpass totp <name> [-w]              # Watch live 2FA countdown ticker
orvpass generate [-l 24] [-d] [--pin] # Generate password, PIN, or Diceware passphrase
orvpass status                        # Vault cryptographic diagnostics
```

### 💻 3. DevOps & Environment Secret Injection
```bash
orvpass run -- <command>              # Inject secrets into child process RAM (zero disk exposure)
orvpass dotenv [--file .env]          # Sync .env files with encrypted vault
orvpass docker <service>              # Generate in-memory Docker Compose secrets
orvpass k8s [-n namespace]            # Output Kubernetes Secret manifests
orvpass tf                            # Generate Terraform data source blocks
orvpass aws [--profile staging]       # Inject temporary AWS STS tokens
orvpass pipe <name>                   # Clean stdout stream for UNIX pipelines
```

### 🛡️ 4. Cryptographic Security & Hardware Keys
```bash
orvpass sss --split / --recover       # Shamir's Secret Sharing (3-of-5 shard recovery)
orvpass yubikey                       # FIDO2 / YubiKey HMAC-SHA1 hardware auth
orvpass secure-enclave                # Apple Silicon Secure Enclave & TPM 2.0 binding
orvpass pqc-kem                       # Post-Quantum ML-KEM-768 (Kyber) hybrid encryption
orvpass duress-wipe                   # Emergency multi-pass memory & disk purge
orvpass age                           # Age recipient plugin integration
orvpass dead-man-switch [--arm]       # Automated emergency shard release timer
```

### 🔍 5. Security Audits & Threat Intelligence
```bash
orvpass audit [--json]                # Watchdog audit (weak, reused, expired secrets)
orvpass pwned-check                   # Offline k-anonymity breach check (HIBP)
orvpass strength <password>           # Zxcvbn-style entropy and crack-time analyzer
orvpass cert-expiry                   # Inspect TLS certs and SSH key expirations
orvpass qr <name>                     # Display ANSI terminal QR code for mobile scan
orvpass leak-detector                 # Install Git pre-commit secret leak hook
orvpass audit-export [--format md]    # Export SOC2 / ISO-27001 compliance audit log
```

### 👥 6. Team Governance & Sysadmin
```bash
orvpass org-vault <action>            # Multi-tenant team vault partition
orvpass p2p-sync --peer <ip>          # Peer-to-peer LAN / Tailscale sync
orvpass webhook <url>                 # Dispatch HMAC-signed mutation events
orvpass ssh <list|agent>              # Native SSH agent socket integration
orvpass orvsend <text> [--expires 24] # Ephemeral end-to-end encrypted secret drop
```

---

## 🔒 Cryptographic Specifications & Benchmarks

| Cryptographic Component | Algorithm / Specification | Benchmark Metric |
| :--- | :--- | :--- |
| **Key Derivation Function** | Argon2id ($m=64\text{ MB}, t=3, p=4$) | $\sim 104\text{ ms}$ (SIMD accelerated) |
| **Authenticated Encryption** | ChaCha20-Poly1305 ($256\text{-bit}$ key, $96\text{-bit}$ nonce) | $386.83\text{ MB/s}$ throughput |
| **Post-Quantum Key Exchange** | ML-KEM-768 (NIST FIPS 203) + X25519 Hybrid | $< 1\text{ ms}$ key encapsulation |
| **Secret Sharding** | Shamir's Secret Sharing ($k=3, n=5$, GF($2^8$)) | Instant polynomial reconstruction |
| **Paper Backup Recovery** | BIP-39 Standard 24-Word Mnemonic | Standardized deterministic entropy |
| **Timing-Safe Comparison** | Constant-time slice comparison (`subtle`) | Resistance to side-channel analysis |
| **RAM Sanitization** | `ZeroizeOnDrop` memory overwrite | Zero memory retention on exit |

---

## 🐚 Shell Integration & Auto-Aliases

Supercharge your daily terminal workflow by activating native shell shortcuts:

```bash
# Add to ~/.zshrc, ~/.bashrc, or config.fish:
eval "$(orvpass init-shell)"
```

Installed aliases:
- `op` -> `orvpass`
- `opg` -> `orvpass get`
- `opc` -> `orvpass cp` (copy password)
- `opcu` -> `orvpass cpu` (copy username)
- `opct` -> `orvpass cpt` (copy 2FA TOTP)
- `opl` -> `orvpass list`
- `opgen` -> `orvpass generate`

Generate autocomplete scripts:
```bash
orvpass completions zsh > ~/.zfunc/_orvpass
orvpass completions bash > /etc/bash_completion.d/orvpass
orvpass completions fish > ~/.config/fish/completions/orvpass.fish
```

---

## 📄 License & Security

- **License**: Apache License 2.0. Copyright (c) 2026 krtvyasingh.
- **Security Policy**: See [SECURITY.md](SECURITY.md) for vulnerability disclosure procedures.
- **Verification**: All 35+ test suites pass in continuous integration with zero warnings.
