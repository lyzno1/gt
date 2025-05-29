# GT (Git Toolkit) Makefile

.PHONY: help build test install uninstall clean fmt clippy check dev release

# 默认目标
help:
	@echo "GT (Git Toolkit) - 可用命令:"
	@echo ""
	@echo "开发相关:"
	@echo "  make build     # 编译项目"
	@echo "  make test      # 运行测试"
	@echo "  make check     # 检查代码格式和lint"
	@echo "  make fmt       # 格式化代码"
	@echo "  make clippy    # 运行clippy检查"
	@echo "  make dev       # 开发模式构建"
	@echo ""
	@echo "安装相关:"
	@echo "  make install   # 安装到本地"
	@echo "  make uninstall # 卸载"
	@echo ""
	@echo "发布相关:"
	@echo "  make release   # 发布构建"
	@echo "  make clean     # 清理构建文件"

# 开发相关
build:
	cargo build

dev:
	cargo build

test:
	cargo test

check: fmt clippy test

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# 安装相关
install:
	@echo "运行安装脚本..."
	./install.sh

uninstall:
	@echo "卸载 GT..."
	rm -f ~/.local/bin/gt
	@echo "GT 已卸载"

# 发布相关
release:
	cargo build --release

clean:
	cargo clean

# 快速开发测试
run:
	cargo run --

# 运行特定命令进行测试
run-help:
	cargo run -- --help

run-version:
	cargo run -- --version

# 格式化并检查
format:
	cargo fmt --all 