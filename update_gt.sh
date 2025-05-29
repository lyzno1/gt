#!/bin/bash

# GT 自更新脚本
# 使用 GT 自己来更新源码，然后重新编译安装

set -e

echo "🔄 开始更新 GT..."

# 1. 使用 GT 自己来同步最新代码
echo "📥 使用 GT 拉取最新更新..."
gt update

# 2. 重新编译和安装
echo "🔨 重新编译 GT..."
cargo build --release

# 3. 安装到系统路径
echo "📦 安装新版本..."
cargo install --path . --force

# 4. 复制到正确的路径（如果需要）
if [ -f ~/.cargo/bin/gt ] && [ -d ~/.local/bin ]; then
    echo "📋 复制到 ~/.local/bin..."
    cp ~/.cargo/bin/gt ~/.local/bin/gt
fi

# 5. 验证安装
echo "✅ 验证安装..."
gt --version || echo "GT 版本信息："

echo "🎉 GT 更新完成！"
echo ""
echo "📋 更新后的功能："
echo "  gt status    # 检查状态"
echo "  gt start     # 开始新分支"
echo "  gt save      # 保存工作"
echo "  gt update    # 同步更新"
echo "  gt ship      # 提交成果" 