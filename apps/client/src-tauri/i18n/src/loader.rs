use crate::schema::Schema;
use anyhow::Context;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    EN,
    ES,
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Locale {
    fn bundled_json(&self) -> &'static str {
        match self {
            Locale::EN => include_str!("../locales/en.json"),
            Locale::ES => include_str!("../locales/es.json"),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Locale::EN => "en",
            Locale::ES => "es",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Locale::EN => "English",
            Locale::ES => "Español",
        }
    }

    pub fn all() -> &'static [Locale] {
        &[Locale::EN, Locale::ES]
    }
}

#[derive(Debug)]
pub struct LocaleData {
    pub locale: Locale,
    pub data: Schema,
}

#[derive(Default)]
pub struct LocaleLoader {
    locales: HashMap<Locale, LocaleData>,
}

impl LocaleLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_locale(&mut self, locale: Locale) -> anyhow::Result<()> {
        if self.locales.contains_key(&locale) {
            return Ok(());
        }

        let content = locale.bundled_json();
        let data = serde_json::from_str::<Schema>(content)
            .context(format!("failed to parse {:?} locale", locale))?;

        self.locales.insert(locale, LocaleData { locale, data });

        Ok(())
    }

    // Dont like this api, result and option doesn't make sense here D:
    pub fn get_locale(&mut self, locale: Locale) -> anyhow::Result<Option<&LocaleData>> {
        if self.locales.contains_key(&locale) {
            return Ok(self.locales.get(&locale));
        }

        self.load_locale(locale)
            .with_context(|| format!("failed to load locale: {}", locale))?;
        Ok(self.locales.get(&locale))
    }

    pub fn unload_locale(&mut self, locale: Locale) {
        self.locales.remove(&locale);
    }

    pub fn unload_all_except(&mut self, keep: Locale) {
        self.locales.retain(|k, _| *k == keep);
    }
}
