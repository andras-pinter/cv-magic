use crate::parser::Config;
use crate::renderer::Renderer;

pub struct Index {
    renderer: Renderer,
}

impl Index {
    pub fn new(config: Config) -> crate::Result<Self> {
        let mut renderer = Renderer::new(&config.template)?;
        renderer.add_context(config);

        Ok(Index { renderer })
    }

    pub fn render(&self) -> crate::Result<String> {
        Ok(self.renderer.render()?)
    }
}
