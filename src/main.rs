use ggez::{ContextBuilder, conf, graphics, GameResult};
use mooeye::scene_manager::SceneManager;

mod scenes;
use scenes::main_menu::MainMenu;

const WIDTH: f32 = 768.;
const HEIGHT: f32 = 512.;

fn main() -> GameResult {
    //code snippet to fetch and set resource dir

    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    //generate game context (window etc.)

    let (mut ctx, event_loop): (ggez::context::Context, ggez::event::EventLoop<()>) =
        ContextBuilder::new("Mooeye Test", "Linus Mußmächer")
            .add_resource_path(resource_dir)
            .window_setup(conf::WindowSetup::default().title("Rosemary"))
            .window_mode(
                conf::WindowMode::default()
                    .fullscreen_type(conf::FullscreenType::Windowed)
                    .resizable(true)
                    .dimensions(WIDTH, HEIGHT),
            )
            .build()?;

    //add fonts

    ctx.gfx.add_font(
        "Retro",
        graphics::FontData::from_path(&ctx, "/retro_gaming.ttf")?,
    );

    // create Start Scene

    let start_scene = MainMenu::new(&ctx);

    //create Scene Manager

    SceneManager::new_and_run(event_loop, ctx, start_scene);
}
