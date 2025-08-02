use std::rc::Rc;
use std::cell::RefCell;
use slint::{VecModel, ComponentHandle};

mod models;
mod utils;

use utils::data::DataManager;
use utils::notification::NotificationManager;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let data_manager = Rc::new(DataManager::new().expect("无法初始化数据管理器"));
    let app_state = Rc::new(RefCell::new(data_manager.load_app_state()));
    let notification_manager = NotificationManager::new(app_state.borrow().settings.reminder_enabled);
    
    // 设置初始提醒间隔
    notification_manager.update_settings(
        app_state.borrow().settings.reminder_enabled,
        app_state.borrow().settings.reminder_interval
    );
    
    // 启动提醒循环
    {
        let notification_manager_clone = notification_manager.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                notification_manager_clone.start_reminder_loop().await;
            });
        });
    }
    
    let ui = AppWindow::new()?;
    
    // 初始化UI状态
    {
        let mut state = app_state.borrow_mut();
        
        // 如果今天已经有一些饮水记录，确保达标状态正确
        if state.today_stats.total_amount >= state.settings.daily_goal {
            state.today_stats.goal_achieved = true;
        }
        
        ui.global::<AppState>().set_daily_goal(state.settings.daily_goal as i32);
        ui.global::<AppState>().set_total_today(state.today_stats.total_amount as i32);
        ui.global::<AppState>().set_progress_percentage(state.get_progress_percentage());
        ui.global::<AppState>().set_reminder_enabled(state.settings.reminder_enabled);
        ui.global::<AppState>().set_reminder_interval(state.settings.reminder_interval as i32);
        ui.global::<AppState>().set_current_page(0); // 确保从主页开始
        
        // 设置统计数据
        ui.global::<AppState>().set_weekly_average(state.get_weekly_average() as i32);
        ui.global::<AppState>().set_streak_days(state.get_streak_days() as i32);
        ui.global::<AppState>().set_max_daily(state.get_max_daily_amount() as i32);
        ui.global::<AppState>().set_total_week(state.get_weekly_total() as i32);
        
        // 设置7天数据
        let seven_days_data: Vec<i32> = state.get_seven_days_data().into_iter().map(|x| x as i32).collect();
        let seven_days_model = std::rc::Rc::new(slint::VecModel::from(seven_days_data));
        ui.global::<AppState>().set_seven_days_data(seven_days_model.into());
        
        // 设置今日记录（按时间倒序）
        let mut records: Vec<WaterRecord> = state.today_stats.records.iter().map(|r| {
            WaterRecord {
                id: r.id as i32,
                amount: r.amount as i32,
                time: r.timestamp.format("%H:%M").to_string().into(),
            }
        }).collect();
        records.reverse(); // 倒序排列，最新的记录在前面
        
        let records_model = Rc::new(VecModel::from(records));
        ui.global::<AppState>().set_today_records(records_model.into());
    }
    
    // 设置回调函数
    {
        let app_state_clone = app_state.clone();
        let ui_weak = ui.as_weak();
        let data_manager_clone = data_manager.clone();
        let notification_manager_clone = notification_manager.clone();
        
        ui.global::<AppState>().on_add_water(move |amount| {
            let mut state = app_state_clone.borrow_mut();
            state.add_water_record(amount as u32);
            
            // 更新UI
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_total_today(state.today_stats.total_amount as i32);
                ui.global::<AppState>().set_progress_percentage(state.get_progress_percentage());
                
                // 更新统计数据
                ui.global::<AppState>().set_weekly_average(state.get_weekly_average() as i32);
                ui.global::<AppState>().set_streak_days(state.get_streak_days() as i32);
                ui.global::<AppState>().set_max_daily(state.get_max_daily_amount() as i32);
                ui.global::<AppState>().set_total_week(state.get_weekly_total() as i32);
                
                // 更新7天数据
                let seven_days_data: Vec<i32> = state.get_seven_days_data().into_iter().map(|x| x as i32).collect();
                let seven_days_model = std::rc::Rc::new(slint::VecModel::from(seven_days_data));
                ui.global::<AppState>().set_seven_days_data(seven_days_model.into());
                
                // 更新记录列表（按时间倒序）
                let mut records: Vec<WaterRecord> = state.today_stats.records.iter().map(|r| {
                    WaterRecord {
                        id: r.id as i32,
                        amount: r.amount as i32,
                        time: r.timestamp.format("%H:%M").to_string().into(),
                    }
                }).collect();
                records.reverse(); // 倒序排列，最新的记录在前面
                
                let records_model = Rc::new(VecModel::from(records));
                ui.global::<AppState>().set_today_records(records_model.into());
                
                // 检查是否达成目标
                if state.today_stats.goal_achieved && (state.today_stats.total_amount - amount as u32) < state.today_stats.goal_amount {
                    let _ = notification_manager_clone.show_goal_achieved();
                }
            }
            
            // 保存数据
            let _ = data_manager_clone.save_app_state(&state);
        });
    }
    
    {
        let app_state_clone = app_state.clone();
        let ui_weak = ui.as_weak();
        let data_manager_clone = data_manager.clone();
        
        ui.global::<AppState>().on_undo_last_record(move || {
            let mut state = app_state_clone.borrow_mut();
            if state.undo_last_record() {
                // 更新UI
                if let Some(ui) = ui_weak.upgrade() {
                    ui.global::<AppState>().set_total_today(state.today_stats.total_amount as i32);
                    ui.global::<AppState>().set_progress_percentage(state.get_progress_percentage());
                    
                    // 更新统计数据
                    ui.global::<AppState>().set_weekly_average(state.get_weekly_average() as i32);
                    ui.global::<AppState>().set_streak_days(state.get_streak_days() as i32);
                    ui.global::<AppState>().set_max_daily(state.get_max_daily_amount() as i32);
                    ui.global::<AppState>().set_total_week(state.get_weekly_total() as i32);
                    
                    // 更新7天数据
                    let seven_days_data: Vec<i32> = state.get_seven_days_data().into_iter().map(|x| x as i32).collect();
                    let seven_days_model = std::rc::Rc::new(slint::VecModel::from(seven_days_data));
                    ui.global::<AppState>().set_seven_days_data(seven_days_model.into());
                    
                    // 更新记录列表（按时间倒序）
                    let mut records: Vec<WaterRecord> = state.today_stats.records.iter().map(|r| {
                        WaterRecord {
                            id: r.id as i32,
                            amount: r.amount as i32,
                            time: r.timestamp.format("%H:%M").to_string().into(),
                        }
                    }).collect();
                    records.reverse(); // 倒序排列，最新的记录在前面
                    
                    let records_model = Rc::new(VecModel::from(records));
                    ui.global::<AppState>().set_today_records(records_model.into());
                }
                
                // 保存数据
                let _ = data_manager_clone.save_app_state(&state);
            }
        });
    }
    
    {
        let app_state_clone = app_state.clone();
        let ui_weak = ui.as_weak();
        let data_manager_clone = data_manager.clone();
        
        ui.global::<AppState>().on_set_daily_goal(move |goal| {
            let mut state = app_state_clone.borrow_mut();
            state.settings.daily_goal = goal as u32;
            state.today_stats.goal_amount = goal as u32;
            
            // 更新UI
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_daily_goal(goal);
                ui.global::<AppState>().set_progress_percentage(state.get_progress_percentage());
            }
            
            // 保存数据
            let _ = data_manager_clone.save_app_state(&state);
        });
    }
    
    {
        let app_state_clone = app_state.clone();
        let ui_weak = ui.as_weak();
        let data_manager_clone = data_manager.clone();
        let notification_manager_clone = notification_manager.clone();
        
        ui.global::<AppState>().on_toggle_reminder(move |enabled| {
            let mut state = app_state_clone.borrow_mut();
            state.settings.reminder_enabled = enabled;
            
            // 更新通知管理器设置
            notification_manager_clone.update_settings(enabled, state.settings.reminder_interval);
            
            // 更新UI
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_reminder_enabled(enabled);
            }
            
            // 保存数据
            let _ = data_manager_clone.save_app_state(&state);
        });
    }
    
    {
        let app_state_clone = app_state.clone();
        let ui_weak = ui.as_weak();
        let data_manager_clone = data_manager.clone();
        let notification_manager_clone = notification_manager.clone();
        
        ui.global::<AppState>().on_set_reminder_interval(move |interval| {
            let mut state = app_state_clone.borrow_mut();
            state.settings.reminder_interval = interval as u32;
            
            // 更新通知管理器设置
            notification_manager_clone.update_settings(state.settings.reminder_enabled, interval as u32);
            
            // 更新UI
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_reminder_interval(interval);
            }
            
            // 保存数据
            let _ = data_manager_clone.save_app_state(&state);
        });
    }
    
    {
        let ui_weak = ui.as_weak();
        ui.global::<AppState>().on_show_custom_input_dialog(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_show_custom_input(true);
                ui.global::<AppState>().set_custom_amount("".into());
            }
        });
    }
    
    {
        let ui_weak = ui.as_weak();
        ui.global::<AppState>().on_hide_custom_input_dialog(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_show_custom_input(false);
                ui.global::<AppState>().set_custom_amount("".into());
            }
        });
    }
    
    {
        let app_state_clone = app_state.clone();
        let ui_weak = ui.as_weak();
        let data_manager_clone = data_manager.clone();
        let notification_manager_clone = notification_manager.clone();
        
        ui.global::<AppState>().on_add_custom_water(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let amount_str = ui.global::<AppState>().get_custom_amount();
                if let Ok(amount) = amount_str.to_string().parse::<u32>() {
                    if amount > 0 && amount <= 2000 { // 限制输入范围
                        let mut state = app_state_clone.borrow_mut();
                        state.add_water_record(amount);
                        
                        // 更新UI
                        ui.global::<AppState>().set_total_today(state.today_stats.total_amount as i32);
                        ui.global::<AppState>().set_progress_percentage(state.get_progress_percentage());
                        
                        // 更新统计数据
                        ui.global::<AppState>().set_weekly_average(state.get_weekly_average() as i32);
                        ui.global::<AppState>().set_streak_days(state.get_streak_days() as i32);
                        ui.global::<AppState>().set_max_daily(state.get_max_daily_amount() as i32);
                        ui.global::<AppState>().set_total_week(state.get_weekly_total() as i32);
                        
                        // 更新7天数据
                        let seven_days_data: Vec<i32> = state.get_seven_days_data().into_iter().map(|x| x as i32).collect();
                        let seven_days_model = std::rc::Rc::new(slint::VecModel::from(seven_days_data));
                        ui.global::<AppState>().set_seven_days_data(seven_days_model.into());
                        
                        // 更新记录列表（按时间倒序）
                        let mut records: Vec<WaterRecord> = state.today_stats.records.iter().map(|r| {
                            WaterRecord {
                                id: r.id as i32,
                                amount: r.amount as i32,
                                time: r.timestamp.format("%H:%M").to_string().into(),
                            }
                        }).collect();
                        records.reverse(); // 倒序排列，最新的记录在前面
                        
                        let records_model = Rc::new(VecModel::from(records));
                        ui.global::<AppState>().set_today_records(records_model.into());
                        
                        // 检查是否达成目标
                        if state.today_stats.goal_achieved && (state.today_stats.total_amount - amount) < state.today_stats.goal_amount {
                            let _ = notification_manager_clone.show_goal_achieved();
                        }
                        
                        // 保存数据
                        let _ = data_manager_clone.save_app_state(&state);
                        
                        // 关闭对话框
                        ui.global::<AppState>().set_show_custom_input(false);
                        ui.global::<AppState>().set_custom_amount("".into());
                    }
                }
            }
        });
    }
    
    {
        let ui_weak = ui.as_weak();
        ui.global::<AppState>().on_switch_page(move |page| {
            // 更新当前页面
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<AppState>().set_current_page(page);
            }
        });
    }
    
    ui.run()
}
