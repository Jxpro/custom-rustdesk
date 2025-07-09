#!/bin/bash

# RustDesk Custom ID Tool - i18n Coverage Test Script
# Comprehensive testing of internationalization functionality coverage and correctness

# Color definitions
readonly RED='\033[0;31m' GREEN='\033[0;32m' YELLOW='\033[1;33m' BLUE='\033[0;34m' NC='\033[0m'

# Test data constants
readonly TEST_ID="test123"
readonly TEST_UUID="12345678-1234-1234-1234-123456789abc"
readonly INVALID_EID="invalid"

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Unified logging functions
log_message() {
    local level="$1"
    local message="$2"
    local color="$3"
    echo -e "${color}[${level}]${NC} ${message}"
}

log_info() { log_message "INFO" "$1" "$BLUE"; }
log_success() { log_message "PASS" "$1" "$GREEN"; ((PASSED_TESTS++)); }
log_error() { log_message "FAIL" "$1" "$RED"; }
log_warning() { log_message "WARN" "$1" "$YELLOW"; }

# Optimized test function
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_pattern="$3"
    
    ((TOTAL_TESTS++))
    log_info "Running test: $test_name"
    
    local output
    if output=$(eval "$test_command" 2>&1) && echo "$output" | grep -q "$expected_pattern"; then
        log_message "PASS" "$test_name" "$GREEN"
        ((PASSED_TESTS++))
        return 0
    else
        log_message "FAIL" "$test_name - Expected pattern '$expected_pattern' not found" "$RED"
        echo "Output: $output"
        return 1
    fi
}

# Check if required files exist
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    local required_files=("Cargo.toml" "i18n/en.yml" "i18n/zh.yml")
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            log_error "$file not found. Please run this script from the project root."
            exit 1
        fi
    done
    
    log_message "PASS" "Prerequisites check passed" "$GREEN"
}

# Build project
build_project() {
    log_info "Building project..."
    if cargo build --quiet >/dev/null 2>&1; then
        log_message "PASS" "Project build successful" "$GREEN"
    else
        log_error "Project build failed"
        return 1
    fi
}

# Test basic startup and language switching
test_basic_functionality() {
    log_info "=== Testing Basic Functionality ==="
    
    local base_cmd="echo '0' | cargo run --quiet --"
    
    run_test "English mode startup" "$base_cmd --lang en" "RustDesk Custom ID Tool"
    run_test "Chinese mode startup" "$base_cmd --lang zh" "RustDesk 自定义 ID 工具"
    run_test "Default language startup" "$base_cmd" "RustDesk Custom ID Tool"
}

# Test CLI encryption functionality
test_cli_encryption() {
    log_info "=== Testing CLI Encryption ==="
    
    local base_cmd="cargo run --quiet -- --id '$TEST_ID' --uuid '$TEST_UUID'"
    
    run_test "CLI encryption (English)" "$base_cmd --lang en" "is encrypted to"
    run_test "CLI encryption (Chinese)" "$base_cmd --lang zh" "已加密为"
    
    # Get encrypted ID for subsequent decryption tests
    ENCRYPTED_ID=$(cargo run --quiet -- --id "$TEST_ID" --uuid "$TEST_UUID" --lang en 2>/dev/null | grep -o '[A-Za-z0-9+/=]\{20,\}' | head -1)
    
    if [ -n "$ENCRYPTED_ID" ]; then
        log_message "PASS" "Encrypted ID captured: $ENCRYPTED_ID" "$GREEN"
    else
        log_error "Failed to capture encrypted ID"
    fi
}

# Test CLI decryption functionality
test_cli_decryption() {
    log_info "=== Testing CLI Decryption ==="
    
    if [ -n "$ENCRYPTED_ID" ]; then
        local base_cmd="cargo run --quiet -- --eid '$ENCRYPTED_ID' --uuid '$TEST_UUID'"
        
        run_test "CLI decryption (English)" "$base_cmd --lang en" "is decrypted to"
        run_test "CLI decryption (Chinese)" "$base_cmd --lang zh" "已解密为"
    else
        log_warning "Skipping decryption tests - no encrypted ID available"
    fi
}

# Test error handling
test_error_handling() {
    log_info "=== Testing Error Handling ==="
    
    # Test missing UUID parameter
    run_test "Missing UUID error (English)" "cargo run --quiet -- --id '$TEST_ID' --lang en" "UUID is required"
    run_test "Missing UUID error (Chinese)" "cargo run --quiet -- --id '$TEST_ID' --lang zh" "需要 UUID"
    
    # Test invalid encrypted ID
    local invalid_cmd="cargo run --quiet -- --eid '$INVALID_EID' --uuid '$TEST_UUID'"
    run_test "Invalid encrypted ID error (English)" "$invalid_cmd --lang en" "Error occurred during decryption"
    run_test "Invalid encrypted ID error (Chinese)" "$invalid_cmd --lang zh" "解密过程中发生错误"
}

# Test help system
test_help_system() {
    log_info "=== Testing Help System ==="
    
    local help_cmd="echo '3' | cargo run --quiet --"
    run_test "Help system (English)" "$help_cmd --lang en" "Program Function"
    run_test "Help system (Chinese)" "$help_cmd --lang zh" "程序功能"
}

# Test interactive mode
test_interactive_mode() {
    log_info "=== Testing Interactive Mode ==="
    
    local interactive_input="printf '1\\n$TEST_ID\\n$TEST_UUID\\n'"
    run_test "Interactive encryption mode (English)" "$interactive_input | cargo run --quiet -- --lang en" "Please enter your custom ID"
    run_test "Interactive encryption mode (Chinese)" "$interactive_input | cargo run --quiet -- --lang zh" "请输入您的自定义 ID"
}

# Validate translation file integrity
validate_translation_files() {
    log_info "=== Validating Translation Files ==="
    
    local key_pattern='^[a-zA-Z_][a-zA-Z0-9_]*:'
    local files=("en" "zh")
    
    # Check basic syntax of translation files
    for lang in "${files[@]}"; do
        if grep -q "$key_pattern" "i18n/$lang.yml"; then
            log_message "PASS" "$lang translation file has valid key-value pairs" "$GREEN"
        else
            log_error "$lang translation file format issues"
        fi
    done
    
    # Check consistency of translation keys
    local en_keys zh_keys
    en_keys=$(grep -E '^[a-zA-Z_]+:' i18n/en.yml | cut -d: -f1 | sort)
    zh_keys=$(grep -E '^[a-zA-Z_]+:' i18n/zh.yml | cut -d: -f1 | sort)
    
    if [ "$en_keys" = "$zh_keys" ]; then
        log_message "PASS" "Translation keys are consistent between languages" "$GREEN"
    else
        log_error "Translation keys mismatch between languages"
        echo "English keys: $(echo "$en_keys" | wc -l)"
        echo "Chinese keys: $(echo "$zh_keys" | wc -l)"
    fi
}

# Check emoji usage
check_emoji_usage() {
    log_info "=== Checking Emoji Usage ==="
    
    local emoji_pattern='[🎯📝🔐👋🔑🔍📖🚪❌✅📋💡⚠️📞🚀🍎🪟🐙📧📂✏️🔄📏🆔]'
    local en_count zh_count
    
    en_count=$(grep -o "$emoji_pattern" i18n/en.yml | wc -l)
    zh_count=$(grep -o "$emoji_pattern" i18n/zh.yml | wc -l)
    
    log_info "English emoji count: $en_count"
    log_info "Chinese emoji count: $zh_count"
    
    if [ "$en_count" -eq "$zh_count" ] && [ "$en_count" -gt 0 ]; then
        log_message "PASS" "Emoji usage is consistent and present" "$GREEN"
    else
        log_warning "Emoji usage may be inconsistent between languages"
    fi
}

# Performance test
performance_test() {
    log_info "=== Performance Test ==="
    
    local start_time end_time startup_time
    start_time=$(date +%s.%N)
    echo '0' | cargo run --quiet -- --lang en >/dev/null 2>&1
    end_time=$(date +%s.%N)
    startup_time=$(echo "$end_time - $start_time" | bc)
    
    log_info "Startup time: ${startup_time}s"
    
    if [ "$(echo "$startup_time < 5.0" | bc -l)" = "1" ]; then
        log_message "PASS" "Startup performance is good" "$GREEN"
    else
        log_warning "Startup time is slower than expected"
    fi
}

# Generate test report
generate_report() {
    FAILED_TESTS=$((TOTAL_TESTS - PASSED_TESTS))
    local success_rate=0
    
    [ $TOTAL_TESTS -gt 0 ] && success_rate=$(( PASSED_TESTS * 100 / TOTAL_TESTS ))
    
    cat << EOF

===========================================
           i18n Coverage Test Report
===========================================
Total Tests: $TOTAL_TESTS
$(echo -e "${GREEN}Passed: $PASSED_TESTS${NC}")
$(echo -e "${RED}Failed: $FAILED_TESTS${NC}")
Success Rate: ${success_rate}%
===========================================

EOF
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}🎉 All tests passed! i18n coverage is complete.${NC}"
        exit 0
    else
        echo -e "${RED}❌ Some tests failed. Please review the issues above.${NC}"
        exit 1
    fi
}

# Main function
main() {
    echo "🚀 Starting i18n Coverage Test..."
    echo
    
    # Prerequisites check and build
    check_prerequisites
    build_project
    
    # Execute all tests
    local test_functions=(
        test_basic_functionality
        test_cli_encryption
        test_cli_decryption
        test_error_handling
        test_help_system
        test_interactive_mode
        validate_translation_files
        check_emoji_usage
        performance_test
    )
    
    for test_func in "${test_functions[@]}"; do
        "$test_func"
    done
    
    generate_report
}

# Run main function
main "$@"