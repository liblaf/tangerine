use super::template::Template;
use crate::core::PATTERN_END;
use crate::core::PATTERN_START;
use color_eyre::eyre::Result;
use etcetera::AppStrategy;

pub struct Environment<'a> {
    env: minijinja::Environment<'a>,
}

#[derive(Clone, Debug)]
pub enum Segment {
    Text(String),
    Template(Template),
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        let mut env = minijinja::Environment::new();
        env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

        minijinja_embed::load_templates!(env);
        let strategy = etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
            app_name: "tangerine".into(),
            author: "liblaf".into(),
            top_level_domain: "local".into(),
        })
        .unwrap();
        env.set_loader(minijinja::path_loader(strategy.in_data_dir("templates")));
        Environment { env }
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
                Segment::Template(template) => {
                    match self
                        .env
                        .get_template(&(template.name.to_string() + ".jinja"))
                    {
                        Ok(tmpl) => {
                            let rendered = tmpl.render(&template.context)?;
                            lines.push(rendered);
                        }
                        Err(err) => {
                            lines.extend(template.lines.clone());
                            tracing::error!(error = %err, template = %template.name);
                        }
                    }
                }
            }
        }
        let mut text = lines.join("\n");
        if !text.ends_with("\n") {
            text.push('\n');
        }
        Ok(text)
    }
}
