use std::collections::BTreeMap;

#[derive(Debug)]
pub struct StringDictionary {
    dict: BTreeMap<String, String>,
}

impl StringDictionary {
    pub fn new() -> Self {
        StringDictionary {
            dict: BTreeMap::new(),
        }
    }
    // FIXME: KeePass actually returns C# int which is i32.
    pub fn count(&self) -> usize {
        self.dict.len()
    }
}
