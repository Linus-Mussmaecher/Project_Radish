
use ggez::{graphics::Color, *};

use mooeye::{*};
use std::time::Duration;

use crate::PALETTE;

use super::game_message::GameMessage;



pub fn construct_game_ui(ctx: &Context) -> Result<UiElement<GameMessage>, GameError>{
    let box_vis = mooeye::ui_element::Visuals {
        background: Color::from_rgb_u32(PALETTE[0]),
        border: Color::from_rgb_u32(PALETTE[7]),
        border_width: 3.,
        rounded_corners: 6.,
    };

    // main box
    let mut main_box = containers::StackBox::new();

    // options icon
    let cog_icon = graphics::Image::from_path(ctx, "/sprites/cog.png")?
        .to_element_builder(1, ctx)
        .with_visuals(box_vis)
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Max)
        .scaled(2., 2.)
        .with_offset(-10., -10.)
        .as_shrink()
        .build();

    main_box.add(cog_icon)?;

    // gold display
    let gold_icon = sprite::Sprite::from_path_fmt(
        "/sprites/coin_16_16.png",
        ctx,
        Duration::from_secs_f32(0.25),
    )?
    .to_element_builder(0, ctx)
    .scaled(2., 2.)
    .build();

    let gold_text = graphics::Text::new(
        graphics::TextFragment::new("0000").color(Color::from_rgb_u32(PALETTE[6])),
    )
    .set_scale(32.)
    .set_font("Retro")
    .to_owned()
    .to_element_builder(0, ctx)
    .with_message_handler(|message_set, _, transitions| {
        for message in message_set {
            if let ui_element::UiMessage::Extern(GameMessage::UpdateGold(new_gold)) = message {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO).with_new_content(
                        graphics::Text::new(
                            graphics::TextFragment::new(format!("{:04}", *new_gold))
                                .color(Color::from_rgb_u32(PALETTE[6])),
                        )
                        .set_scale(32.)
                        .set_font("Retro")
                        .to_owned(),
                    ),
                );
            }
        }
    })
    .build();

    let mut gold_box = containers::HorizontalBox::new();
    gold_box.add(gold_icon)?;
    gold_box.add(gold_text)?;
    let gold_box = gold_box
        .to_element_builder(0, ctx)
        .with_visuals(box_vis)
        .with_alignment(ui_element::Alignment::Min, ui_element::Alignment::Min)
        .with_offset(10., 10.)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("Your current amount of gold.")
                    .color(Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_tooltip_layout()
            .with_visuals(box_vis)
            .build(),
        )
        .build();

    main_box.add(gold_box)?;

    // city health display

    let city_display = sprite::Sprite::from_path_fmt(
        "/sprites/city_16_16.png",
        ctx,
        Duration::from_secs_f32(0.25),
    )?
    .to_element_builder(0, ctx)
    .scaled(2., 2.)
    .build();

    let city_text = graphics::Text::new(
        graphics::TextFragment::new("100").color(Color::from_rgb_u32(PALETTE[6])),
    )
    .set_scale(32.)
    .set_font("Retro")
    .to_owned()
    .to_element_builder(0, ctx)
    .with_message_handler(|message_set, _, transitions| {
        for message in message_set {
            if let ui_element::UiMessage::Extern(GameMessage::UpdateCityHealth(new_health)) =
                message
            {
                transitions.push_back(
                    ui_element::Transition::new(Duration::ZERO).with_new_content(
                        graphics::Text::new(
                            graphics::TextFragment::new(format!("{:03}", *new_health))
                                .color(Color::from_rgb_u32(PALETTE[6])),
                        )
                        .set_scale(32.)
                        .set_font("Retro")
                        .to_owned(),
                    ),
                );
            }
        }
    })
    .build();

    let mut city_box = containers::HorizontalBox::new();
    city_box.add(city_display)?;
    city_box.add(city_text)?;
    let city_box = city_box
        .to_element_builder(0, ctx)
        .with_visuals(box_vis)
        .with_alignment(ui_element::Alignment::Max, ui_element::Alignment::Min)
        .with_offset(-10., 10.)
        .with_tooltip(
            graphics::Text::new(
                graphics::TextFragment::new("The health your city currently has left.")
                    .color(Color::from_rgb_u32(PALETTE[6])),
            )
            .set_scale(24.)
            .set_font("Retro")
            .to_owned()
            .to_element_builder(0, ctx)
            .with_tooltip_layout()
            .with_visuals(box_vis)
            .build(),
        )
        .build();

    main_box.add(city_box)?;

    Ok(main_box.to_element_builder(0, ctx).as_fill().build())
}

struct Covering{
    covering: f32,
    color: graphics::Color,
}

impl Covering{
    pub fn new(color: graphics::Color) -> Self{
        Self{
            covering: 1.,
            color,
        }
    }

    pub fn set_covering(&mut self, covering: f32){
        self.covering = covering;
    }
}

impl UiContent<GameMessage> for Covering{
    fn draw_content(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas, param: ui_element::UiDrawParam) {
        let mut target_mod = param.target;
        target_mod.y += (1.-self.covering) * target_mod.h;
        target_mod.h *= self.covering;
        canvas.draw(&graphics::Quad, param.param.dest_rect(target_mod).color(self.color));
    }
}