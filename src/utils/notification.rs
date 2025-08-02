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

    pub fn show_water_reminder(&self) -> Result<(), Box<dyn std::error::Error>> {
        let enabled = *self.enabled.lock().unwrap();
        if !enabled {
            return Ok(());
        }

        println!("正在发送水提醒通知...");
        
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

