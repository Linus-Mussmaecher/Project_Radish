use ggez::*;

mod scenes;

mod game_state;

const WIDTH: f32 = 1200.;
const HEIGHT: f32 = 900.;

const PALETTE: [u32; 16] = [
    0x385074, 0x4670a2, 0x70819d, 0x86a2b8, 0xc0d1de, 0xb2a08a, 0xd9b48a, 0xfeeb9f, 0xffebbc,
    0xf0d1a5, 0x968981, 0x7f7574, 0x484850, 0x313848, 0x1c283e, 0x0b1321,
];

fn main() -> GameResult {

    // for debugging
    //std::env::set_var("RUST_BACKTRACE", "1");

    //generate game context (window etc.)

    let (mut ctx, event_loop): (ggez::context::Context, ggez::event::EventLoop<()>) =
        ContextBuilder::new("radish", "Linus Mußmächer")
            .add_resource_path("./resources")
            .window_setup(
                conf::WindowSetup::default()
                    .icon("/sprites/spells/mana.png")
                    .title("Power Defense"),
            )
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
    ctx.gfx.add_font(
        "Retro_M",
        graphics::FontData::from_path(&ctx, "/fonts/retro_mono.otf")?,
    );

    // create Start Scene

    let start_scene = scenes::main_menu::MainMenu::new(&ctx)?;

    //create Scene Manager

    mooeye::scene_manager::SceneManager::new_and_run(event_loop, ctx, start_scene);
}
