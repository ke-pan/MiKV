use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    storage: RefCell<HashMap<String, String>>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            storage: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, k: String) -> Option<String> {
        if let Some(v) = self.storage.borrow().get(&k) {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn set(&self, k: String, v: String) -> Option<String> {
        self.storage.borrow_mut().insert(k, v)
    }

    pub fn delete(&self, k: &String) -> Option<String> {
        self.storage.borrow_mut().remove(k)
    }
}
