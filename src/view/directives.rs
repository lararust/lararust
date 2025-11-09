use std::collections::HashMap;

pub type DirectiveFn = fn(&str) -> String;

pub struct Directives {
    pub map: HashMap<String, DirectiveFn>,
}

impl Directives {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("uppercase".to_string(), |arg| {
            format!("{{{{ {} | upper }}}}", arg)
        });
        Self { map }
    }

    pub fn apply(&self, name: &str, arg: &str) -> Option<String> {
        self.map.get(name).map(|f| f(arg))
    }
}
