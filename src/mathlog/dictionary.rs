use std::collections::HashMap;

type TypstName = String;
type MathlogName = String;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Dictionary {
    pub idents: HashMap<TypstName, MathlogName>,
    pub modules: HashMap<TypstName, Dictionary>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            idents: HashMap::new(),
            modules: HashMap::new(),
        }
    }

    pub fn insert_ident(&mut self, ident: &str, math_ident: &str) {
        self.idents
            .insert(ident.to_string(), math_ident.to_string());
    }

    pub fn insert_mod(&mut self, mod_name: &str, dic: Dictionary) {
        self.modules.insert(mod_name.to_string(), dic);
    }

    pub fn read(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let dic_file = std::fs::read_to_string(path)?;
        serde_json::from_str(&dic_file).map_err(|e| e.into())
    }

    pub fn get<I>(&self, path: I) -> Option<&MathlogName>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut path = path.into_iter();
        let mut dic = self;
        let mut ident = path.next()?;
        for next in path {
            dic = dic.modules.get(ident.as_ref())?;
            ident = next;
        }
        dic.idents.get(ident.as_ref())
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}
