use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use trayicon::{Icon, MenuBuilder, TrayIconBuilder};
use winreg::RegKey;
use winreg::enums::*;


#[derive(Clone, Eq, PartialEq, Debug)]
enum Events {
    ClickTrayIcon,
    Exit,
}

#[derive(Debug)]
enum AppIcon {
    Icon1,
    Icon2,
}

impl AppIcon {
    fn next(&mut self) {
        use AppIcon::*;
        *self = match *self {
            Icon1 => Icon2,
            Icon2 => Icon1,
        }
    }
}

fn daylight() {
    // HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize /v AppsUseLightTheme /d 1 /t REG_DWORD /f
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (personalize, _disp) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize").unwrap();
    personalize.set_value("AppsUseLightTheme", &1u32).unwrap();
}

fn nightshift() {
    // HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize /v AppsUseLightTheme /d 0 /t REG_DWORD /f
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (personalize, _disp) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize").unwrap();
    personalize.set_value("AppsUseLightTheme", &0u32).unwrap();
}


fn main() {
    let event_loop = EventLoop::<Events>::with_user_event();
    let your_app_window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();
    let proxy = event_loop.create_proxy();

    // default Icon
    let mut app_icon = AppIcon::Icon2;

    let icon = include_bytes!("../res/sun.ico");
    let icon2 = include_bytes!("../res/moon.ico");

    let second_icon = Icon::from_buffer(icon2, None, None).unwrap();
    let first_icon = Icon::from_buffer(icon, None, None).unwrap();

    // Needlessly complicated tray icon with all the whistles and bells
    let mut tray_icon = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(icon)
        .tooltip("Nightshift")
        .on_click(Events::ClickTrayIcon)
        .menu(
            MenuBuilder::new()
                .item("E&xit", Events::Exit),
        )
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Move the tray_icon to the main loop (even if you don't use it)
        //
        // Tray icon uses normal message pump from winit, for orderly closure
        // and removal of the tray icon when you exit it must be moved inside.
        let _ = tray_icon;

        match event {
            // Main window events
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == your_app_window.id() => *control_flow = ControlFlow::Exit,

            // User events
            Event::UserEvent(e) => match e {
                Events::Exit => *control_flow = ControlFlow::Exit,
                Events::ClickTrayIcon => {
                    app_icon.next();
                    match app_icon {
                        AppIcon::Icon1 => {
                            nightshift();
                            tray_icon.set_icon(&second_icon).unwrap();
                        },
                        AppIcon::Icon2 => {
                            daylight();
                            tray_icon.set_icon(&first_icon).unwrap();
                        },
                    }
                }
            },
            _ => (),
        }
    });
}
