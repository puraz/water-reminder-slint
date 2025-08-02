use std::fs;
use std::path::PathBuf;
use chrono::{Local, NaiveDate};
use crate::models::{AppState, DailyStats, UserSettings};

pub struct DataManager {
    data_dir: PathBuf,
}

impl DataManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = dirs::data_dir()
            .ok_or("无法获取数据目录")?
            .join("water-reminder");
        
        fs::create_dir_all(&data_dir)?;
        
        Ok(Self { data_dir })
    }

    pub fn load_settings(&self) -> UserSettings {
        let settings_path = self.data_dir.join("settings.json");
        if let Ok(content) = fs::read_to_string(settings_path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            UserSettings::default()
        }
    }

    pub fn save_settings(&self, settings: &UserSettings) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = self.data_dir.join("settings.json");
        let content = serde_json::to_string_pretty(settings)?;
        fs::write(settings_path, content)?;
        Ok(())
    }

    pub fn load_daily_stats(&self, date: NaiveDate) -> Option<DailyStats> {
        let file_path = self.data_dir.join(format!("stats_{}.json", date.format("%Y-%m-%d")));
        if let Ok(content) = fs::read_to_string(file_path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn save_daily_stats(&self, stats: &DailyStats) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.data_dir.join(format!("stats_{}.json", stats.date.format("%Y-%m-%d")));
        let content = serde_json::to_string_pretty(stats)?;
        fs::write(file_path, content)?;
        Ok(())
    }

    pub fn load_app_state(&self) -> AppState {
        let settings = self.load_settings();
        let today = Local::now().date_naive();
        
        let today_stats = self.load_daily_stats(today)
            .unwrap_or_else(|| DailyStats {
                date: today,
                total_amount: 0,
                goal_amount: settings.daily_goal,
                records: Vec::new(),
                goal_achieved: false,
            });

        let mut weekly_stats = Vec::new();
        // 加载过去6天的数据（不包括今天）
        for i in 1..=6 {
            let date = today - chrono::Duration::days(i);
            if let Some(stats) = self.load_daily_stats(date) {
                weekly_stats.push(stats);
            }
        }
        // 按日期排序，最旧的在前面
        weekly_stats.sort_by(|a, b| a.date.cmp(&b.date));

        let last_record_id = today_stats.records
            .iter()
            .map(|r| r.id)
            .max()
            .unwrap_or(0);

        AppState {
            settings,
            today_stats,
            weekly_stats,
            last_record_id,
        }
    }

    pub fn save_app_state(&self, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
        self.save_settings(&state.settings)?;
        self.save_daily_stats(&state.today_stats)?;
        Ok(())
    }
}