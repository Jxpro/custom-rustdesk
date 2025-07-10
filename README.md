# RustDesk Custom ID Tool

## 📖 Introduction

>   Note that if a certain ID is invalid and reset, it may be that the ID is too short or occupied, please try a different ID.

中文文档: [README_CN.md](https://github.com/Jxpro/custom-rustdesk/blob/main/README_CN.md)

This project aims to provide a custom ID generation function for `RustDesk`, making it easier for users to remember and manage devices. The primary function is to simulate the official encryption algorithm, encrypting user-entered custom IDs and outputting the encrypted ID. By replacing the `enc_id` field in the configuration file with this output, users can complete the setup.

For MacOS, the configuration file is located at:

-   `~/Library/Preferences/com.carriez.RustDesk/RustDesk.toml`

For Windows, the configuration file is located at:

-   `C:\Users\username\AppData\Roaming\RustDesk\config\RustDesk.toml`

When running in service mode in Windows, specified by `--service` :

-   `C:\Windows\ServiceProfiles\LocalService\AppData\Roaming\RustDesk\config\RustDesk.toml`

## ✨ Features
-   🔒 Generate encrypted custom ID
-   🔓 Decrypt and validate encrypted ID
-   🔑 Use UUID as the encryption and decryption key
-   💬 Interactive mode for easy operation
-   📚 Comprehensive help system
-   🌍 Multi-language support (English/Chinese)
-   ⌨️ Command-line interface with detailed parameter descriptions

## 🚀 Installation & Getting Started

### 📦 Option 1: Download Pre-built Binaries (Recommended)

The easiest way to get started is to download the pre-built binaries from our releases page:

**📥 [Download Latest Release](https://github.com/Jxpro/custom-rustdesk/releases)**

Available platforms:
- **Linux**: `custom-rustdesk-linux-x86_64-gnu`, `custom-rustdesk-linux-aarch64-gnu`
- **Linux (MUSL)**: `custom-rustdesk-linux-x86_64-musl`, `custom-rustdesk-linux-aarch64-musl`
- **Windows**: `custom-rustdesk-windows-x86_64.exe`, `custom-rustdesk-windows-aarch64.exe`
- **macOS**: `custom-rustdesk-macos-universal` (supports both Intel and Apple Silicon)

#### Quick Start with Pre-built Binary:

1. Download the appropriate binary for your platform
2. Make it executable (Linux/macOS): `chmod +x custom-rustdesk-*`
3. Run directly:
   ```bash
   # Interactive mode
   ./custom-rustdesk-macos-universal
   
   # Command line mode
   ./custom-rustdesk-macos-universal --id 123456 --uuid your-uuid-here
   ```

### 🔨 Option 2: Build from Source

If you prefer to build from source or need to modify the code:

#### 📋 Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- Git

#### 🛠️ Build Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Jxpro/custom-rustdesk.git
   cd custom-rustdesk
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the built binary:**
   ```bash
   # Interactive mode
   cargo run --release
   
   # Or run the built binary directly
   ./target/release/custom-rustdesk
   ```

#### 🧪 Development Build
For development purposes, you can run directly with cargo:
```bash
cargo run
```

## 📘 Usage

### 💬 Interactive Mode

Run without parameters to enter interactive mode:

```bash
# Using pre-built binary
./custom-rustdesk-macos-universal

# Or from source
cargo run
```

The interactive menu provides:
1. **Encrypt Mode**: Generate encrypted ID from custom ID
2. **Decrypt Mode**: Verify and decrypt encrypted ID
3. **View Help**: Display comprehensive help information
4. **Exit**: Quit the application

### ⌨️ Command Line Mode

The tool supports both command-line and interactive modes. For command-line usage:

```bash
# Using pre-built binary:
# Generate encrypted ID
./custom-rustdesk-macos-universal --id <CustomID> --uuid <MachineUUID>

# Verify encrypted ID
./custom-rustdesk-macos-universal --eid <EncryptedID> --uuid <MachineUUID>

# Set language (en/zh)
./custom-rustdesk-macos-universal --lang zh

# Show help
./custom-rustdesk-macos-universal --help

# From source:
# Generate encrypted ID
cargo run -- --id <CustomID> --uuid <MachineUUID>

# Verify encrypted ID
cargo run -- --eid <EncryptedID> --uuid <MachineUUID>

# Set language (en/zh)
cargo run -- --lang zh

# Show help
cargo run -- --help
```

#### 📝 Command Line Parameters

- `-i, --id <ID>`: Custom ID to encrypt
- `-e, --eid <EID>`: Encrypted ID to decrypt
- `-u, --uuid <UUID>`: UUID for encryption/decryption
- `-l, --lang <LANG>`: Set the language (en/zh) [default: en]
- `-h, --help`: Show detailed help information

### 🌍 Language Support

The tool supports both English and Chinese:
- Default language is English
- Use `--lang zh` for Chinese interface
- Language setting affects all output including help text and error messages

### 🔍 Getting UUID

>   You can get more information at the official tool [machine-uid](https://github.com/rustdesk-org/machine-uid)

1.  **Windows:**

    -   Press `Win + R` to open the Run dialog box.
    -   Type `regedit` and press Enter to open the Registry Editor.
    -   Navigate to `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography`.
    -   Copy the `MachineGuid` value as the `uuid` parameter.

2.  **MacOS:**

    -   Open Terminal.
    -   Enter the following command: `ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID`
    -   Copy the UUID from the output as the `uuid` parameter.

### ✅ Validate UUID

1.  Download the pre-built binary or clone the code locally.
2.  Find the `enc_id` field in the respective configuration file.
3.  Run the validation command:
    ```bash
    # Using pre-built binary
    ./custom-rustdesk-macos-universal --eid $enc_id --uuid $uuid
    
    # Or from source
    cargo run -- --eid $enc_id --uuid $uuid
    ```
4.  The program will output the decrypted ID, compare it with the current ID to check consistency.

### 🎯 Customize ID

1.  Download the pre-built binary or clone the code locally.
2.  Run the encryption command:
    ```bash
    # Using pre-built binary
    ./custom-rustdesk-macos-universal --id $custom_id --uuid $uuid
    
    # Or from source
    cargo run -- --id $custom_id --uuid $uuid
    ```
3.  The program will output the encrypted ID, copy and replace it in the `enc_id` field of the configuration file.

Example of program execution:

```bash
# Using pre-built binary
./custom-rustdesk-macos-universal --id 123456 --uuid 12345678-1234-1234-1234-123456789012

# From source
cargo run -- --id 123456 --uuid 12345678-1234-1234-1234-123456789012
```

## 🔐 Encryption Process

This program uses the `crypto::secretbox` module in the `sodiumoxide` library for symmetric encryption. The encryption key comes from the provided UUID string.

1.  Convert the custom ID string into a byte array.
2.  Convert the UUID string into a byte array and adjust its size to match the key length requirement.
3.  Use the `sodiumoxide::crypto::secretbox` module to create a key and `nonce`.
4.  Choose encryption or decryption operation based on the `encrypt` parameter.
5.  Use the `secretbox::seal` or `secretbox::open` function for encryption or decryption.
6.  Convert the encrypted byte array to a `base64` encoded string and output it to the console.

## 🤝 Contributing

You are welcome to contribute to this project! You can participate in the following ways:

-   Submit code patches or issue reports
-   Provide feedback and suggestions
-   Help promote the project

## 📧 Contact

If you have any questions, feel free to discuss on [github issue](https://github.com/Jxpro/custom-rustdesk/issues), or send an email to [jxpro@qq.com](mailto:jxpro@qq.com) to contact me.
