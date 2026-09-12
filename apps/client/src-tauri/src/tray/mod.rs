use anyhow::Context;
use i18n::I18nState;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuAction {
    Quit,
}

impl AsRef<str> for MenuAction {
    fn as_ref(&self) -> &str {
        match self {
            MenuAction::Quit => "quit",
        }
    }
}

impl TryFrom<&str> for MenuAction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "quit" => Ok(MenuAction::Quit),
            _ => Err(()),
        }
    }
}

fn build_menu(app: &AppHandle) -> anyhow::Result<Menu<Wry>> {
    let i18n = app.state::<I18nState>();

    let quit = MenuItem::with_id(
        app,
        MenuAction::Quit.as_ref(),
        &i18n.current().tray.menu.quit,
        true,
        None::<&str>,
    )?;

    Ok(Menu::with_items(app, &[&quit])?)
}

pub fn setup(app: &AppHandle) -> anyhow::Result<()> {
    let menu = build_menu(app)?;

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
                    MenuAction::Quit => app.exit(0),
                }
            }
        })
        .build(app)?;

    Ok(())
}
