# GT (Git Toolkit) 安装指南

GT是一个用Rust编写的现代Git工作流工具，旨在替代传统的bash脚本，提供更快、更可靠的Git操作体验。

## 🎯 快速安装（推荐）

### 方法1: 使用安装脚本（最简单）

```bash
# 1. 克隆项目
git clone https://github.com/lyzno1/gt.git
cd gt

# 2. 运行安装脚本（自动完成所有步骤）
./install.sh
```

安装脚本会自动：
- ✅ 检查Rust环境
- ✅ 编译release版本
- ✅ 安装到 `~/.local/bin/gt`
- ✅ 配置PATH环境变量
- ✅ 验证安装

### 方法2: 使用Makefile

```bash
# 克隆并进入项目
git clone https://github.com/lyzno1/gt.git
cd gt

# 使用make安装
make install
```

### 方法3: 手动安装

```bash
# 1. 克隆项目
git clone https://github.com/lyzno1/gt.git
cd gt

# 2. 编译
cargo build --release

# 3. 复制到本地bin目录
mkdir -p ~/.local/bin
cp target/release/gt ~/.local/bin/
chmod +x ~/.local/bin/gt

# 4. 添加到PATH（如果还没有）
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## ✅ 验证安装

安装完成后，验证GT是否正常工作：

```bash
# 检查版本
gt --version

# 查看帮助
gt --help

# 测试命令（在git仓库中）
gt status
```

预期输出：
```
gt 0.1.0
```

## 🔧 系统要求

### 必需依赖
- **Rust** 1.70+（包含cargo）
- **Git** 2.0+

### 安装Rust（如果没有）
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 支持的操作系统
- ✅ macOS (Intel & Apple Silicon)
- ✅ Linux (x86_64)
- ✅ Windows (via WSL recommended)

## 🌟 使用方法

### 基本工作流
```bash
# 开始新功能
gt start feature/new-feature

# 保存进度
gt save -m "Add new feature"

# 同步主分支
gt update

# 提交工作成果
gt ship --pr

# 清理分支
gt clean feature/new-feature
```

### Git命令增强版本
GT也提供了增强版的Git命令：
```bash
gt status        # 增强版git status
gt add .         # 增强版git add
gt commit -m ""  # 增强版git commit
gt push          # 增强版git push
```

## 🗑️ 卸载

如果需要卸载GT：

```bash
# 使用make卸载
make uninstall

# 或手动删除
rm ~/.local/bin/gt

# 从PATH中移除（如果需要）
# 编辑 ~/.zshrc 或 ~/.bashrc，移除相关行
```

## 🔄 更新

更新到最新版本：

```bash
cd gt
git pull origin main
cargo build --release
cp target/release/gt ~/.local/bin/
```

## 🚨 故障排除

### 问题1: `gt: command not found`
**解决方案:**
```bash
# 检查PATH
echo $PATH | grep -o "$HOME/.local/bin"

# 如果没有输出，添加到PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 问题2: 编译失败
**解决方案:**
```bash
# 更新Rust
rustup update

# 清理并重新编译
cargo clean
cargo build --release
```

### 问题3: 权限错误
**解决方案:**
```bash
# 确保二进制文件有执行权限
chmod +x ~/.local/bin/gt
```

## 📞 获取帮助

- **GitHub Issues**: https://github.com/lyzno1/gt/issues
- **文档**: https://github.com/lyzno1/gt
- **命令帮助**: `gt --help` 或 `gt <command> --help`

## 🎉 开始使用

安装完成！现在你可以在任何Git仓库中使用`gt`命令，享受现代化的Git工作流体验！

```bash
# 第一次使用
cd /path/to/your/git/repo
gt status
``` 