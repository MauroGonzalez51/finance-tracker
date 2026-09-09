use anyhow::Context;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuAction {
    Show,
    Quit,
}

impl AsRef<str> for MenuAction {
    fn as_ref(&self) -> &str {
        match self {
            MenuAction::Show => "show",
            MenuAction::Quit => "quit",
        }
    }
}

impl TryFrom<&str> for MenuAction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "show" => Ok(MenuAction::Show),
            "quit" => Ok(MenuAction::Quit),
            _ => Err(()),
        }
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

pub fn setup(app: &AppHandle) -> anyhow::Result<()> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, MenuAction::Show.as_ref(), "Show", true, None::<&str>)?,
            &MenuItem::with_id(app, MenuAction::Quit.as_ref(), "Quit", true, None::<&str>)?,
        ],
    )?;

    TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .context("failed to get default app icon")?
                .clone(),
        )
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            if let Ok(action) = MenuAction::try_from(event.id.as_ref()) {
                match action {
                    MenuAction::Show => show_main_window(app),
                    MenuAction::Quit => app.exit(0),
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}
