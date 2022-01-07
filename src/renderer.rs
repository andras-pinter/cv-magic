use crate::parser::Config;

pub struct Renderer {
    renderer: tera::Tera,
    context: tera::Context,
}

impl Renderer {
    pub fn new<P: AsRef<std::path::Path>>(template: P) -> crate::Result<Self> {
        let mut renderer = tera::Tera::default();
        renderer.add_template_file(template, Some("index"))?;

        Ok(Renderer {
            renderer,
            context: tera::Context::default(),
        })
    }

    pub fn add_context(&mut self, config: Config) {
        for (k, v) in config.globals {
            self.context.insert(k, &v)
        }
        self.context.insert("sections", &config.sections);
    }

    pub fn render(&self) -> Result<String, tera::Error> {
        self.renderer.render("index", &self.context)
    }
}

impl TryFrom<&'static str> for Renderer {
    type Error = tera::Error;

    fn try_from(template: &'static str) -> Result<Self, Self::Error> {
        let mut renderer = tera::Tera::default();
        renderer.add_raw_template("index", template)?;

        Ok(Renderer {
            renderer,
            context: tera::Context::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Renderer};

    #[test]
    fn test_template_rendering_globals() {
        let config = Config::try_from(
            r#"
        template = "test.tpl"

        [globals]
        title = "My awesome CV"
        "#,
        )
        .expect("Invalid configuration");
        let mut renderer = Renderer::try_from(
            r#"
        <title>{{title}}</title>
        "#,
        )
        .expect("Invalid template");
        renderer.add_context(config);

        let rendering = renderer.render();
        assert!(
            rendering.is_ok(),
            "Rendering should be ok: {:?}",
            rendering.err()
        );
        assert_eq!(rendering.unwrap().trim(), "<title>My awesome CV</title>")
    }

    #[test]
    fn test_template_rendering_sections() {
        let config = Config::try_from(
            r#"
        template = "test.tpl"

        [sections.first]
        title = "junior sw dev"
        year = "2010"

        [sections.second]
        title = "senior sw dev"
        year = "2017"

        [sections.technologies]
        languages = ["rust", "python"]
        "#,
        )
        .expect("Invalid configuration");
        let mut renderer = Renderer::try_from(
            r#"
        {% for key, section in sections -%}
            {% if key != "technologies" -%}
            <h1>{{section.title}}</h1>
            <h2>{{section.year}}</h2>
            {% endif -%}
        {% endfor -%}
        Recent: {{ sections.second.title }}
        <ul>
            {% for lang in sections.technologies.languages -%}
            <li>{{ lang }}</li>
            {% endfor -%}
        </ul>
        "#,
        )
        .expect("Invalid template");
        renderer.add_context(config);

        let rendering = renderer.render();
        assert!(
            rendering.is_ok(),
            "Rendering should be ok: {:?}",
            rendering.err()
        );
        assert_eq!(
            rendering.unwrap().trim(),
            r#"<h1>junior sw dev</h1>
            <h2>2010</h2>
            <h1>senior sw dev</h1>
            <h2>2017</h2>
            Recent: senior sw dev
        <ul>
            <li>rust</li>
            <li>python</li>
            </ul>"#
        )
    }
}
