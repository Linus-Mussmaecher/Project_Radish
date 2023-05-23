pub mod game_over_menu;
pub mod game_ui;
pub mod in_game_menu;
pub mod main_menu;
pub mod wave_menu;
pub mod game_state;

const BUTTON_VIS: mooeye::ui_element::Visuals = mooeye::ui_element::Visuals {
    background: {
        let c = crate::PALETTE[0].to_be_bytes();
        ggez::graphics::Color::new(
            c[1] as f32 / 255.,
            c[2] as f32 / 255.,
            c[3] as f32 / 255.,
            1.,
        )
    },
    border: {
        let c = crate::PALETTE[7].to_be_bytes();
        ggez::graphics::Color::new(
            c[1] as f32 / 255.,
            c[2] as f32 / 255.,
            c[3] as f32 / 255.,
            1.,
        )
    },
    border_widths: [3.; 4],
    corner_radii: [3.; 4],
};

const BUTTON_HOVER_VIS: mooeye::ui_element::Visuals = mooeye::ui_element::Visuals {
    background: {
        let c = crate::PALETTE[1].to_be_bytes();
        ggez::graphics::Color::new(
            c[1] as f32 / 255.,
            c[2] as f32 / 255.,
            c[3] as f32 / 255.,
            1.,
        )
    },
    border: {
        let c = crate::PALETTE[7].to_be_bytes();
        ggez::graphics::Color::new(
            c[1] as f32 / 255.,
            c[2] as f32 / 255.,
            c[3] as f32 / 255.,
            1.,
        )
    },
    border_widths: [3.; 4],
    corner_radii: [3.; 4],
};
