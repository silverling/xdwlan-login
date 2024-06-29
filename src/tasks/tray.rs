use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use tray_icon::TrayIcon;
use tray_icon::{
    menu::{
        CheckMenuItem, Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem, PredefinedMenuItem,
    },
    Icon, TrayIconBuilder,
};
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use super::{AppEvent, Task};
use crate::utils::{get_program_folder, is_autostart, toggle_autostart};

pub struct TrayTask;

impl TrayTask {
    pub fn new() -> Self {
        TrayTask {}
    }
}

impl Task for TrayTask {
    fn run(&self, sender: Sender<AppEvent>, _: Receiver<AppEvent>) -> anyhow::Result<()> {
        log::debug!("Tray task started.");

        let mut tray = Tray::new(sender);
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut tray)?;

        log::debug!("Tray task stopped.");

        Ok(())
    }
}

enum UserEvent {
    OpenFolder,
    Autostart,
    Quit,
}

struct TrayItems {
    autostart: CheckMenuItem,
}

pub struct Tray {
    menu_channel: &'static MenuEventReceiver,
    sender: Sender<AppEvent>,

    tray_icon: Option<TrayIcon>,
    roi_items: Option<TrayItems>,
    event_table: Option<HashMap<MenuId, UserEvent>>,
}

impl Tray {
    fn new(sender: Sender<AppEvent>) -> Self {
        Tray {
            menu_channel: MenuEvent::receiver(),
            sender,
            tray_icon: None,
            roi_items: None,
            event_table: None,
        }
    }

    fn init(&mut self) -> anyhow::Result<()> {
        let icon = Icon::from_resource_name("app-icon", None)?;
        let autostart = is_autostart();

        let menu = Box::new(Menu::new());
        let menu_item_openfolder = MenuItem::new("Open folder", true, None);
        let menu_item_autostart = CheckMenuItem::new("Autostart", true, autostart, None);
        let menu_item_quit = MenuItem::new("Quit", true, None);

        menu.append_items(&[
            &menu_item_openfolder,
            &menu_item_autostart,
            &PredefinedMenuItem::separator(),
            &menu_item_quit,
        ])?;

        let mut event_table = HashMap::new();
        event_table.insert(menu_item_openfolder.id().to_owned(), UserEvent::OpenFolder);
        event_table.insert(menu_item_autostart.id().to_owned(), UserEvent::Autostart);
        event_table.insert(menu_item_quit.id().to_owned(), UserEvent::Quit);

        let tray_icon = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip("Tray Icon App Tooltip")
            .with_icon(icon)
            .with_title("Tray Icon App")
            .build()?;

        self.roi_items = Some(TrayItems {
            autostart: menu_item_autostart,
        });
        self.event_table = Some(event_table);
        self.tray_icon = Some(tray_icon);

        Ok(())
    }
}

impl ApplicationHandler for Tray {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                if let Err(e) = self.init() {
                    log::error!("{}", e);
                    self.sender.send(AppEvent::Quit).unwrap();
                    event_loop.exit();
                }
            }
            _ => {
                if let Ok(menu_event) = self.menu_channel.try_recv() {
                    if let Some(user_event) =
                        self.event_table.as_ref().unwrap().get(menu_event.id())
                    {
                        match user_event {
                            UserEvent::OpenFolder => {
                                // Open the folder where the program is located.
                                if let Err(e) = std::process::Command::new("explorer")
                                    .arg(get_program_folder())
                                    .spawn()
                                {
                                    log::error!("Open folder error: {}", e)
                                }
                            }
                            UserEvent::Autostart => match toggle_autostart() {
                                Ok(state) => {
                                    self.roi_items
                                        .as_mut()
                                        .unwrap()
                                        .autostart
                                        .set_checked(state);
                                }
                                Err(e) => log::error!("{}", e),
                            },
                            UserEvent::Quit => {
                                if let Err(e) = self.sender.send(AppEvent::Quit) {
                                    log::error!("{}", e);
                                }
                                let _ = self.sender.send(AppEvent::Quit);
                                event_loop.exit();
                            }
                        }
                    }
                }
            }
        }
    }
}
