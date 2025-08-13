use super::constants::PATTERN_END;
use super::constants::PATTERN_START;
use super::template::Template;
use color_eyre::Result;
use etcetera::AppStrategy;

#[derive(Debug)]
pub struct Environment<'a> {
    context: minijinja::Value,
    env: minijinja::Environment<'a>,
}

#[derive(Debug)]
pub enum Segment {
    Template(Template),
    Text(String),
}

impl<'a> Environment<'a> {
    pub fn get_template(
        &self,
        name: &str,
    ) -> std::result::Result<minijinja::Template<'_, '_>, minijinja::Error> {
        self.env.get_template(name)
    }

    #[tracing::instrument(level = "trace")]
    pub fn new() -> Result<Self> {
        let mut env = minijinja::Environment::new();
        env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
        minijinja_embed::load_templates!(env);
        let strategy = etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
            app_name: crate::build::PROJECT_NAME.to_string(),
            author: super::constants::AUTHOR.to_string(),
            top_level_domain: String::new(),
        })?;
        env.set_loader(minijinja::path_loader(strategy.in_data_dir("templates")));
        let context = crate::utils::load_copier_answers()?;
        tracing::debug!("Copier Answers: {}", context);
        Ok(Environment { env, context })
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn parse_text(&self, text: String) -> Result<Vec<Segment>> {
        let mut in_template: bool = false;
        let mut segments: Vec<Segment> = Vec::new();
        let mut template_lines: Vec<String> = Vec::new();
        for line in text.lines() {
            if in_template {
                template_lines.push(line.to_string());
                if PATTERN_END.is_match(line) {
                    in_template = false;
                    let template = Template::from_lines(std::mem::take(&mut template_lines))?;
                    segments.push(Segment::Template(template));
                    continue;
                }
            } else if PATTERN_START.is_match(line) {
                in_template = true;
                template_lines.push(line.to_string());
                continue;
            } else {
                segments.push(Segment::Text(line.to_string()));
                continue;
            }
        }
        Ok(segments)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn render(&self, segments: &[Segment]) -> Result<String> {
        let mut lines: Vec<String> = Vec::new();
        for segment in segments {
            match segment {
                Segment::Text(text) => lines.push(text.to_string()),
                Segment::Template(template) => lines.push(self.render_template(template)?),
            }
        }
        let mut text = lines.join("\n");
        if !text.ends_with("\n") {
            text.push('\n');
        }
        Ok(text)
    }

    #[tracing::instrument(level = "trace", skip_all, fields(template = template.name))]
    pub fn render_template(&self, template: &Template) -> Result<String> {
        let template_name = if template.name.ends_with(".jinja") {
            template.name.to_string()
        } else {
            template.name.to_string() + ".jinja"
        };
        match self.get_template(&template_name) {
            Ok(tmpl) => {
                let context =
                    minijinja::value::merge_maps([self.context.clone(), template.context.clone()]);
                tracing::debug!(%context);
                let rendered = tmpl.render(context)?;
                let rendered = if rendered.trim().lines().next().unwrap().contains("-*-") {
                    rendered.lines().skip(1).collect::<Vec<_>>().join("\n")
                } else {
                    rendered
                };
                let first = template.lines.first().unwrap();
                let last = template.lines.last().unwrap();
                let result = [first, rendered.trim(), last].join("\n");
                Ok(result)
            }
            Err(err) => {
                tracing::error!("{}", err);
                Ok(template.lines.join("\n"))
            }
        }
    }
}
