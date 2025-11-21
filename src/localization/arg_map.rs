use std::{borrow::Cow, collections::HashMap};

use fluent_bundle::FluentValue;

#[derive(Debug, Default)]
pub struct ArgMapBuilder<'a> {
    values: Vec<(String, FluentValue<'a>)>,
}

impl<'a> ArgMapBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn arg<S: Into<String>>(mut self, key: S, value: FluentValue<'a>) -> Self {
        self.values.push((key.into(), value));
        self
    }

    pub fn arg_str<S: Into<String>>(mut self, key: S, value: &'a str) -> Self {
        self.values.push((key.into(), FluentValue::from(value)));
        self
    }

    pub fn arg_num<S: Into<String>>(mut self, key: S, value: i64) -> Self {
        self.values.push((key.into(), FluentValue::from(value)));
        self
    }

    pub fn build(self) -> HashMap<Cow<'static, str>, FluentValue<'a>> {
        let mut map = HashMap::new();
        for (k, v) in self.values {
            map.insert(Cow::Owned(k), v);
        }
        map
    }
}
