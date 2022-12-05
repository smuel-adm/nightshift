//#![windows_subsystem = "windows"]

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
    Sun,
    Moon,
}

impl AppIcon {
    fn next(&mut self) {
        use AppIcon::*;
        *self = match *self {
            Sun => Moon,
            Moon => Sun,
        }
    }
}

fn is_nightshift() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize") {
        Ok(personalize) => match personalize.get_value("AppsUseLightTheme") {
            Ok(value) => match value {
                0u32 => true,
                _ => false,
            },
            Err(_e) => false
        },
        Err(_e) => false,
    }
}


fn set_daylight() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (personalize, _disp) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize").unwrap();
    personalize.set_value("AppsUseLightTheme", &1u32).unwrap();
}

fn set_nightshift() {
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
    let mut app_icon = match is_nightshift() {
        true => AppIcon::Moon,
        false => AppIcon::Sun,
    };

    let icon_sun = include_bytes!("../res/sun.ico");
    let icon_moon = include_bytes!("../res/moon.ico");
    
    let sun_icon = Icon::from_buffer(icon_sun, None, None).unwrap();
    let moon_icon = Icon::from_buffer(icon_moon, None, None).unwrap();

    let icon = match is_nightshift() {
        true => moon_icon.clone(),
        false => sun_icon.clone(),
    };

    // Needlessly complicated tray icon with all the whistles and bells
    let mut tray_icon = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon(icon)
        .tooltip("Nightshift - Toggle Dark/ Light Mode")
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
                    match app_icon {
                        AppIcon::Sun => {
                            set_nightshift();
                            tray_icon.set_icon(&moon_icon).unwrap();
                        },
                        AppIcon::Moon => {
                            set_daylight();
                            tray_icon.set_icon(&sun_icon).unwrap();
                        },
                    }
                    app_icon.next();
                }
            },
            _ => (),
        }
    });
}
