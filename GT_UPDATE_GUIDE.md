# 🔄 GT 自更新指南

## 📋 概述

GT 现在支持自更新功能！你不再需要手动使用 `cargo` 命令来更新 GT，而是可以使用 GT 自己的命令来完成更新。

## 🚀 更新方式

### 1. 使用内置命令（推荐）

```bash
# 检查是否有可用更新
gt update-self --check

# 更新到最新版本（会提示确认）
gt update-self

# 直接更新，跳过确认
gt update-self -y

# 预演模式，查看会执行什么操作
gt --dry-run update-self
```

### 2. 使用更新脚本

```bash
# 运行项目根目录下的更新脚本
./update_gt.sh
```

## 🔍 更新过程

GT 的自更新过程包括以下步骤：

1. **📥 拉取最新源码**
   - 使用 `gt update` 命令同步最新代码
   - 自动处理 stash 和 rebase

2. **🔨 重新编译**
   - 使用 `cargo install --path . --force` 重新编译
   - 生成优化的 release 版本

3. **📦 安装到系统**
   - 安装到 `~/.cargo/bin/gt`
   - 自动复制到 `~/.local/bin/gt`（如果需要）

4. **✅ 验证安装**
   - 检查新版本是否正常工作
   - 显示版本信息

## 🛡️ 安全特性

- **预演模式**：使用 `--dry-run` 查看会执行什么操作
- **确认提示**：默认会询问是否确定更新
- **错误处理**：如果更新失败，会保留原版本
- **网络检查**：使用与原始 gw 相同的网络处理方式

## 📝 使用示例

### 日常更新检查
```bash
# 每天检查一次更新
gt update-self --check
```

### 快速更新
```bash
# 直接更新，适合自动化脚本
gt update-self -y
```

### 安全更新
```bash
# 先预演，再确认
gt --dry-run update-self
gt update-self
```

## 🔧 故障排除

### 问题：更新后 gt 命令不可用
**解决方案：**
```bash
# 手动复制到正确路径
cp ~/.cargo/bin/gt ~/.local/bin/gt

# 或者重新安装
cd /path/to/gt/project
cargo install --path . --force
```

### 问题：网络连接失败
**解决方案：**
```bash
# 检查 git 连接
git pull origin main

# 如果 git 正常，重试 GT 更新
gt update-self
```

### 问题：编译失败
**解决方案：**
```bash
# 手动编译检查错误
cargo build --release

# 清理后重试
cargo clean
gt update-self
```

## 🎯 最佳实践

1. **定期检查更新**
   ```bash
   # 建议每周检查一次
   gt update-self --check
   ```

2. **更新前备份重要工作**
   ```bash
   # 确保当前工作已保存
   gt status
   gt save -m "更新前保存"
   ```

3. **使用预演模式**
   ```bash
   # 重要更新前先预演
   gt --dry-run update-self
   ```

4. **验证更新结果**
   ```bash
   # 更新后测试基本功能
   gt status
   gt --help
   ```

## 🔄 与原始 gw 的对比

| 功能 | 原始 gw | GT |
|------|---------|-----|
| 更新方式 | 手动 git pull | `gt update-self` |
| 网络处理 | 系统 git 命令 | 系统 git 命令 ✅ |
| 预演模式 | 支持 | 支持 ✅ |
| 错误处理 | 基本 | 增强 ✅ |
| 用户体验 | 命令行 | 彩色输出 + 进度提示 ✅ |

## 📚 相关命令

- `gt status` - 检查仓库状态
- `gt update` - 同步分支到最新状态
- `gt --help` - 查看所有可用命令
- `gt update-self --help` - 查看更新命令帮助

---

🎉 **现在你可以完全使用 GT 自己来管理更新了！不再需要记住复杂的 cargo 命令。** 