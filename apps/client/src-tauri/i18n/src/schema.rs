use crate::loader::{Locale, LocaleLoader};
use anyhow::Context;
use i18n_macros::i18n_schema;

i18n_schema! {
    pub struct Schema {
        pub tray: struct Tray {
            pub menu: struct TrayMenu {
                pub show: String,
                pub hide: String,
            }
        }
    }
}

pub struct I18nState {
    pub current_locale: Locale,
    pub loader: LocaleLoader,
}

impl Default for I18nState {
    fn default() -> Self {
        Self {
            current_locale: Locale::EN,
            loader: LocaleLoader::new(),
        }
    }
}

impl I18nState {
    pub fn new() -> anyhow::Result<Self> {
        let mut loader = LocaleLoader::new();
        loader.load_locale(Locale::EN)?;

        Ok(Self {
            current_locale: Locale::EN,
            loader,
        })
    }

    pub fn current(&mut self) -> anyhow::Result<&Schema> {
        let locale = self
            .loader
            .get_locale(self.current_locale)
            .with_context(|| format!("failed to load locale: {}", self.current_locale))?
            .ok_or_else(|| anyhow::anyhow!("locale not found: {}", self.current_locale))?;

        Ok(&locale.data)
    }

    pub fn set_locale(&mut self, locale: Locale) -> anyhow::Result<()> {
        self.loader.load_locale(locale)?;
        self.current_locale = locale;
        self.loader.unload_all_except(locale);

        Ok(())
    }

    pub fn get_schema(&mut self, locale: Locale) -> anyhow::Result<&Schema> {
        let locale_data = self.loader.get_locale(locale)?;
        Ok(&locale_data.data)
    }
}
