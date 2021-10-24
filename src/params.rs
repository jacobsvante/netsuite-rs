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

impl<'a> From<Params<'a>> for HashMap<&'a str, &'a str> {
    fn from(params: Params<'a>) -> Self {
        let mut map = Self::with_capacity(params.0.len());
        for (k, v) in params.0 {
            map.insert(k, v);
        }
        map
    }
}

impl<'a> Default for Params<'a> {
    fn default() -> Self {
        Self::new()
    }
}
