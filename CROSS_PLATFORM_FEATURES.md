# 跨平台窗口激活功能

本应用现已支持Linux、Windows和macOS三大平台的窗口激活功能。

## 🐧 Linux平台

### 支持的工具
- **wmctrl** - 主要的窗口管理工具
- **xdotool** - 窗口搜索和激活工具
- **xwininfo** - 窗口信息查询工具

### 激活策略
1. 使用wmctrl通过窗口标题直接激活
2. 使用xdotool搜索并激活窗口
3. 使用xwininfo枚举所有窗口，通过窗口ID激活

### 安装依赖
```bash
# Ubuntu/Debian
sudo apt install wmctrl xdotool x11-utils

# Fedora/RHEL
sudo dnf install wmctrl xdotool xorg-x11-utils

# Arch Linux
sudo pacman -S wmctrl xdotool xorg-xwininfo
```

## 🪟 Windows平台

### 技术实现
- 使用Windows API (winapi crate)
- 直接调用系统函数进行窗口操作

### 功能特性
- **FindWindowW** - 通过窗口标题查找窗口
- **EnumWindows** - 枚举所有窗口进行模糊匹配
- **SetForegroundWindow** - 将窗口带到前台
- **ShowWindow** - 显示或恢复窗口
- **IsIconic** - 检测窗口是否最小化

### 激活流程
1. 尝试通过精确窗口标题查找
2. 如果失败，枚举所有窗口进行模糊匹配
3. 检测窗口状态（最小化/正常）
4. 恢复窗口并激活

## 🍎 macOS平台

### 技术实现
- AppleScript自动化
- 系统open命令
- Accessibility API

### 功能特性
- **AppleScript应用激活** - 通过应用名称激活
- **open命令** - 系统级应用启动/激活
- **Accessibility API** - 高级窗口管理（需要权限）

### 激活策略
1. 使用AppleScript查找并激活应用进程
2. 使用open命令激活应用
3. 使用Accessibility API进行精确窗口控制

### 权限要求
macOS可能需要授予以下权限：
- **辅助功能权限** - 用于Accessibility API
- **自动化权限** - 用于AppleScript执行

## 🔧 使用方式

### 自动激活
当提醒时间到达时，应用会自动：
1. 发送系统通知
2. 尝试激活应用程序窗口
3. 将窗口带到前台

### 调试信息
应用提供详细的调试输出，包括：
- 尝试的激活方法
- 成功/失败状态
- 错误信息和建议

## 📋 兼容性说明

### Linux
- 支持所有主流桌面环境（GNOME、KDE、XFCE、Unity等）
- 需要X11窗口系统（Wayland支持有限）

### Windows
- 支持Windows 7及以上版本
- 无需额外依赖，使用系统内置API

### macOS
- 支持macOS 10.10及以上版本
- 可能需要用户授予相关权限

## 🚀 构建说明

### 跨平台构建
```bash
# 当前平台
cargo build

# 指定目标平台
cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-apple-darwin
cargo build --target x86_64-unknown-linux-gnu
```

### 依赖管理
- Linux: 运行时依赖系统工具
- Windows: 编译时包含winapi库
- macOS: 运行时使用系统命令

## 🔍 故障排除

### Linux
- 确保安装了wmctrl、xdotool等工具
- 检查X11环境变量设置
- 验证窗口管理器兼容性

### Windows
- 检查Windows版本兼容性
- 确认应用程序正在运行
- 验证窗口标题是否正确

### macOS
- 检查系统权限设置
- 确认AppleScript支持
- 验证应用程序在Dock中可见

## 📝 开发说明

### 扩展支持
要添加新的窗口激活方法：
1. 在相应的平台条件编译块中添加代码
2. 遵循现有的错误处理模式
3. 添加详细的调试输出
4. 测试各种窗口状态

### 测试建议
- 测试窗口最小化状态
- 测试窗口隐藏状态
- 测试多显示器环境
- 验证权限要求