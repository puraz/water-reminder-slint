use tray_icon::{TrayIcon, TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}, Icon};
use std::sync::mpsc;

pub enum TrayMessage {
    Show,
    Hide,
    Quit,
}

pub struct SystemTray {
    _tray_icon: TrayIcon,
    menu_receiver: mpsc::Receiver<MenuEvent>,
}

impl SystemTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Linux平台需要初始化GTK
        #[cfg(target_os = "linux")]
        {
            if !gtk::is_initialized() {
                gtk::init()?;
            }
        }
        
        // 创建托盘图标
        let icon = Self::create_icon()?;
        
        // 创建菜单项
        let show_item = MenuItem::new("显示水分提醒", true, None);
        let hide_item = MenuItem::new("隐藏到托盘", true, None);
        let separator = MenuItem::new("", false, None);
        let quit_item = MenuItem::new("退出", true, None);
        
        // 创建菜单
        let menu = Menu::new();
        menu.append(&show_item)?;
        menu.append(&hide_item)?;
        menu.append(&separator)?;
        menu.append(&quit_item)?;
        
        // 设置菜单事件接收器
        let (menu_sender, menu_receiver) = mpsc::channel();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            let _ = menu_sender.send(event);
        }));
        
        // 创建托盘图标
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("水分提醒")
            .with_icon(icon)
            .build()?;
        
        Ok(SystemTray {
            _tray_icon: tray_icon,
            menu_receiver,
        })
    }
    
    pub fn handle_events(&self) -> Option<TrayMessage> {
        if let Ok(event) = self.menu_receiver.try_recv() {
            match event.id.0.as_str() {
                "显示水分提醒" => Some(TrayMessage::Show),
                "隐藏到托盘" => Some(TrayMessage::Hide),
                "退出" => Some(TrayMessage::Quit),
                _ => None,
            }
        } else {
            None
        }
    }
    
    fn create_icon() -> Result<Icon, Box<dyn std::error::Error>> {
        // 创建一个简单的水滴图标
        let icon_data = Self::create_water_drop_icon();
        let icon = Icon::from_rgba(icon_data, 32, 32)?;
        Ok(icon)
    }
    
    fn create_water_drop_icon() -> Vec<u8> {
        // 创建32x32的RGBA图标数据
        let mut data = vec![0u8; 32 * 32 * 4];
        
        // 绘制简单的水滴形状
        for y in 0..32 {
            for x in 0..32 {
                let center_x = 16.0;
                let center_y = 20.0;
                let dx = x as f64 - center_x;
                let dy = y as f64 - center_y;
                
                // 水滴形状算法
                let distance = (dx * dx + dy * dy).sqrt();
                let teardrop = if y < 20 {
                    // 圆形部分
                    distance <= 8.0
                } else {
                    // 尖端部分
                    let tip_factor = (32.0 - y as f64) / 12.0;
                    distance <= 8.0 * tip_factor && tip_factor > 0.0
                };
                
                if teardrop {
                    let idx = (y * 32 + x) * 4;
                    data[idx] = 64;      // R - 深蓝色
                    data[idx + 1] = 164; // G
                    data[idx + 2] = 255; // B - 蓝色水滴
                    data[idx + 3] = 255; // A - 完全不透明
                }
            }
        }
        
        data
    }
}