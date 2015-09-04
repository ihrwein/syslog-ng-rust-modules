use std::collections::BTreeMap;
use super::Message;

pub struct Builder {
    uuid: String,
    name: Option<String>,
    values: BTreeMap<String, String>
}

impl Builder {
    pub fn new(uuid: &str) -> Builder {
        Builder {
            uuid: uuid.to_string(),
            name: None,
            values: BTreeMap::new()
        }
    }

    pub fn name(&mut self, name: String) -> &mut Builder {
        self.name = Some(name);
        self
    }

    pub fn pair(&mut self, key: String, value: String) -> &mut Builder {
        self.values.insert(key, value);
        self
    }

    pub fn build(&self) -> Message {
        Message {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            values: self.values.clone()
        }
    }
}
