#!/bin/bash
# GT (Git Toolkit) 安装脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
check_dependencies() {
    print_info "检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo 未找到，请先安装 Rust："
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    if ! command -v git &> /dev/null; then
        print_error "Git 未找到，请先安装 Git"
        exit 1
    fi
    
    print_success "依赖检查通过"
}

# 构建项目
build_project() {
    print_info "构建 GT..."
    cargo build --release
    print_success "构建完成"
}

# 安装到本地
install_binary() {
    print_info "安装 GT 到本地..."
    
    # 创建本地bin目录
    mkdir -p ~/.local/bin
    
    # 复制二进制文件
    cp target/release/gt ~/.local/bin/
    chmod +x ~/.local/bin/gt
    
    print_success "GT 已安装到 ~/.local/bin/gt"
}

# 检查PATH配置
check_path() {
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        print_warning "~/.local/bin 不在 PATH 中"
        
        # 检测shell类型
        if [[ "$SHELL" == *"zsh"* ]]; then
            SHELL_RC="$HOME/.zshrc"
        elif [[ "$SHELL" == *"bash"* ]]; then
            SHELL_RC="$HOME/.bashrc"
        else
            SHELL_RC="$HOME/.profile"
        fi
        
        print_info "将添加到 $SHELL_RC"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
        print_success "PATH 配置已添加到 $SHELL_RC"
        print_info "请运行 'source $SHELL_RC' 或重新打开终端"
    else
        print_success "PATH 配置正确"
    fi
}

# 验证安装
verify_installation() {
    if ~/.local/bin/gt --version > /dev/null 2>&1; then
        print_success "GT 安装成功！"
        print_info "版本信息："
        ~/.local/bin/gt --version
        print_info ""
        print_info "现在你可以使用以下命令："
        print_info "  gt --help          # 查看帮助"
        print_info "  gt status          # 查看仓库状态"
        print_info "  gt start <branch>  # 开始新分支"
        print_info "  gt save -m 'msg'   # 保存变更"
        print_info "  gt ship            # 提交工作"
    else
        print_error "安装验证失败"
        exit 1
    fi
}

# 主函数
main() {
    echo -e "${BLUE}"
    echo "========================================"
    echo "    GT (Git Toolkit) 安装程序"
    echo "========================================"
    echo -e "${NC}"
    
    check_dependencies
    build_project
    install_binary
    check_path
    verify_installation
    
    echo ""
    print_success "安装完成！享受 GT 带来的高效 Git 工作流！"
}

# 运行主函数
main "$@" 