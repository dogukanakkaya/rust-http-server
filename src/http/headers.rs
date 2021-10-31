use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}

impl<'buf> Headers<'buf> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&&'buf str> {
        self.data.get(key)
    }
}

impl<'buf> From<&&'buf str> for Headers<'buf> {
    fn from(headers: &&'buf str) -> Self {
        let mut data = HashMap::new();

        for header in headers.split('\n') {
            if let Some(i) = header.find(':') {
                let key = &header[..i];
                let value = &header[i + 1..];

                data.insert(key, value.trim());
            }
        }

        Self {
            data
        }
    }
}
