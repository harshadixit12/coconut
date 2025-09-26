use std::collections::HashMap;

pub struct Scope {
    pub store: HashMap<String, u64>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            store: HashMap::new(),
        }
    }

    pub fn dec_var(&mut self, id: String, value: u64) -> Option<u64> {
        self.store.insert(id, value)
    }

    pub fn set_var(&mut self, id: String, value: u64) -> Option<u64> {
        self.store.insert(id, value)
    }

    pub fn get_var(&self, id: String) -> Option<&u64> {
        self.store.get(&id.clone())
    }
}