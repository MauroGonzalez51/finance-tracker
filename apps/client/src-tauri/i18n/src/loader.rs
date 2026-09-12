use crate::schema::Schema;
use anyhow::Context;
use std::{collections::HashMap, sync::Arc};

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

#[derive(Default, Clone)]
pub struct LocaleLoader {
    cache: HashMap<Locale, Arc<Schema>>,
}

impl LocaleLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, locale: Locale) -> anyhow::Result<()> {
        if self.cache.contains_key(&locale) {
            return Ok(());
        }

        let content = locale.bundled_json();
        let schema = serde_json::from_str::<Schema>(content)
            .with_context(|| format!("failed to parse {} locale", locale))?;

        self.cache.insert(locale, Arc::new(schema));
        Ok(())
    }

    pub fn get(&self, locale: Locale) -> Option<Arc<Schema>> {
        self.cache.get(&locale).cloned()
    }

    pub fn unload(&mut self, locale: Locale) {
        self.cache.remove(&locale);
    }

    pub fn keep_only(&mut self, locale: Locale) {
        self.cache.retain(|k, _| *k == locale);
    }

    pub fn loaded(&self) -> Vec<Locale> {
        self.cache.keys().copied().collect()
    }
}
