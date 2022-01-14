use crate::Config;
use std::collections::HashMap;
use std::ffi::OsStr;

pub mod prelude {
    pub const PROFILE_PIC: &str = "profile_pic";
    pub const CV_CLASS: &str = "cv.cls";
    pub const CV_TEX: &str = "cv.tex";
}

fn filename(path: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    std::path::PathBuf::from(path.to_string().trim_end_matches("\""))
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(String::from)
        .map(tera::Value::String)
        .ok_or(tera::Error::call_filter("filename", "renderer::filename"))
}

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
        let mut renderer = tera::Tera::new(&template_dir)?;
        renderer.register_filter("filename", filename);

        Ok(Render {
            renderer,
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
