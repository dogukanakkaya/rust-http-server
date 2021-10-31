use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(query_string: &'buf str) -> Self {
        let mut data = HashMap::new();

        for pair in query_string.split('&') {
            if let Some(i) = pair.find('=') {
                let key = &pair[..i];
                let value = &pair[i + 1..];

                data.entry(key)
                    .and_modify(|existing: &mut Value| match existing {
                        Value::Single(prev_value) => {
                            *existing = Value::Multiple(vec![prev_value, value]);
                        }
                        Value::Multiple(prev_values) => prev_values.push(value)
                    })
                    .or_insert(Value::Single(value));
            }
        }

        Self {
            data
        }
    }
}