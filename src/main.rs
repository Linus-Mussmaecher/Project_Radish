use ggez::{ContextBuilder, conf, graphics, GameResult};
use mooeye::scene_manager::SceneManager;

mod scenes;
use scenes::main_menu::MainMenu;

const WIDTH: f32 = 768.;
const HEIGHT: f32 = 512.;

const PALETTE: [u32;16] = [
    0x385074,
    0x4670a2,
    0x70819d,
    0x86a2b8,
    0xc0d1de,
    0xb2a08a,
    0xd9b48a,
    0xfeeb9f,
    0xffebbc,
    0xf0d1a5,
    0x968981,
    0x7f7574,
    0x484850,
    0x313848,
    0x1c283e,
    0x0b1321,
];

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
        graphics::FontData::from_path(&ctx, "/fonts/retro_gaming.ttf")?,
    );

    // create Start Scene

    let start_scene = MainMenu::new(&ctx);

    //create Scene Manager

    SceneManager::new_and_run(event_loop, ctx, start_scene);
}
