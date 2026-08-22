use godot::prelude::*;

use crate::game_scripts::game_utils::PlayerCharacterType::{
    Bear, Beaver, Boar, Leshik, Moose, Wolf,
};
pub fn player_color_based_on_number(player_num: u8) -> Color {
    // Golden ratio conjugate for optimal hue spacing
    const GOLDEN_CONJUGATE: f32 = 0.618033988749895; // (sqrt(5) - 1) / 2

    // Generate hue using golden ratio method
    let hue = ((player_num as f32) * GOLDEN_CONJUGATE) % 1.0;

    // Saturation and value with slight variation
    let saturation = 0.85;
    let value = 0.85;

    // Convert HSV to RGB
    let (r, g, b) = hsv_to_rgb(hue, saturation, value);

    Color::from_rgb(r, g, b)
}
pub fn complementary_color(color: Color) -> Color {
    Color {
        r: 1.0 - color.r,
        g: 1.0 - color.g,
        b: 1.0 - color.b,
        a: color.a,
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (v, v, v);
    }

    let i = (h * 6.0) as u8;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => unreachable!(),
    }
}
#[derive(GodotConvert, Var, Export, Default, Clone, Copy, Debug)]
#[godot(via = GString)]
pub enum Direction {
    #[default]
    Left,
    Right,
}
#[derive(GodotConvert, Var, Export, Default, Clone, Copy, Debug)]
#[godot(via = GString)]
pub enum ThrowableSpawnDirection {
    #[default]
    Up,
    Left,
    Right,
}
pub fn vec3_to_vec2(input: Vector3) -> Vector2 {
    Vector2 {
        x: input.x,
        y: input.y,
    }
}
#[derive(GodotConvert, Var, Export, Clone, Copy, Debug)]
#[godot(via = GString)]
pub enum PlayerCharacterType {
    Bear,
    Moose,
    Beaver,
    Wolf,
    Boar,
    Leshik,
}
impl PlayerCharacterType {
    pub fn node_name(&self) -> &'static str {
        match self {
            Self::Bear => "bear",
            Self::Boar => "boar",
            Self::Moose => "moose",
            Self::Beaver => "beaver",
            Self::Wolf => "wolf",
            _ => "leshik",
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Self::Bear => Color::from_html("#753F2F").unwrap_or_default(),
            Self::Boar => Color::from_html("#695648").unwrap_or_default(),
            Self::Moose => Color::from_html("#411A01").unwrap_or_default(),
            Self::Beaver => Color::from_html("#663931").unwrap_or_default(),
            Self::Wolf => Color::from_html("#474747").unwrap_or_default(),
            _ => Color::from_html("#2D421D").unwrap_or_default(),
        }
    }
    pub fn from_str(str: &str) -> Self {
        match str {
            "bear" => Bear,
            "beaver" => Beaver,
            "wolf" => Wolf,
            "boar" => Boar,
            "moose" => Moose,
            _ => Leshik,
        }
    }
}
