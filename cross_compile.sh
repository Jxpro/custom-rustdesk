#!/bin/bash

# =============================================================================
# RustDesk 跨平台编译脚本
# =============================================================================
# 基于 macOS ARM Rust 测试
#
# 功能特性:
#   ✅ macOS 平台编译 (ARM64/x86_64)
#   ✅ Universal Binary 自动创建
#   ✅ Linux musl 编译 (静态链接，高兼容性)
#   ⚠️ Linux GNU 编译 (x86_64 有链接问题，推荐使用 musl)
#   ⚠️ Windows 编译 (MinGW，可能遇到 C 依赖库问题)
#   🔧 自动化工具链管理
#
# 支持的目标平台:
#   macos-arm64      -> aarch64-apple-darwin     (✅ 完全支持)
#   macos-x64        -> x86_64-apple-darwin      (✅ 完全支持)
#   linux-arm64-musl -> aarch64-unknown-linux-musl (✅ 完全支持)
#   linux-x64-musl   -> x86_64-unknown-linux-musl (✅ 完全支持)
#   linux-arm64-gnu  -> aarch64-unknown-linux-gnu (✅ 完全支持)
#   linux-x64-gnu    -> x86_64-unknown-linux-gnu (❌ libsodium 链接问题)
#   windows-x64-gnu  -> x86_64-pc-windows-gnu    (❌ sodiumoxide 编译问题)
#   windows-arm64    -> aarch64-pc-windows-msvc   (⚠️  需要 MSVC 工具链)
#
# 快速使用:
#   ./cross_compile.sh --setup                    # 首次环境设置
#   ./cross_compile.sh --list                     # 查看支持的平台
#   ./cross_compile.sh macos-arm64                # 编译单个平台
#   ./cross_compile.sh macos-arm64 macos-x64      # 编译多个平台
#   ./cross_compile.sh --all                      # 编译所有平台
#
# 推荐的分发策略:
#   - macOS 用户: 使用 Universal Binary (macos-universal)
#   - Linux 服务器: 使用 musl 版本以获得最大兼容性
#   - 容器化部署: 优先选择 musl 静态链接版本
#   - ARM 设备: 使用对应的 ARM64 版本
#
# 已知问题和解决方案:
#   ❌ linux-x64-gnu: libsodium 链接问题
#      错误: undefined reference to __stack_chk_guard
#      原因: GNU 工具链交叉编译兼容性问题
#      解决: 使用 linux-x64-musl (推荐) 或 linux-arm64-gnu
#   ❌ windows-x64-gnu: sodiumoxide 编译问题
#      原因: C 依赖库交叉编译复杂性
#      解决: 在 Windows 环境原生编译或使用预编译库
#   ❌ windows-arm64: MSVC 工具链依赖
#      原因: aarch64-pc-windows-msvc 需要 Windows MSVC 环境
#      解决: 在 Windows ARM64 设备或 GitHub Actions 中编译
#   ✅ 推荐组合: macos-*, linux-*-musl, linux-arm64-gnu
# =============================================================================

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目信息
PROJECT_NAME="custom-rustdesk"
# 从 Cargo.toml 动态读取版本信息
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
OUTPUT_DIR="dist"

# 已知会失败的目标平台列表（不应导致脚本以错误码退出）
KNOWN_FAILING_TARGETS=("linux-x64-gnu" "windows-x64-gnu" "windows-arm64")

# 支持的目标平台（使用数组而非关联数组以提高兼容性）
TARGET_PLATFORMS="macos-arm64 macos-x64 linux-x64-gnu linux-arm64-gnu linux-x64-musl linux-arm64-musl windows-x64-gnu windows-arm64"
TARGET_TRIPLES="aarch64-apple-darwin x86_64-apple-darwin x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-unknown-linux-musl x86_64-pc-windows-gnu aarch64-pc-windows-msvc"

# =============================================================================
# 目标三元组查询函数
# =============================================================================
# 根据平台名称 (如 macos-arm64) 查找对应的 Rust 目标三元组
# (如 aarch64-apple-darwin)，用于 cargo build --target 参数
get_target_triple() {
    local platform="$1"
    local platforms=($TARGET_PLATFORMS)
    local triples=($TARGET_TRIPLES)
    
    for i in "${!platforms[@]}"; do
        if [[ "${platforms[$i]}" == "$platform" ]]; then
            echo "${triples[$i]}"
            return 0
        fi
    done
    return 1
}

# =============================================================================
# 平台支持检查函数
# =============================================================================
# 验证用户输入的平台名称是否在支持列表中
# 用于命令行参数验证，防止编译不支持的目标
is_platform_supported() {
    local platform="$1"
    local platforms=($TARGET_PLATFORMS)
    
    for p in "${platforms[@]}"; do
        if [[ "$p" == "$platform" ]]; then
            return 0
        fi
    done
    return 1
}

# =============================================================================
# 依赖检查函数
# =============================================================================
# 检查编译所需的基础工具:
#   - Rust 工具链 (rustc, cargo)
#   - Homebrew (macOS 上用于安装交叉编译工具链)
# 如果缺少必要工具，脚本将退出并提示用户安装
check_dependencies() {
    echo -e "${BLUE}检查依赖工具...${NC}"
    
    # 检查 Rust 工具链
    if ! command -v rustc &> /dev/null; then
        echo -e "${RED}错误: 未找到 Rust 工具链，请先安装 Rust${NC}"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}错误: 未找到 Cargo，请检查 Rust 安装${NC}"
        exit 1
    fi
    
    # 检查 Homebrew (macOS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if ! command -v brew &> /dev/null; then
            echo -e "${YELLOW}警告: 未找到 Homebrew，某些交叉编译工具链可能无法自动安装${NC}"
        fi
    fi
    
    echo -e "${GREEN}依赖检查完成${NC}"
}

# =============================================================================
# 交叉编译工具链安装函数
# =============================================================================
# 自动安装所需的交叉编译工具链:
#   - messense/macos-cross-toolchains: Linux GNU 工具链
#     * x86_64-linux-gnu-gcc (❌ 已知问题: libsodium 链接错误)
#     * aarch64-linux-gnu-gcc (✅ 工作正常，推荐用于 ARM64)
#   - FiloSottile/musl-cross: Linux musl 工具链 (推荐)
#     * x86_64-linux-musl-gcc (静态链接，高兼容性)
#     * aarch64-linux-musl-gcc (ARM64 静态链接)
#   - mingw-w64: Windows MinGW 工具链
#     * x86_64-w64-mingw32-gcc (❌ 已知问题: sodiumoxide 编译)
# 仅在 macOS 上运行，其他平台跳过
install_toolchains() {
    echo -e "${BLUE}安装交叉编译工具链...${NC}"
    
    # 检查并安装 Linux 交叉编译工具链 (仅在 macOS 上)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # 检查是否已添加 messense tap
        if ! brew tap | grep -q "messense/macos-cross-toolchains"; then
            echo -e "${YELLOW}添加 messense/macos-cross-toolchains tap...${NC}"
            brew tap messense/macos-cross-toolchains
        fi
        
        # 安装 Linux 工具链
        local gnu_toolchains=("x86_64-linux-gnu-gcc" "aarch64-linux-gnu-gcc")
        for toolchain in "${gnu_toolchains[@]}"; do
            if ! command -v "${toolchain}" &> /dev/null; then
                echo -e "${YELLOW}安装 ${toolchain} 工具链...${NC}"
                brew install "${toolchain}" || echo -e "${YELLOW}警告: ${toolchain} 工具链安装失败，将跳过相关目标${NC}"
            fi
        done
        
        # 检查并安装 musl 工具链
        if ! brew tap | grep -q "FiloSottile/musl-cross"; then
            echo -e "${YELLOW}添加 FiloSottile/musl-cross tap...${NC}"
            brew tap FiloSottile/musl-cross
        fi
        
        for toolchain in "musl-cross"; do
            if ! command -v "x86_64-linux-musl-gcc" &> /dev/null; then
                echo -e "${YELLOW}安装 musl 工具链...${NC}"
                brew install "${toolchain}" || echo -e "${YELLOW}警告: musl 工具链安装失败，将跳过 musl 目标${NC}"
            fi
        done
        
        # 安装 Windows 工具链
        if ! command -v "x86_64-w64-mingw32-gcc" &> /dev/null; then
            echo -e "${YELLOW}安装 Windows MinGW 工具链...${NC}"
            brew install mingw-w64 || echo -e "${YELLOW}警告: MinGW 工具链安装失败，将跳过 Windows 目标${NC}"
        fi
    fi
    
    echo -e "${GREEN}工具链安装完成${NC}"
}

# =============================================================================
# Rust 目标平台安装函数
# =============================================================================
# 使用 rustup 安装所有支持的目标平台:
#   - aarch64-apple-darwin (macOS ARM64)
#   - x86_64-apple-darwin (macOS Intel)
#   - x86_64-unknown-linux-gnu (Linux x64 GNU)
#   - aarch64-unknown-linux-gnu (Linux ARM64 GNU)
#   - x86_64-unknown-linux-musl (Linux x64 musl)
#   - aarch64-unknown-linux-musl (Linux ARM64 musl)
#   - x86_64-pc-windows-gnu (Windows x64 MinGW)
install_rust_targets() {
    echo -e "${BLUE}安装 Rust 目标平台...${NC}"
 # 安装 Rust 目标
    local triples=($TARGET_TRIPLES)
    for target in "${triples[@]}"; do
        echo -e "${YELLOW}安装目标: ${target}${NC}"
        rustup target add "$target" || echo -e "${YELLOW}警告: 目标 ${target} 安装失败${NC}"
    done
    
    echo -e "${GREEN}Rust 目标安装完成${NC}"
}

# =============================================================================
# Cargo 交叉编译配置函数
# =============================================================================
# 创建 .cargo/config.toml 文件，配置:
#   - 各目标平台的链接器 (linker)
#   - 环境变量 (CC_*) 指定 C 编译器
#   - 确保 Cargo 能找到正确的交叉编译工具链
# 这是交叉编译成功的关键配置文件
setup_cargo_config() {
    echo -e "${BLUE}配置 Cargo 交叉编译设置...${NC}"
    
    mkdir -p .cargo
    
    cat > .cargo/config.toml << 'EOF'
# Cargo 交叉编译配置

[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-unknown-linux-gnu-gcc"

[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-gcc"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

# Windows ARM64 使用 MSVC 工具链，无需额外配置
# [target.aarch64-pc-windows-msvc]
# 使用默认的 MSVC 链接器

# 环境变量配置
[env]
CC_x86_64_unknown_linux_gnu = "x86_64-unknown-linux-gnu-gcc"
CC_aarch64_unknown_linux_gnu = "aarch64-unknown-linux-gnu-gcc"
CC_x86_64_unknown_linux_musl = "x86_64-linux-musl-gcc"
CC_aarch64_unknown_linux_musl = "aarch64-linux-musl-gcc"
CC_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-gcc"
EOF
    
    echo -e "${GREEN}Cargo 配置完成${NC}"
}

# =============================================================================
# =============================================================================
# 系统信息显示函数
# =============================================================================
# 显示当前系统的CPU、内存和负载信息，提升编译过程的透明度
show_system_info() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        local cpu_count=$(sysctl -n hw.ncpu 2>/dev/null || echo "未知")
        local memory_gb=$(( $(sysctl -n hw.memsize 2>/dev/null || echo "0") / 1024 / 1024 / 1024 ))
        local load_avg=$(uptime | awk '{print $(NF-2)}' | sed 's/,//' 2>/dev/null || echo "未知")
        echo -e "${BLUE}💻 系统: ${cpu_count}核心, ${memory_gb}GB内存, 负载: ${load_avg}${NC}"
    else
        local cpu_count=$(nproc 2>/dev/null || echo "未知")
        local load_avg=$(uptime | awk '{print $(NF-2)}' | sed 's/,//' 2>/dev/null || echo "未知")
        echo -e "${BLUE}💻 系统: ${cpu_count}核心, 负载: ${load_avg}${NC}"
    fi
}

# =============================================================================
# 编译进度监控函数
# =============================================================================
# 实时监控cargo编译进程，显示动态进度指示器和编译状态
show_compilation_progress() {
    local platform="$1"
    local pid="$2"
    local log_file="$3"
    local dots=0
    local spinner_chars="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    local spinner_index=0
    local last_crate=""
    
    while kill -0 "$pid" 2>/dev/null; do
        # 显示旋转进度指示器
        local spinner_char=${spinner_chars:$spinner_index:1}
        printf "\r${YELLOW}  ${spinner_char} 正在编译 ${platform}"$(printf "%*s" $((dots % 4)) "" | tr ' ' '.')"${NC}\033[K"
        
        # 尝试解析当前编译的crate
        if [[ -f "$log_file" ]]; then
            local current_crate=$(tail -n 5 "$log_file" 2>/dev/null | grep "Compiling" | tail -n 1 | sed 's/.*Compiling \([^ ]*\).*/\1/' 2>/dev/null)
            if [[ -n "$current_crate" ]] && [[ "$current_crate" != "$last_crate" ]]; then
                printf "\r\033[K${BLUE}    📦 编译依赖: $current_crate${NC}\n"
                last_crate="$current_crate"
            fi
        fi
        
        ((dots++))
        spinner_index=$(( (spinner_index + 1) % ${#spinner_chars} ))
        sleep 1
    done
    printf "\r\033[K"
}



# =============================================================================
# 单个目标编译函数
# =============================================================================
# 编译指定的目标平台，包含完整的检查、编译和输出处理流程:
#   1. 验证 Rust 目标和交叉编译工具链
#   2. 显示系统信息（CPU、内存、负载）
#   3. 执行编译并提供实时进度监控（动态指示器、依赖包状态）
#   4. 处理编译结果和二进制文件输出
#   5. 智能错误处理和解决方案建议
#
# 已知问题:
#   - linux-x64-gnu: ❌ libsodium 链接错误 (__stack_chk_guard 未定义)
#     解决方案: 使用 linux-x64-musl 替代 (推荐)
#   - windows-x64-gnu: ❌ sodiumoxide 编译错误
#     解决方案: 在 Windows 环境原生编译
compile_target() {
    local platform_name="$1"
    local target="$2"
    
    echo -e "${BLUE}🔨 编译目标: ${platform_name} (${target})${NC}"
    
    # 显示系统信息
    show_system_info
    
    # 检查目标是否已安装
    if ! rustup target list --installed | grep -q "$target"; then
        echo -e "${YELLOW}目标 ${target} 未安装，跳过编译${NC}"
        return 1
    fi
    
    # 检查交叉编译工具链
    case "$target" in
        *linux-gnu)
            local arch="${target%%-*}"
            local gcc_name="${arch}-linux-gnu-gcc"
            if ! command -v "$gcc_name" &> /dev/null; then
                echo -e "${YELLOW}交叉编译器 ${gcc_name} 未找到，跳过 ${platform_name}${NC}"
                return 1
            fi
            ;;
        *linux-musl)
            local arch="${target%%-*}"
            local gcc_name="${arch}-linux-musl-gcc"
            if ! command -v "$gcc_name" &> /dev/null; then
                echo -e "${YELLOW}交叉编译器 ${gcc_name} 未找到，跳过 ${platform_name}${NC}"
                return 1
            fi
            ;;
        *windows-gnu)
            if ! command -v "x86_64-w64-mingw32-gcc" &> /dev/null; then
                echo -e "${YELLOW}MinGW 编译器未找到，跳过 ${platform_name}${NC}"
                return 1
            fi
            ;;
        *windows-msvc)
            if [[ "$OSTYPE" != "win32" ]] && [[ "$OSTYPE" != "msys" ]] && [[ "$OSTYPE" != "cygwin" ]]; then
                echo -e "${YELLOW}Windows MSVC 工具链需要在 Windows 环境中编译，跳过 ${platform_name}${NC}"
                return 1
            fi
            ;;
    esac
    
    # 执行编译（带实时进度监控）
    echo -e "${YELLOW}🚀 开始编译...${NC}"
    
    # 启动后台编译进程
    cargo build --release --target "$target" >compile_${platform_name}.log 2>&1 &
    local cargo_pid=$!
    
    # 显示实时编译进度
    show_compilation_progress "$platform_name" "$cargo_pid" "compile_${platform_name}.log"
    
    # 等待编译完成并获取退出码
    wait "$cargo_pid"
    local exit_code=$?
    
    if [[ $exit_code -eq 0 ]]; then
        echo -e "${GREEN}✓ ${platform_name} 编译成功${NC}"
        
        # 复制二进制文件到输出目录
        local binary_name="$PROJECT_NAME"
        local source_path="target/${target}/release/${binary_name}"
        
        # Windows 平台添加 .exe 扩展名
        if [[ "$target" == *windows* ]]; then
            source_path="${source_path}.exe"
            binary_name="${binary_name}.exe"
        fi
        
        if [[ -f "$source_path" ]]; then
            mkdir -p "$OUTPUT_DIR"
            local output_name="${PROJECT_NAME}-${VERSION}-${platform_name}"
            if [[ "$target" == *windows* ]]; then
                output_name="${output_name}.exe"
            fi
            cp "$source_path" "${OUTPUT_DIR}/${output_name}"
            echo -e "${GREEN}  → 输出: ${OUTPUT_DIR}/${output_name}${NC}"
        else
            echo -e "${RED}  ✗ 未找到编译输出文件: ${source_path}${NC}"
        fi
        
        # 清理临时日志文件（编译成功时）
        rm -f "compile_${platform_name}.log"
        return 0
    else
        echo -e "${RED}✗ ${platform_name} 编译失败${NC}"
        
        # 提供智能错误建议
        provide_error_suggestion "$platform_name" "$target"
        
        # 保留日志文件供用户查看（编译失败时不清理）
        echo -e "${BLUE}  📄 编译详情已保存到: compile_${platform_name}.log${NC}"
        return 1
    fi
}

# =============================================================================
# 检查是否为已知失败目标
# =============================================================================
# 判断给定的目标平台是否在已知失败列表中
# 用于决定编译失败时是否应该以错误码退出
is_known_failing_target() {
    local platform="$1"
    for known_target in "${KNOWN_FAILING_TARGETS[@]}"; do
        if [[ "$platform" == "$known_target" ]]; then
            return 0  # 是已知失败目标
        fi
    done
    return 1  # 不是已知失败目标
}

# =============================================================================
# 智能错误建议函数
# =============================================================================
# 根据编译失败的目标平台提供针对性的解决建议
# 帮助用户快速定位问题并找到替代方案
provide_error_suggestion() {
    local platform_name="$1"
    local target="$2"
    
    echo -e "${BLUE}💡 建议解决方案:${NC}"
    
    case "$platform_name" in
        "linux-x64-gnu")
            echo -e "${YELLOW}  ❌ 已知问题: libsodium 链接错误${NC}"
            echo -e "${GREEN}  ✅ 推荐替代: ./cross_compile.sh linux-x64-musl${NC}"
            echo -e "${GREEN}  ✅ 或者使用: ./cross_compile.sh linux-arm64-gnu${NC}"
            ;;
        "windows-x64-gnu")
            echo -e "${YELLOW}  ❌ 已知问题: sodiumoxide 交叉编译问题${NC}"
            echo -e "${GREEN}  ✅ 建议: 在 Windows 环境中原生编译${NC}"
            echo -e "${GREEN}  ✅ 或者: 使用预编译的 libsodium 库${NC}"
            ;;
        "windows-arm64")
            echo -e "${YELLOW}  ❌ 已知问题: 需要 Windows MSVC 工具链${NC}"
            echo -e "${GREEN}  ✅ 建议: 在 Windows ARM64 设备上原生编译${NC}"
            echo -e "${GREEN}  ✅ 或者: 使用 GitHub Actions Windows ARM64 runner${NC}"
            ;;
        *linux*)
            echo -e "${YELLOW}  🔧 检查工具链是否正确安装${NC}"
            echo -e "${GREEN}  ✅ 尝试: brew install messense/macos-cross-toolchains/${target%%-*}-linux-gnu-gcc${NC}"
            ;;
        *)
            echo -e "${YELLOW}  🔧 检查编译错误详情: compile_error_${platform_name}.log${NC}"
            echo -e "${GREEN}  ✅ 尝试: ./cross_compile.sh --setup 重新安装工具链${NC}"
            ;;
    esac
    echo ""
}

# =============================================================================
# macOS Universal Binary 创建函数
# =============================================================================
# 使用 lipo 工具将 ARM64 和 x86_64 二进制文件合并为 Universal Binary
# 这样生成的文件可以在所有 macOS 设备上原生运行:
#   - Apple Silicon Mac (M1/M2/M3) 使用 ARM64 部分
#   - Intel Mac 使用 x86_64 部分
# 文件大小约为单架构版本的两倍，但提供最佳兼容性
create_universal_binary() {
    echo -e "${BLUE}创建 macOS Universal Binary...${NC}"
    
    local arm64_binary="target/aarch64-apple-darwin/release/$PROJECT_NAME"
    local x64_binary="target/x86_64-apple-darwin/release/$PROJECT_NAME"
    local universal_binary="${OUTPUT_DIR}/${PROJECT_NAME}-${VERSION}-macos-universal"
    
    if [[ -f "$arm64_binary" ]] && [[ -f "$x64_binary" ]]; then
        if command -v lipo &> /dev/null; then
            mkdir -p "$OUTPUT_DIR"
            lipo -create -output "$universal_binary" "$arm64_binary" "$x64_binary"
            echo -e "${GREEN}✓ Universal Binary 创建成功: ${universal_binary}${NC}"
        else
            echo -e "${YELLOW}警告: lipo 工具未找到，无法创建 Universal Binary${NC}"
        fi
    else
        echo -e "${YELLOW}警告: 缺少 macOS 二进制文件，无法创建 Universal Binary${NC}"
    fi
}

# =============================================================================
# 帮助信息显示函数
# =============================================================================
# 显示脚本的完整使用说明，包括:
#   - 基本用法和选项说明
#   - 所有支持的目标平台列表
#   - 常用命令示例
show_help() {
    echo "RustDesk 跨平台编译脚本"
    echo ""
    echo "用法: $0 [选项] [目标平台...]"
    echo ""
    echo "选项:"
    echo "  -h, --help     显示此帮助信息"
    echo "  -l, --list     列出所有支持的目标平台"
    echo "  -s, --setup    仅执行环境设置（安装工具链和目标）"
    echo "  -c, --clean    清理编译输出"
    echo "  -a, --all      编译所有支持的目标平台"
    echo ""
    echo "支持的目标平台:"
    local platforms=($TARGET_PLATFORMS)
    local triples=($TARGET_TRIPLES)
    for i in "${!platforms[@]}"; do
        echo "  ${platforms[$i]} (${triples[$i]})"
    done
    echo ""
    echo "示例:"
    echo "  $0 --setup                    # 设置编译环境"
    echo "  $0 macos-arm64 macos-x64      # 编译 macOS 平台"
    echo "  $0 linux-x64-musl             # 编译 Linux musl 版本"
    echo "  $0 --all                      # 编译所有平台"
}

# =============================================================================
# 目标平台列表函数
# =============================================================================
# 列出所有支持的目标平台及其安装状态:
#   - 平台名称 (如 macos-arm64)
#   - Rust 目标三元组 (如 aarch64-apple-darwin)
#   - 安装状态 (已安装/未安装)
list_targets() {
    echo "支持的目标平台:"
    local platforms=($TARGET_PLATFORMS)
    local triples=($TARGET_TRIPLES)
    for i in "${!platforms[@]}"; do
        local platform="${platforms[$i]}"
        local target="${triples[$i]}"
        local status="未安装"
        if rustup target list --installed | grep -q "$target"; then
            status="已安装"
        fi
        printf "  %-20s %-30s %s\n" "$platform" "$target" "$status"
    done
}

# =============================================================================
# 编译清理函数
# =============================================================================
# 清理所有编译产物:
#   - 执行 cargo clean 清理 target/ 目录
#   - 删除 dist/ 输出目录
#   - 释放磁盘空间，确保干净的编译环境
clean_build() {
    echo -e "${BLUE}清理编译输出...${NC}"
    cargo clean
    rm -rf "$OUTPUT_DIR"
    echo -e "${GREEN}清理完成${NC}"
}

# =============================================================================
# 主函数 - 脚本执行入口
# =============================================================================
# 执行流程:
#   1. 解析命令行参数 (--help, --list, --setup, --clean, --all, 目标平台)
#   2. 显示项目信息和配置
#   3. 检查依赖工具 (Rust, Cargo, Homebrew)
#   4. 安装交叉编译工具链 (GNU, musl, MinGW)
#   5. 安装 Rust 目标平台
#   6. 配置 Cargo 交叉编译设置
#   7. 编译指定的目标平台
#   8. 创建 macOS Universal Binary (如果适用)
#   9. 显示编译结果和输出文件列表
#
# 支持的操作模式:
#   - 环境设置模式 (--setup): 仅安装工具链，不编译
#   - 清理模式 (--clean): 清理编译输出
#   - 全量编译模式 (--all): 编译所有支持的目标
#   - 选择性编译: 编译指定的目标平台
main() {
    local setup_only=false
    local compile_all=false
    local targets_to_compile=()
    
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -l|--list)
                list_targets
                exit 0
                ;;
            -s|--setup)
                setup_only=true
                shift
                ;;
            -c|--clean)
                clean_build
                exit 0
                ;;
            -a|--all)
                compile_all=true
                shift
                ;;
            -*)
                echo -e "${RED}未知选项: $1${NC}"
                show_help
                exit 1
                ;;
            *)
                if is_platform_supported "$1"; then
                    targets_to_compile+=("$1")
                else
                    echo -e "${RED}未知目标平台: $1${NC}"
                    echo "使用 --list 查看支持的目标平台"
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # 显示项目信息
    echo -e "${GREEN}RustDesk 跨平台编译脚本${NC}"
    echo -e "项目: ${PROJECT_NAME} v${VERSION}"
    echo -e "输出目录: ${OUTPUT_DIR}"
    echo ""
    
    # 检查依赖
    check_dependencies
    
    # 安装工具链和目标
    install_toolchains
    install_rust_targets
    setup_cargo_config
    
    if [[ "$setup_only" == true ]]; then
        echo -e "${GREEN}环境设置完成${NC}"
        exit 0
    fi
    
    # 确定要编译的目标
    if [[ "$compile_all" == true ]]; then
        local platforms=($TARGET_PLATFORMS)
        targets_to_compile=("${platforms[@]}")
    elif [[ ${#targets_to_compile[@]} -eq 0 ]]; then
        echo -e "${YELLOW}未指定编译目标，使用 --help 查看用法${NC}"
        exit 1
    fi
    
    # 开始编译
    echo -e "${BLUE}开始编译...${NC}"
    local success_count=0
    local total_count=${#targets_to_compile[@]}
    local start_time=$(date +%s)
    local failed_targets=()
    
    for i in "${!targets_to_compile[@]}"; do
        local platform="${targets_to_compile[$i]}"
        local current=$((i + 1))
        local target_start_time=$(date +%s)
        
        echo -e "${BLUE}[${current}/${total_count}] 编译目标: ${platform}${NC}"
        
        local target=$(get_target_triple "$platform")
        if [[ -n "$target" ]] && compile_target "$platform" "$target"; then
            ((success_count++))
            local target_end_time=$(date +%s)
            local target_duration=$((target_end_time - target_start_time))
            echo -e "${GREEN}✅ [${current}/${total_count}] ${platform} 编译成功 (${target_duration}s)${NC}"
        else
            failed_targets+=("$platform")
            local target_end_time=$(date +%s)
            local target_duration=$((target_end_time - target_start_time))
            echo -e "${RED}❌ [${current}/${total_count}] ${platform} 编译失败 (${target_duration}s)${NC}"
        fi
        echo ""
    done
    
    # 创建 macOS Universal Binary（如果适用）
    if [[ " ${targets_to_compile[*]} " =~ " macos-arm64 " ]] && [[ " ${targets_to_compile[*]} " =~ " macos-x64 " ]]; then
        create_universal_binary
    fi
    
    # 显示编译结果
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    local minutes=$((total_duration / 60))
    local seconds=$((total_duration % 60))
    
    echo -e "${BLUE}编译完成${NC}"
    echo -e "成功: ${success_count}/${total_count}"
    if [[ $minutes -gt 0 ]]; then
        echo -e "总耗时: ${minutes}分${seconds}秒"
    else
        echo -e "总耗时: ${seconds}秒"
    fi
    
    if [[ -d "$OUTPUT_DIR" ]]; then
        echo -e "\n${GREEN}输出文件:${NC}"
        ls -la "$OUTPUT_DIR"/
    fi
    
    # 显示编译摘要
    echo ""
    echo -e "${BLUE}📊 编译摘要:${NC}"
    for i in "${!targets_to_compile[@]}"; do
        local platform="${targets_to_compile[$i]}"
        local target=$(get_target_triple "$platform")
        local binary_path="$OUTPUT_DIR/${PROJECT_NAME}-${VERSION}-${platform}"
        
        if [[ -f "$binary_path" ]] || [[ -f "${binary_path}.exe" ]]; then
            echo -e "  ${GREEN}✅ ${platform}${NC}"
        else
            echo -e "  ${RED}❌ ${platform}${NC}"
        fi
    done
    
    # 分析编译结果和退出策略
    if [[ $success_count -eq $total_count ]]; then
        echo -e "\n${GREEN}🎉 所有目标编译成功！${NC}"
        exit 0
    else
        # 检查失败的目标是否都是已知会失败的目标
        local unexpected_failures=()
        for failed_target in "${failed_targets[@]}"; do
            if ! is_known_failing_target "$failed_target"; then
                unexpected_failures+=("$failed_target")
            fi
        done
        
        if [[ ${#unexpected_failures[@]} -eq 0 ]]; then
            # 所有失败都是已知的，正常退出
            echo -e "\n${YELLOW}⚠️  部分目标编译失败（已知问题），但不影响整体构建${NC}"
            echo -e "${BLUE}💡 提示: 查看 compile_*.log 获取详细编译信息${NC}"
            exit 0
        else
            # 存在意外失败，以错误码退出
            echo -e "\n${RED}❌ 发现意外的编译失败: ${unexpected_failures[*]}${NC}"
            echo -e "${BLUE}💡 提示: 查看 compile_*.log 获取详细编译信息${NC}"
            exit 1
        fi
    fi
}

# 运行主函数
main "$@"