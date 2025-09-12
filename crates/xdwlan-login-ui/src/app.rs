use crate::notification::Notification;
use crate::server::LoginServer;
use crate::utils::{get_program_folder, is_autostart, toggle_autostart};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tray_icon::TrayIcon;
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem},
};
use winit::event_loop::EventLoopProxy;
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};
extern crate ctrlc;

pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    Quit,
}

enum Action {
    OpenFolder,
    Autostart,
    Quit,
}

/// The struct holds the tray menu items that need to be accessed later.
struct TrayItems {
    autostart: CheckMenuItem,
}

pub struct App {
    tray_icon: Option<TrayIcon>,
    tray_items: Option<TrayItems>,
    event_table: Option<HashMap<MenuId, Action>>,
    login_server: Option<Arc<Mutex<LoginServer>>>,
    event_proxy: EventLoopProxy<UserEvent>,
    notification: Notification,
}

impl App {
    pub fn new(event_proxy: &EventLoop<UserEvent>) -> Self {
        let proxy = event_proxy.create_proxy();

        App {
            tray_icon: None,
            tray_items: None,
            event_table: None,
            login_server: None,
            event_proxy: proxy,
            notification: Notification::new(),
        }
    }

    /// Register event handlers for tray icon events and Ctrl-C signal. This function should only be called once.
    pub fn register_events(event_loop: &EventLoop<UserEvent>) -> anyhow::Result<()> {
        // Register tray icon events
        let proxy = event_loop.create_proxy();
        tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
            _ = proxy.send_event(UserEvent::TrayIconEvent(event));
        }));

        // Register tray icon menu events
        let proxy = event_loop.create_proxy();
        tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
            _ = proxy.send_event(UserEvent::MenuEvent(event));
        }));

        // Register Ctrl-C handler
        let proxy = event_loop.create_proxy();
        ctrlc::set_handler(move || {
            let _ = proxy.send_event(UserEvent::Quit);
        })
        .expect("Error setting Ctrl-C handler");

        Ok(())
    }

    /// When event loop is ready, `init` should be called to initialize the tray icon.
    fn init(&mut self) -> anyhow::Result<()> {
        let icon = get_embed_icon()?;

        let menu = Box::new(Menu::new());
        let menu_item_openfolder = MenuItem::new("打开程序目录", true, None);
        let menu_item_autostart = CheckMenuItem::new("开机自启", true, is_autostart(), None);
        let menu_item_quit = MenuItem::new("退出", true, None);

        menu.append_items(&[
            &menu_item_openfolder,
            &menu_item_autostart,
            &PredefinedMenuItem::separator(),
            &menu_item_quit,
        ])?;

        let mut event_table = HashMap::new();
        event_table.insert(menu_item_openfolder.id().to_owned(), Action::OpenFolder);
        event_table.insert(menu_item_autostart.id().to_owned(), Action::Autostart);
        event_table.insert(menu_item_quit.id().to_owned(), Action::Quit);

        let tray_icon = TrayIconBuilder::new()
            .with_menu(menu)
            .with_icon(icon)
            .with_tooltip("西电校园网登录助手")
            .with_title("西电校园网登录助手")
            .build()?;

        self.tray_items = Some(TrayItems {
            autostart: menu_item_autostart,
        });
        self.tray_icon = Some(tray_icon);
        self.event_table = Some(event_table);

        Ok(())
    }

    /// Quit the application and the login server.
    fn exit(&mut self) {
        if let Some(login_server) = &self.login_server {
            let _ = login_server.lock().unwrap().exit();
        }
        self.notification.show("应用已退出.");
        std::fs::remove_file(&self.notification.icon).ok();
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                // Initialize the tray icon.
                if self.init().is_err() {
                    self.event_proxy.send_event(UserEvent::Quit).ok();
                }

                // Lanunch the login server.
                self.login_server = Some(Arc::new(Mutex::new(LoginServer::new())));

                // Show a notification indicating the app is ready.
                self.notification.show("应用已准备就绪.");
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::MenuEvent(tray_icon::menu::MenuEvent { id }) => {
                if let Some(action) = self.event_table.as_ref().unwrap().get(&id) {
                    match action {
                        // Open the folder where the program is located.
                        Action::OpenFolder => {
                            _ = std::process::Command::new("explorer")
                                .arg(get_program_folder())
                                .spawn();
                        }
                        // Toggle autostart setting.
                        Action::Autostart => match toggle_autostart() {
                            Ok(state) => {
                                self.tray_items
                                    .as_mut()
                                    .unwrap()
                                    .autostart
                                    .set_checked(state);
                            }
                            _ => {}
                        },
                        // Dispatch quit event.
                        Action::Quit => {
                            self.event_proxy.send_event(UserEvent::Quit).ok();
                        }
                    }
                }
            }
            UserEvent::Quit => {
                self.exit();
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn get_embed_icon() -> anyhow::Result<Icon> {
    let icon = tray_icon::Icon::from_resource_name("app-icon", None)?;
    Ok(icon)
}
