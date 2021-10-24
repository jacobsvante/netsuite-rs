use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Params<'a>(Vec<(&'a str, &'a str)>);

impl<'a> Params<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, k: &'a str, v: &'a str) {
        self.0.push((k, v))
    }

    pub fn get(&self) -> &Vec<(&'a str, &'a str)> {
        &self.0
    }
}

impl<'a> Into<HashMap<&'a str, &'a str>> for Params<'a> {
    fn into(self) -> HashMap<&'a str, &'a str> {
        let mut map = HashMap::with_capacity(self.0.len());
        for (k, v) in self.0 {
            map.insert(k, v);
        }
        map
    }
}
