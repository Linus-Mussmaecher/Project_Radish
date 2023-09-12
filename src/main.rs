#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use good_web_game::*;

mod music;
mod options;
mod scenes;

const WIDTH: f32 = 1200.;
const HEIGHT: f32 = 900.;

const PALETTE: [u32; 16] = [
    0x385074, 0x4670a2, 0x70819d, 0x86a2b8, 0xc0d1de, 0xb2a08a, 0xd9b48a, 0xfeeb9f, 0xffebbc,
    0xf0d1a5, 0x968981, 0x7f7574, 0x484850, 0x313848, 0x1c283e, 0x0b1321,
];

thread_local! {
    /// The font used in this example.
    pub static RETRO: std::cell::RefCell<Option<graphics::Font>>  = std::cell::RefCell::new(None);
    pub static RETRO_M: std::cell::RefCell<Option<graphics::Font>>  = std::cell::RefCell::new(None);
}

fn main() -> GameResult {
    // for debugging
    // std::env::set_var("RUST_BACKTRACE", "full");

    // Fetch and set resource directory.

    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("./resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    // Generate game context and event loop.

    let conf = good_web_game::conf::Conf::default()
        .physical_root_dir(Some(resource_dir))
        .cache(Some(include_bytes!("../resources/resources.tar")));

    good_web_game::start(conf, |ctx, mut gfx_ctx| {
        // Add fonts from the resource folder.
        RETRO.with(|bs| {
            *bs.borrow_mut() = good_web_game::graphics::Font::new_glyph_font_bytes(
                ctx,
                &ctx.filesystem
                    .open("./fonts/retro_gaming.ttf")
                    .unwrap()
                    .bytes
                    .into_inner(),
            )
            .ok();
        });

        RETRO_M.with(|bs| {
            *bs.borrow_mut() = good_web_game::graphics::Font::new_glyph_font_bytes(
                ctx,
                &ctx.filesystem
                    .open("./fonts/retro_mono.otf")
                    .unwrap()
                    .bytes
                    .into_inner(),
            )
            .ok();
        });

        let start_scene = scenes::main_menu::MainMenu::new(ctx, gfx_ctx).unwrap();
        let sm = mooeye::scene_manager::SceneManager::new(start_scene);
        Box::new(sm)
    })
}
