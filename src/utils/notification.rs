use std::time::Duration;
use tokio::time;
use std::sync::{Arc, Mutex};

#[cfg(not(target_os = "linux"))]
use notify_rust::{Notification, Timeout};

#[cfg(target_os = "linux")]
use std::process::Command;

#[derive(Clone)]
pub struct NotificationManager {
    enabled: Arc<Mutex<bool>>,
    interval: Arc<Mutex<u32>>,
}

impl NotificationManager {
    pub fn new(enabled: bool) -> Self {
        Self { 
            enabled: Arc::new(Mutex::new(enabled)),
            interval: Arc::new(Mutex::new(15)), // 默认15分钟
        }
    }
    
    pub fn update_settings(&self, enabled: bool, interval: u32) {
        *self.enabled.lock().unwrap() = enabled;
        *self.interval.lock().unwrap() = interval;
    }

    fn activate_window(&self) {
        println!("尝试激活应用程序窗口...");
        
        #[cfg(target_os = "linux")]
        {
            // 策略1: 使用wmctrl激活窗口
            let window_titles = [
                "💧 Water Reminder - 喝水提醒",
                "Water Reminder - 喝水提醒", 
                "Water Reminder",
                "water-reminder"
            ];
            
            let mut success = false;
            
            // 1. 尝试使用wmctrl
            for title in &window_titles {
                println!("尝试使用wmctrl激活窗口: '{}'", title);
                match std::process::Command::new("wmctrl")
                    .arg("-a")
                    .arg(title)
                    .output() {
                    Ok(output) => {
                        if output.status.success() {
                            println!("wmctrl成功激活窗口: '{}'", title);
                            success = true;
                            break;
                        } else if !output.stderr.is_empty() {
                            println!("wmctrl失败: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    },
                    Err(e) => {
                        println!("无法执行wmctrl: {}", e);
                        break; // 如果wmctrl不存在，不要继续尝试其他标题
                    }
                }
            }
            
            if !success {
                println!("wmctrl失败，尝试xdotool...");
                
                // 2. 尝试使用xdotool
                for title in &window_titles {
                    match std::process::Command::new("xdotool")
                        .arg("search")
                        .arg("--name")
                        .arg(title)
                        .arg("windowactivate")
                        .output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("xdotool成功激活窗口: '{}'", title);
                                success = true;
                                break;
                            }
                        },
                        Err(_) => continue,
                    }
                }
            }
            
            if !success {
                println!("xdotool失败，尝试xwininfo查找...");
                
                // 3. 使用xwininfo查找确切的窗口ID
                match std::process::Command::new("xwininfo")
                    .arg("-tree")
                    .arg("-root")
                    .output() {
                    Ok(output) => {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        for line in output_str.lines() {
                            // 查找包含我们应用标题的行
                            if (line.contains("Water Reminder") || line.contains("water-reminder")) 
                                && !line.contains("(has no name)") {
                                if let Some(window_id) = line.split_whitespace().nth(0) {
                                    if window_id.starts_with("0x") {
                                        println!("找到窗口ID: {} -> {}", window_id, line.trim());
                                        
                                        // 使用xdotool激活特定窗口ID
                                        match std::process::Command::new("xdotool")
                                            .arg("windowactivate")
                                            .arg(window_id)
                                            .output() {
                                            Ok(output) => {
                                                if output.status.success() {
                                                    println!("成功通过窗口ID激活窗口");
                                                    success = true;
                                                    break;
                                                }
                                            },
                                            Err(e) => println!("xdotool激活失败: {}", e),
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => println!("xwininfo执行失败: {}", e),
                }
            }
            
            if !success {
                println!("所有窗口激活方法都失败了，窗口可能已关闭或不可见");
                println!("提示：请确保应用程序仍在运行，或手动点击任务栏图标");
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows下的窗口激活
            println!("Windows平台 - 尝试激活窗口...");
            
            unsafe {
                use std::ffi::OsStr;
                use std::os::windows::ffi::OsStrExt;
                use winapi::um::winuser::{
                    FindWindowW, SetForegroundWindow, ShowWindow, IsIconic, 
                    SW_RESTORE, SW_SHOW, GetWindowTextW, EnumWindows
                };
                use winapi::shared::windef::HWND;
                use winapi::shared::minwindef::{BOOL, LPARAM};
                
                let window_titles = [
                    "💧 Water Reminder - 喝水提醒",
                    "Water Reminder - 喝水提醒", 
                    "Water Reminder",
                    "water-reminder"
                ];
                
                let mut window_found = false;
                
                // 1. 直接通过窗口标题查找
                for title in &window_titles {
                    let wide_title: Vec<u16> = OsStr::new(title)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect();
                    
                    let hwnd = FindWindowW(std::ptr::null(), wide_title.as_ptr());
                    
                    if !hwnd.is_null() {
                        println!("Windows: 找到窗口句柄，标题: '{}'", title);
                        
                        // 如果窗口最小化，先恢复
                        if IsIconic(hwnd) != 0 {
                            println!("Windows: 窗口已最小化，正在恢复...");
                            ShowWindow(hwnd, SW_RESTORE);
                        } else {
                            ShowWindow(hwnd, SW_SHOW);
                        }
                        
                        // 将窗口带到前台
                        if SetForegroundWindow(hwnd) != 0 {
                            println!("Windows: 成功激活窗口");
                            window_found = true;
                            break;
                        } else {
                            println!("Windows: 激活窗口失败");
                        }
                    }
                }
                
                // 2. 如果直接查找失败，枚举所有窗口
                if !window_found {
                    println!("Windows: 直接查找失败，枚举所有窗口...");
                    
                    extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
                        unsafe {
                            let mut window_text = [0u16; 256];
                            let len = GetWindowTextW(hwnd, window_text.as_mut_ptr(), 256);
                            
                            if len > 0 {
                                let title = String::from_utf16_lossy(&window_text[..len as usize]);
                                if title.contains("Water Reminder") || title.contains("water-reminder") {
                                    println!("Windows: 枚举找到匹配窗口: '{}'", title);
                                    
                                    // 恢复并激活窗口
                                    if IsIconic(hwnd) != 0 {
                                        ShowWindow(hwnd, SW_RESTORE);
                                    } else {
                                        ShowWindow(hwnd, SW_SHOW);
                                    }
                                    
                                    if SetForegroundWindow(hwnd) != 0 {
                                        println!("Windows: 通过枚举成功激活窗口");
                                        return 0; // 停止枚举
                                    }
                                }
                            }
                            1 // 继续枚举
                        }
                    }
                    
                    EnumWindows(Some(enum_windows_proc), 0);
                }
                
                if !window_found {
                    println!("Windows: 所有窗口激活方法都失败了");
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS下的窗口激活
            println!("macOS平台 - 尝试激活窗口...");
            
            // 方法1: 使用AppleScript激活应用程序
            let app_names = [
                "Water Reminder",
                "water-reminder"
            ];
            
            let mut success = false;
            
            for app_name in &app_names {
                let script = format!(
                    r#"tell application "System Events"
                        set appName to "{}"
                        if exists (processes whose name is appName) then
                            tell application appName to activate
                            return true
                        end if
                        return false
                    end tell"#, 
                    app_name
                );
                
                println!("macOS: 尝试使用AppleScript激活应用: '{}'", app_name);
                
                match std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(&script)
                    .output() {
                    Ok(output) => {
                        if output.status.success() {
                            let result = String::from_utf8_lossy(&output.stdout).trim();
                            if result == "true" {
                                println!("macOS: AppleScript成功激活应用");
                                success = true;
                                break;
                            }
                        } else {
                            println!("macOS: AppleScript执行失败: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    },
                    Err(e) => {
                        println!("macOS: 无法执行osascript: {}", e);
                        break;
                    }
                }
            }
            
            // 方法2: 如果AppleScript失败，尝试使用open命令
            if !success {
                println!("macOS: AppleScript失败，尝试使用open命令...");
                
                // 尝试通过bundle identifier激活（如果应用是打包的）
                match std::process::Command::new("open")
                    .arg("-a")
                    .arg("Water Reminder")
                    .output() {
                    Ok(output) => {
                        if output.status.success() {
                            println!("macOS: open命令成功激活应用");
                            success = true;
                        } else {
                            println!("macOS: open命令失败: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    },
                    Err(e) => {
                        println!("macOS: 无法执行open命令: {}", e);
                    }
                }
            }
            
            // 方法3: 使用Accessibility API (需要权限)
            if !success {
                println!("macOS: 尝试使用Accessibility API...");
                
                let script = r#"
                tell application "System Events"
                    set frontApp to name of first application process whose frontmost is true
                    set waterApp to first application process whose name contains "water" or name contains "Water"
                    if waterApp exists then
                        set frontmost of waterApp to true
                        return true
                    end if
                    return false
                end tell
                "#;
                
                match std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(script)
                    .output() {
                    Ok(output) => {
                        if output.status.success() {
                            let result = String::from_utf8_lossy(&output.stdout).trim();
                            if result == "true" {
                                println!("macOS: Accessibility API成功激活窗口");
                                success = true;
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
            
            if !success {
                println!("macOS: 所有窗口激活方法都失败了");
                println!("提示：确保应用程序正在运行，或检查辅助功能权限");
            }
        }
    }

    pub fn show_water_reminder(&self) -> Result<(), Box<dyn std::error::Error>> {
        let enabled = *self.enabled.lock().unwrap();
        if !enabled {
            return Ok(());
        }

        println!("正在发送水提醒通知...");
        
        // 尝试激活窗口
        self.activate_window();
        
        #[cfg(target_os = "linux")]
        {
            // Linux: 使用notify-send原生命令
            let output = Command::new("notify-send")
                .arg("💧 喝水提醒")
                .arg("该喝水了！保持良好的饮水习惯对健康很重要。")
                .arg("--urgency=normal")
                .arg("--expire-time=10000") // 10秒
                .arg("--icon=dialog-information")
                .arg("--app-name=Water Reminder")
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("通知发送成功 (notify-send)");
                        Ok(())
                    } else {
                        let error_msg = String::from_utf8_lossy(&result.stderr);
                        eprintln!("notify-send失败: {}", error_msg);
                        Err(format!("notify-send failed: {}", error_msg).into())
                    }
                },
                Err(e) => {
                    eprintln!("无法启动notify-send: {}", e);
                    Err(Box::new(e))
                }
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Windows和macOS: 使用notify-rust
            let mut notification = Notification::new();
            notification
                .summary("💧 喝水提醒")
                .body("该喝水了！保持良好的饮水习惯对健康很重要。")
                .appname("Water Reminder")
                .timeout(Timeout::Milliseconds(10000)); // 10秒

            #[cfg(target_os = "macos")]
            {
                notification.subtitle("Water Reminder");
            }

            match notification.show() {
                Ok(_handle) => {
                    println!("通知发送成功 (notify-rust)");
                    Ok(())
                },
                Err(e) => {
                    eprintln!("通知发送失败: {}", e);
                    Err(Box::new(e))
                }
            }
        }
    }

    pub fn show_goal_achieved(&self) -> Result<(), Box<dyn std::error::Error>> {
        let enabled = *self.enabled.lock().unwrap();
        if !enabled {
            return Ok(());
        }

        println!("正在发送目标达成通知...");

        #[cfg(target_os = "linux")]
        {
            // Linux: 使用notify-send原生命令
            let output = Command::new("notify-send")
                .arg("🎉 目标达成！")
                .arg("恭喜！您今天已经完成了饮水目标！")
                .arg("--urgency=normal")
                .arg("--expire-time=10000") // 10秒
                .arg("--icon=dialog-information")
                .arg("--app-name=Water Reminder")
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("目标达成通知发送成功 (notify-send)");
                        Ok(())
                    } else {
                        let error_msg = String::from_utf8_lossy(&result.stderr);
                        eprintln!("notify-send失败: {}", error_msg);
                        Err(format!("notify-send failed: {}", error_msg).into())
                    }
                },
                Err(e) => {
                    eprintln!("无法启动notify-send: {}", e);
                    Err(Box::new(e))
                }
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Windows和macOS: 使用notify-rust
            let mut notification = Notification::new();
            notification
                .summary("🎉 目标达成！")
                .body("恭喜！您今天已经完成了饮水目标！")
                .appname("Water Reminder")
                .timeout(Timeout::Milliseconds(10000)); // 10秒

            #[cfg(target_os = "macos")]
            {
                notification.subtitle("Water Reminder");
            }

            match notification.show() {
                Ok(_handle) => {
                    println!("目标达成通知发送成功 (notify-rust)");
                    Ok(())
                },
                Err(e) => {
                    eprintln!("目标达成通知发送失败: {}", e);
                    Err(Box::new(e))
                }
            }
        }
    }

    pub async fn start_reminder_loop(&self) {
        loop {
            let (enabled, interval) = {
                let enabled = *self.enabled.lock().unwrap();
                let interval = *self.interval.lock().unwrap();
                (enabled, interval)
            };
            
            if !enabled || interval == 0 {
                // 如果禁用了提醒或间隔为0，等待1秒后重新检查
                time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            
            // 等待设定的间隔时间
            time::sleep(Duration::from_secs(interval as u64 * 60)).await;
            
            // 发送提醒
            if let Err(e) = self.show_water_reminder() {
                eprintln!("发送通知失败: {}", e);
            }
        }
    }
}

