use druid::{
    theme,
    Color, Env, Key,
};
use theme::FOREGROUND_DARK;

pub const HOT_COLOUR: Key<Color> = Key::new("thomhuds.hot_colour");
pub const RED: Key<Color> = Key::new("thomhuds.red");
pub const PALE_RED: Key<Color> = Key::new("thomhuds.pale_red");
pub const GREEN: Key<Color> = Key::new("thomhuds.green");
pub const PALE_GREEN: Key<Color> = Key::new("thomhuds.pale_green");
pub const BLUE: Key<Color> = Key::new("thomhuds.blue");
pub const PALE_BLUE: Key<Color> = Key::new("thomhuds.pale_blue");

pub fn theme(env: &mut Env, _: &crate::State) {
    env.set(theme::BUTTON_BORDER_RADIUS, 4.);
    env.set(theme::FONT_NAME, "Roboto");
    env.set(HOT_COLOUR, Color::grey(0.25));
    env.set(FOREGROUND_DARK, Color::grey(0.6));

    env.set(RED, Color::from_rgba32_u32(0xF44336FF));
    env.set(PALE_RED, Color::from_rgba32_u32(0xEF9A9AFF));
    env.set(GREEN, Color::from_rgba32_u32(0x4CAF50FF));
    env.set(PALE_GREEN, Color::from_rgba32_u32(0xA5D6A7FF));
    env.set(BLUE, Color::from_rgba32_u32(0x2196F3FF));
    env.set(PALE_BLUE, Color::from_rgba32_u32(0x90CAF9FF));
}
