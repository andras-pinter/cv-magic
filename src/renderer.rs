use crate::Config;

pub struct Render {
    renderer: tera::Tera,
    current_dir: std::path::PathBuf,
}

impl Render {
    pub fn new(current_dir: &std::path::PathBuf) -> crate::error::Result<Self> {
        let template_dir = current_dir
            .join("templates")
            .join("*.tex.tera")
            .to_string_lossy()
            .to_string();
        Ok(Render {
            renderer: tera::Tera::new(&template_dir)?,
            current_dir: current_dir.clone(),
        })
    }

    pub fn render(&self, config: &Config) -> crate::error::Result<String> {
        let ctx = tera::Context::from_serialize(config)?;
        Ok(self.renderer.render("cv.tex.tera", &ctx)?)
    }

    pub fn current_dir(&self) -> &std::path::PathBuf {
        &self.current_dir
    }
}
