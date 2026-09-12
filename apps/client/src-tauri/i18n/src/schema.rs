use crate::loader::{Locale, LocaleLoader};
use anyhow::Context;
use i18n_macros::i18n_schema;
use std::sync::{Arc, Mutex};

i18n_schema! {
    pub struct Schema {
        pub tray: struct Tray {
            pub menu: struct TrayMenu {
                pub quit: String,
            }
        }
    }
}

pub struct I18nState {
    current: Mutex<(Locale, Arc<Schema>)>,
    loader: Mutex<LocaleLoader>,
}

impl Default for I18nState {
    fn default() -> Self {
        Self::new(Locale::EN).expect("failed to load default locale")
    }
}

impl I18nState {
    pub fn new(locale: Locale) -> anyhow::Result<Self> {
        let mut loader = LocaleLoader::new();
        loader.load(locale)?;

        let schema = loader
            .get(locale)
            .with_context(|| format!("failed to load {} locale", locale))?
            .clone();

        Ok(Self {
            current: Mutex::new((Locale::EN, schema)),
            loader: Mutex::new(loader),
        })
    }

    pub fn current(&self) -> Arc<Schema> {
        let (_, schema) = &*self.current.lock().unwrap();
        schema.clone()
    }

    pub fn locale(&self) -> Locale {
        let (locale, _) = *self.current.lock().unwrap();
        locale
    }

    pub fn schema(&mut self, locale: Locale) -> anyhow::Result<Arc<Schema>> {
        let mut loader = self.loader.lock().unwrap();
        loader.load(locale)?;
        loader
            .get(locale)
            .with_context(|| format!("failed to load {} locale", locale))
    }

    pub fn set_locale(&mut self, locale: Locale) -> anyhow::Result<()> {
        let mut loader = self.loader.lock().unwrap();

        loader.load(locale)?;

        let new_schema = loader.get(locale).context("failed to get locale")?;
        loader.keep_only(locale);

        let mut current = self.current.lock().unwrap();
        *current = (locale, new_schema);

        Ok(())
    }
}
