#[derive(Debug, Clone)]
pub struct Params(Vec<(String, String)>);

impl Params {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<K: ToString, V: ToString>(&mut self, k: K, v: V) {
        self.0.push((k.to_string(), v.to_string()))
    }

    pub fn get(&self) -> &Vec<(String, String)> {
        &self.0
    }
}

impl From<Params> for Vec<(String, String)> {
    fn from(params: Params) -> Self {
        params.0
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "cli")]
#[derive(Debug)]
pub(crate) struct ParamStr(String, String);

#[cfg(feature = "cli")]
impl ParamStr {
    pub(crate) fn new(k: String, v: String) -> Self {
        Self(k, v)
    }
}

#[cfg(feature = "cli")]
impl From<Vec<ParamStr>> for Params {
    fn from(param_strings: Vec<ParamStr>) -> Self {
        let mut p = Params::new();
        for ps in param_strings {
            p.push(ps.0, ps.1);
        }
        p
    }
}
