# 💧 Water Reminder - 喝水提醒应用

基于 Rust + Slint 构建的现代化跨平台桌面饮水记录和提醒应用。

## 功能特性

### ✨ 核心功能
- **直观的饮水记录**：快速记录100ml、200ml、300ml、500ml或自定义量的饮水
- **智能进度跟踪**：实时显示今日饮水量和完成百分比
- **目标管理**：可自定义每日饮水目标（1000ml-5000ml）
- **历史记录**：详细的饮水记录时间轴
- **撤销功能**：一键撤销最近的记录

### 📊 数据统计
- **一周统计**：查看过去7天的饮水趋势
- **成就系统**：连续达标天数和完成率统计
- **平均数据**：每日平均饮水量分析

### 🔔 智能提醒
- **定时提醒**：可设置15分钟到4小时的提醒间隔
- **系统通知**：原生系统通知提醒喝水
- **达标庆祝**：完成每日目标时的成就通知

### 💾 数据持久化
- **本地存储**：所有数据安全保存在本地
- **设置同步**：用户偏好和配置自动保存
- **跨会话保持**：重启应用时恢复所有数据

## 界面设计

### 🏠 主页
- 高颜值进度卡片显示今日饮水量
- 2x2网格快速添加按钮
- 滚动式饮水记录历史
- 自定义量输入和撤销功能

### 📈 统计页面
- 本周平均饮水量卡片
- 7天饮水量趋势图（开发中）
- 成就徽章展示区域

### ⚙️ 设置页面
- 每日目标调整（±100ml递增）
- 提醒开关和间隔设置
- 应用关于信息

## 技术栈

- **后端**: Rust
- **UI框架**: Slint 1.8
- **数据存储**: JSON文件
- **时间处理**: Chrono
- **序列化**: Serde
- **通知**: notify-rust
- **异步**: Tokio

## 系统要求

- **操作系统**: Windows, macOS, Linux
- **内存**: 最小50MB运行时内存
- **存储**: 约10MB安装空间
- **图形**: 支持现代图形API

## 快速开始

### 安装依赖
```bash
# 确保已安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone <repository-url>
cd water-reminder3
```

### 构建运行
```bash
# 开发环境运行
cargo run

# 发布构建
cargo build --release
```

### 使用说明

1. **首次启动**：应用会创建默认的2000ml每日目标
2. **记录饮水**：点击100ml、200ml、300ml、500ml按钮或自定义量
3. **查看进度**：主页卡片实时显示完成百分比
4. **设置目标**：在设置页面调整每日饮水目标
5. **启用提醒**：在设置中开启定时提醒功能

## 数据存储

应用数据存储在系统标准目录：
- **Windows**: `%APPDATA%/water-reminder/`
- **macOS**: `~/Library/Application Support/water-reminder/`
- **Linux**: `~/.local/share/water-reminder/`

文件结构：
```
water-reminder/
├── settings.json           # 用户设置
├── stats_2024-08-02.json  # 每日数据（按日期）
└── stats_2024-08-01.json
```

## 项目结构

```
src/
├── main.rs                 # 主程序入口
├── models/
│   └── mod.rs             # 数据模型定义
└── utils/
    ├── mod.rs             # 工具模块
    ├── data.rs            # 数据管理
    └── notification.rs    # 通知管理
ui/
└── app.slint             # UI界面定义
```

## 开发规范

### 代码质量
- 遵循Rust最佳实践
- 包含详细的中文注释
- 模块化清晰的项目结构
- 无编译警告的代码

### UI设计原则
- 现代化Material Design风格
- 400x600像素高比宽比窗口
- 渐变色和圆角设计元素
- 流畅的动画过渡效果

## 贡献指南

1. Fork 本项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 联系方式

- 项目主页: [GitHub Repository]
- 问题反馈: [Issues](issues)
- 功能建议: [Discussions](discussions)

---

**享受健康的饮水习惯！** 💧✨