extern crate alloc;
use alloc::collections::BTreeMap;
use std::env::var;
use std::fs;
use std::path::PathBuf;

pub struct Tests(BTreeMap<String, String>);

impl Tests {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn load() -> Self {
        Self(
            fs::read(Self::path())
                .map(|content| postcard::from_bytes(&content).unwrap())
                .unwrap_or_default(),
        )
    }

    fn path() -> PathBuf {
        PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("output")
    }

    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }

    pub fn set(&mut self, key: String, content: &str) {
        self.0
            .entry(key)
            .and_modify(|old| content.clone_into(old))
            .or_insert_with(|| content.to_owned());
    }

    pub fn store(&self) {
        fs::write(Self::path(), postcard::to_allocvec(&self.0).unwrap()).unwrap();
    }
}
