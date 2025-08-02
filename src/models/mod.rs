use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterRecord {
    pub id: u64,
    pub amount: u32, // ml
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: NaiveDate,
    pub total_amount: u32,
    pub goal_amount: u32,
    pub records: Vec<WaterRecord>,
    pub goal_achieved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub daily_goal: u32, // ml
    pub reminder_interval: u32, // minutes
    pub reminder_enabled: bool,
    pub start_time: String, // "07:00"
    pub end_time: String,   // "22:00"
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            daily_goal: 2000,
            reminder_interval: 15, // 默认15分钟
            reminder_enabled: true,
            start_time: "07:00".to_string(),
            end_time: "22:00".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub settings: UserSettings,
    pub today_stats: DailyStats,
    pub weekly_stats: Vec<DailyStats>,
    pub last_record_id: u64,
}

impl AppState {
    pub fn new() -> Self {
        let today = Local::now().date_naive();
        Self {
            settings: UserSettings::default(),
            today_stats: DailyStats {
                date: today,
                total_amount: 0,
                goal_amount: 2000,
                records: Vec::new(),
                goal_achieved: false,
            },
            weekly_stats: Vec::new(),
            last_record_id: 0,
        }
    }

    pub fn add_water_record(&mut self, amount: u32) {
        self.last_record_id += 1;
        let record = WaterRecord {
            id: self.last_record_id,
            amount,
            timestamp: Local::now(),
        };

        self.today_stats.records.push(record);
        self.today_stats.total_amount += amount;
        self.today_stats.goal_achieved = self.today_stats.total_amount >= self.today_stats.goal_amount;
    }

    pub fn undo_last_record(&mut self) -> bool {
        if let Some(last_record) = self.today_stats.records.pop() {
            self.today_stats.total_amount = self.today_stats.total_amount.saturating_sub(last_record.amount);
            self.today_stats.goal_achieved = self.today_stats.total_amount >= self.today_stats.goal_amount;
            true
        } else {
            false
        }
    }

    pub fn get_progress_percentage(&self) -> f32 {
        if self.today_stats.goal_amount == 0 {
            return 0.0;
        }
        (self.today_stats.total_amount as f32 / self.today_stats.goal_amount as f32 * 100.0).min(100.0)
    }

    // 统计功能方法
    pub fn get_weekly_average(&self) -> u32 {
        if self.weekly_stats.is_empty() {
            return self.today_stats.total_amount;
        }
        let total: u32 = self.weekly_stats.iter().map(|s| s.total_amount).sum::<u32>() + self.today_stats.total_amount;
        let days = self.weekly_stats.len() + 1;
        total / days as u32
    }

    pub fn get_weekly_total(&self) -> u32 {
        let weekly_total: u32 = self.weekly_stats.iter().map(|s| s.total_amount).sum();
        weekly_total + self.today_stats.total_amount
    }

    pub fn get_streak_days(&self) -> u32 {
        let mut streak = 0;
        
        // 检查今天是否达标
        if self.today_stats.goal_achieved {
            streak += 1;
        } else {
            return 0; // 今天没达标，连击中断
        }
        
        // 从最近的日期开始往前检查
        for stats in self.weekly_stats.iter().rev() {
            if stats.goal_achieved {
                streak += 1;
            } else {
                break;
            }
        }
        
        streak
    }

    pub fn get_max_daily_amount(&self) -> u32 {
        let mut max_amount = self.today_stats.total_amount;
        
        for stats in &self.weekly_stats {
            if stats.total_amount > max_amount {
                max_amount = stats.total_amount;
            }
        }
        
        max_amount
    }

    // 获取7天的饮水数据数组，用于柱状图显示
    pub fn get_seven_days_data(&self) -> Vec<u32> {
        let mut seven_days = vec![0u32; 7];
        
        // 获取过去6天的数据（如果有的话）
        let available_days = std::cmp::min(self.weekly_stats.len(), 6);
        for i in 0..available_days {
            if let Some(stats) = self.weekly_stats.get(self.weekly_stats.len() - available_days + i) {
                seven_days[i] = stats.total_amount;
            }
        }
        
        // 最后一天是今天
        seven_days[6] = self.today_stats.total_amount;
        
        seven_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert_eq!(state.settings.daily_goal, 2000);
        assert_eq!(state.today_stats.total_amount, 0);
        assert_eq!(state.today_stats.records.len(), 0);
    }

    #[test]
    fn test_add_water_record() {
        let mut state = AppState::new();
        state.add_water_record(250);
        
        assert_eq!(state.today_stats.total_amount, 250);
        assert_eq!(state.today_stats.records.len(), 1);
        assert_eq!(state.today_stats.records[0].amount, 250);
    }

    #[test]
    fn test_undo_last_record() {
        let mut state = AppState::new();
        state.add_water_record(250);
        state.add_water_record(300);
        
        assert_eq!(state.today_stats.total_amount, 550);
        
        let undone = state.undo_last_record();
        assert!(undone);
        assert_eq!(state.today_stats.total_amount, 250);
        assert_eq!(state.today_stats.records.len(), 1);
    }

    #[test]
    fn test_progress_percentage() {
        let mut state = AppState::new();
        state.add_water_record(1000); // 50% of 2000ml goal
        
        let progress = state.get_progress_percentage();
        assert_eq!(progress, 50.0);
    }

    #[test]
    fn test_goal_achievement() {
        let mut state = AppState::new();
        state.add_water_record(2000); // Exactly meet the goal
        
        assert!(state.today_stats.goal_achieved);
        
        state.add_water_record(500); // Exceed the goal
        assert!(state.today_stats.goal_achieved);
    }
}