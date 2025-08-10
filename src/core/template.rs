use super::constants::PATTERN_START;
use eyre::Result;

#[derive(Clone, Debug)]
pub struct Template {
    pub name: String,
    pub context: serde_yml::Value,
    pub lines: Vec<String>,
}

impl Template {
    pub fn from_lines(lines: Vec<String>) -> Result<Self> {
        let first = lines.first().unwrap();
        let captures = PATTERN_START.captures(first).unwrap();
        let name = captures.name("name").unwrap().as_str();
        let context = match captures.name("context") {
            Some(context) => serde_yml::from_str(context.as_str())?,
            None => serde_yml::Value::Null,
        };
        Ok(Self {
            name: name.to_string(),
            context,
            lines,
        })
    }
}
