use super::template::Template;
use crate::core::PATTERN_END;
use crate::core::PATTERN_START;
use color_eyre::eyre::Result;

pub struct Environment<'a> {
    jinja: minijinja::Environment<'a>,
}

#[derive(Clone, Debug)]
pub enum Segment {
    Text(String),
    Template(Template),
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        let jinja = minijinja::Environment::new();
        Environment { jinja }
    }

    pub fn parse_text(&self, text: String) -> Result<Vec<Segment>> {
        let mut in_template: bool = false;
        let mut segments: Vec<Segment> = Vec::new();
        let mut template_lines: Vec<String> = Vec::new();
        for line in text.lines() {
            if in_template {
                template_lines.push(line.to_string());
                if PATTERN_END.is_match(line) {
                    in_template = false;
                    let template = Template::from_lines(template_lines.clone())?;
                    segments.push(Segment::Template(template));
                    template_lines.clear();
                    continue;
                }
            } else if PATTERN_START.is_match(line) {
                in_template = true;
                template_lines.push(line.to_string());
                continue;
            } else {
                segments.push(Segment::Text(line.to_string()));
            }
        }
        Ok(segments)
    }

    pub fn render(&self, segments: &[Segment]) -> Result<String> {
        let mut lines: Vec<String> = Vec::new();
        for segment in segments {
            match segment {
                Segment::Text(text) => lines.push(text.to_string()),
                Segment::Template(template) => match self.jinja.get_template(&template.name) {
                    Ok(tmpl) => {
                        let rendered = tmpl.render(&template.context)?;
                        lines.push(rendered);
                    }
                    Err(err) => {
                        lines.extend(template.lines.clone());
                        tracing::error!(error = %err, template = %template.name);
                    }
                },
            }
        }
        let mut text = lines.join("\n");
        if !text.ends_with("\n") {
            text.push('\n');
        }
        return Ok(text);
    }
}
