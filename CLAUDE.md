# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a modern cross-platform desktop water reminder application built with Rust and Slint UI framework. The app helps users track daily water intake, set hydration goals, and receive reminder notifications.

## Development Commands

### Build and Run
```bash
# Development build and run
cargo run

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies
- The project uses Slint 1.8 for the UI framework
- Build process requires `slint-build` crate to compile `.slint` files
- No additional setup required beyond standard Rust toolchain

## Architecture Overview

### Core Components

**UI Layer (`ui/app.slint`)**
- Modern Material Design interface with 400x600px window
- Three-page navigation: 主页 (Home), 统计 (Stats), 设置 (Settings)
- Reactive UI with global `AppState` managing all application state
- Custom components: `WaterButton`, `NavBar`, `AchievementBadge`, `SettingGroup`

**Backend Architecture (`src/`)**
- `main.rs`: Application entry point, UI initialization, and callback setup
- `models/mod.rs`: Core data structures (`AppState`, `DailyStats`, `UserSettings`, `WaterRecord`)
- `utils/data.rs`: File-based persistence using JSON in platform-specific directories
- `utils/notification.rs`: Cross-platform system notifications using `notify-rust`

### Data Flow

1. **State Management**: `AppState` struct holds all application state including settings, today's stats, and weekly history
2. **Persistence**: Data automatically saved on every user action via `DataManager`
3. **UI Updates**: Slint's reactive system updates UI when global `AppState` properties change
4. **Notifications**: Asynchronous notification system for reminders and goal achievements

### Key Design Patterns

**Data Storage Structure:**
```
~/.local/share/water-reminder/  (Linux)
├── settings.json              # User preferences and configuration
├── stats_2024-08-02.json     # Daily records (one file per date)
└── stats_2024-08-01.json
```

**Event Handling:**
- UI callbacks in `main.rs` handle user interactions
- Each callback updates `AppState`, saves to disk, and refreshes UI
- Pattern: borrow state → modify → save → update UI

**Testing:**
- Unit tests included in `models/mod.rs` for core business logic
- Tests cover water record management, progress calculation, and goal achievement

## Development Notes

### Slint Integration
- UI components defined in `.slint` files are compiled at build time
- Global `AppState` provides two-way data binding between Rust and UI
- Callback functions bridge UI events to Rust business logic

### Async Considerations
- Main application runs synchronously with Slint's event loop
- Notification reminders use Tokio for async timing (though not fully implemented in reminder loop)
- File I/O operations are synchronous and fast for JSON data

### Localization
- Application is primarily in Chinese with some English technical terms
- All user-facing text in UI components is in Chinese
- Error messages and debug output in Chinese

### Error Handling
- Uses `Result<T, Box<dyn std::error::Error>>` pattern throughout
- Graceful fallbacks: missing data files create defaults, failed notifications don't crash app
- Data corruption handled by falling back to default values