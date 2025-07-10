# RustDesk 自定义 ID 工具

## 📖 简介

>   注意，如果某个ID无效被重置，可能是ID过短或被占用，请尝试使用其他ID

本项目旨在为 `RustDesk` 提供自定义 ID 生成功能，方便用户记忆和管理设备。主要功能为模拟官方加密算法，将用户输入的自定义 ID 加密并输出加密后的 ID，将其替换到配置文件中的 `enc_id` 字段即可。

`MacOS` 配置文件位置

-   `~/Library/Preferences/com.carriez.RustDesk/RustDesk.toml`

`Windows` 配置文件位置

-   `C:\Users\username\AppData\Roaming\RustDesk\config\RustDesk.toml`

`Windows`下服务模式（指定`--service`）的配置文件，会覆盖`C:\Users\username`下的`RustDesk.toml`

-   `C:\Windows\ServiceProfiles\LocalService\AppData\Roaming\RustDesk\config\RustDesk.toml`

## ✨ 功能

*   🔒 生成加密后的自定义 ID
*   🔓 对加密后的 ID 进行解密验证
*   🔑 使用 UUID 作为加密和解密密钥
*   📋 自动复制加密/解密结果到剪贴板
*   💬 交互式模式，操作简便
*   📚 完善的帮助系统
*   🌍 多语言支持（中文/英文）
*   ⌨️ 命令行界面，包含详细参数说明

## 🚀 安装与快速开始

### 📦 方式一：下载预编译二进制文件（推荐）

最简单的使用方式是从我们的发布页面下载预编译的二进制文件：

**📥 [下载最新版本](https://github.com/Jxpro/custom-rustdesk/releases)**

支持的平台：
- **Linux**: `custom-rustdesk-linux-x86_64-gnu`, `custom-rustdesk-linux-aarch64-gnu`
- **Linux (MUSL)**: `custom-rustdesk-linux-x86_64-musl`, `custom-rustdesk-linux-aarch64-musl`
- **Windows**: `custom-rustdesk-windows-x86_64.exe`, `custom-rustdesk-windows-aarch64.exe`
- **macOS**: `custom-rustdesk-macos-universal`（支持 Intel 和 Apple Silicon）

#### 预编译二进制文件快速开始：

1. 下载适合您平台的二进制文件
2. 添加执行权限（Linux/macOS）：`chmod +x custom-rustdesk-*`
3. 直接运行：
   ```bash
   # 交互式模式
   ./custom-rustdesk-macos-universal
   
   # 命令行模式
   ./custom-rustdesk-macos-universal --id 123456 --uuid your-uuid-here
   ```

### 🔨 方式二：从源码构建

如果您希望从源码构建或需要修改代码：

#### 📋 前置要求
- [Rust](https://rustup.rs/)（最新稳定版本）
- Git

#### 🛠️ 构建步骤

1. **克隆仓库：**
   ```bash
   git clone https://github.com/Jxpro/custom-rustdesk.git
   cd custom-rustdesk
   ```

2. **构建项目：**
   ```bash
   cargo build --release
   ```

3. **运行构建的二进制文件：**
   ```bash
   # 交互式模式
   cargo run --release
   
   # 或直接运行构建的二进制文件
   ./target/release/custom-rustdesk
   ```

#### 🧪 开发构建
开发时可以直接使用 cargo 运行：
```bash
cargo run
```

## 📘 使用方法

### 💬 交互式模式

不带参数运行进入交互式模式：

```bash
# 使用预编译二进制文件
./custom-rustdesk-macos-universal

# 或从源码运行
cargo run
```

交互式菜单提供：
1. **加密模式**：从自定义 ID 生成加密 ID
2. **解密模式**：验证并解密加密 ID
3. **查看帮助**：显示详细帮助信息
4. **退出**：退出应用程序

### ⌨️ 命令行模式

工具支持命令行和交互式两种模式。命令行使用方法：

```bash
# 使用预编译二进制文件：
# 生成加密 ID
./custom-rustdesk-macos-universal --id <自定义ID> --uuid <机器UUID>

# 验证加密 ID
./custom-rustdesk-macos-universal --eid <加密ID> --uuid <机器UUID>

# 设置语言（en/zh）
./custom-rustdesk-macos-universal --lang zh

# 显示帮助
./custom-rustdesk-macos-universal --help

# 从源码运行：
# 生成加密 ID
cargo run -- --id <自定义ID> --uuid <机器UUID>

# 验证加密 ID
cargo run -- --eid <加密ID> --uuid <机器UUID>

# 设置语言（en/zh）
cargo run -- --lang zh

# 显示帮助
cargo run -- --help
```

#### 📝 命令行参数

- `-i, --id <ID>`：要加密的自定义 ID
- `-e, --eid <EID>`：要解密的加密 ID
- `-u, --uuid <UUID>`：用于加密/解密的 UUID
- `-l, --lang <LANG>`：设置语言（en/zh）[默认：en]
- `-h, --help`：显示详细帮助信息

### 🌍 语言支持

工具支持中文和英文：
- 默认语言为英文
- 使用 `--lang zh` 切换到中文界面
- 语言设置影响所有输出，包括帮助文本和错误信息

### 🔍 获取 UUID

#### 🤖 自动检测 UUID（推荐）

**本工具现已支持自动检测机器 UUID！** 这是最简单、最便捷的方式：

- **交互式模式**：运行程序时会自动检测并显示机器 UUID，询问是否使用
- **命令行模式**：当未提供 `--uuid` 参数时，自动检测并确认使用
- **跨平台支持**：支持 Windows、macOS 和 Linux 系统
- **用户确认**：检测到 UUID 后会询问用户确认，直接按回车键默认选择「是」

使用示例：
```bash
# 自动检测 UUID（推荐方式）
./custom-rustdesk-macos-universal --id 123456
# 程序会自动检测 UUID 并询问确认

# 交互式模式也支持自动检测
./custom-rustdesk-macos-universal
```

#### 📋 手动获取 UUID

如果需要手动获取或验证 UUID，以用于为其他设备配置 RustDesk，请参考以下方法：

>   你还可以通过官方工具 [machine-uid](https://github.com/rustdesk-org/machine-uid) 获取更多完整信息

1.  **Windows:**

    -   打开终端。
    -   输入以下命令：`(Get-ItemProperty -Path Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography).MachineGuid`
    -   将 `MachineGuid` 值复制为 `uuid` 参数。

2.  **MacOS:**

    -   打开终端。
    -   输入以下命令：`ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID`
    -   将输出中的 UUID 复制为 `uuid` 参数。

3.  **Linux:**

    -   打开终端。
    -   输入以下命令：`cat /etc/machine-id` 或 `cat /var/lib/dbus/machine-id`
    -   将输出的机器 ID 复制为 `uuid` 参数。

### ✅ 验证 UUID

1.  下载预编译二进制文件或将代码克隆到本地
2.  在对应的配置文件中找到 `enc_id` 字段
3.  运行验证命令：
    ```bash
    # 使用预编译二进制文件
    ./custom-rustdesk-macos-universal --eid $enc_id --uuid $uuid
    
    # 或从源码运行
    cargo run -- --eid $enc_id --uuid $uuid
    ```
4.  程序会输出解密后的 ID，与当前 ID 比较是否一致

### 🎯 自定义ID

#### 🚀 使用自动检测 UUID（推荐）

1.  下载预编译二进制文件或将代码克隆到本地
2.  运行加密命令（无需手动提供 UUID）：
    ```bash
    # 使用预编译二进制文件
    ./custom-rustdesk-macos-universal --id 我的电脑
    
    # 或从源码运行
    cargo run -- --id 我的电脑
    ```
3.  程序会自动检测 UUID 并询问确认，直接按回车键或输入 `y` 确认
4.  程序输出加密后的 ID，复制并替换到配置文件中的 `enc_id` 字段

#### 📋 手动指定 UUID

如果需要使用特定的 UUID，可以手动指定：

```bash
# 使用预编译二进制文件
./custom-rustdesk-macos-universal --id 我的电脑 --uuid 12345678-1234-1234-1234-123456789012

# 从源码运行
cargo run -- --id 我的电脑 --uuid 12345678-1234-1234-1234-123456789012
```

#### 💡 程序运行示例

**自动检测模式：**
```bash
$ ./custom-rustdesk-macos-universal --id 测试电脑
🤖 自动检测到机器 UUID：
📱 检测到的 UUID: 3C17252C-4A25-54AB-8A92-B88D3D6665AA

✅ 使用此 UUID？(y/n): [直接按回车或输入y]
"测试电脑" 已加密为 "00u33upzDoDQeMfJZ36o3owBtJ0Ip8qKr2dff8qsbAug=="
✅ 已复制到剪切板
📝 请将配置文件中的 id 替换为 enc_id 字段
```

**手动指定模式：**
```bash
$ ./custom-rustdesk-macos-universal --id 测试电脑 --uuid 12345678-1234-1234-1234-123456789012
"测试电脑" 已加密为 "00M72xC5id8C/F+IsG6VOWs5MEV2xhPI/nBBo="
✅ 已复制到剪切板
📝 请将配置文件中的 id 替换为 enc_id 字段
```

## 🔐 加密流程

该程序使用`sodiumoxide `库中的`crypto::secretbox` 模块进行对称加密。加密密钥来自提供的 UUID 字符串。

1.   将自定义 ID 字符串转换为字节数组。
2.   将 UUID 字符串转换为字节数组，并调整其大小以匹配密钥长度要求。
3.   使用`sodiumoxide::crypto::secretbox`模块创建密钥和`nonce`。
4.   根据`encrypt`参数选择加密或解密操作。
5.   使用`secretbox::seal`或`secretbox::open`函数进行加密或解密操作。
6.   将加密后的字节数组转换为`base64`编码字符串，并输出到控制台。

## 🤝 贡献

欢迎您对该项目进行贡献！您可以通过以下方式参与：

-   提交代码补丁或问题报告
-   提供反馈和建议
-   帮助推广项目

## 📧 联系方式

如果有任何问题，欢迎到 [github issue](https://github.com/Jxpro/custom-rustdesk/issues) 进行讨论，或发送电子邮件到 [jxpro@qq.com](mailto:jxpro@qq.com) 来联系我