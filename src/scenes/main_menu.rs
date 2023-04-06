use ggez::graphics;
use mooeye::{scene_manager::Scene, UiElement, UiContent};



pub struct MainMenu{
    gui: UiElement<()>,
}

impl MainMenu {
    pub fn new(ctx: &ggez::Context) -> Self{
        let mut text = ggez::graphics::Text::new("Radish!\nPower Defense").set_font("Retro").set_scale(32.).to_owned().to_element_measured(0, ctx);
        text.layout.x_size = text.layout.x_size.to_shrink();
        text.layout.y_size = text.layout.y_size.to_shrink();

        Self { 
            gui: text,
         }
    }
}

impl Scene for MainMenu{
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        Ok(mooeye::scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, mouse_listen: bool) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}