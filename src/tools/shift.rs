use crate::widget::{
    input_label,
    mode_selector::{mode_selector, ModeColour},
    titled_panel,
};
use druid::{
    widget::{Controller, CrossAxisAlignment, Flex, MainAxisAlignment, Stepper, TextBox},
    Data, Env, Event, EventCtx, Lens, LensExt, Widget, WidgetExt,
};

fn shift(plaintext: &str, count: u8) -> String {
    let mut out = String::with_capacity(plaintext.len());
    for mut c in plaintext.chars() {
        let mut x = c as u32;
        match c {
            'a'..='z' => {
                x %= 97;
                x += count as u32;
                x %= 26;
                x += 97;
            }
            'A'..='Z' => {
                x %= 65;
                x += count as u32;
                x %= 26;
                x += 65;
            }
            _ => {}
        }
        c = std::char::from_u32(x).unwrap();
        out.push(c);
    }
    out
}

fn find_key(plaintext: &str, ciphertext: &str) -> Option<u8> {
    let mut current = None;
    for (plain, cipher) in plaintext.chars().zip(ciphertext.chars()) {
        match (plain.is_ascii_alphabetic(), cipher.is_ascii_alphabetic()) {
            (true, true) => {
                if plain.is_ascii_uppercase() != cipher.is_ascii_uppercase() {
                    return None;
                }
                let key = ((cipher as i32 - plain as i32) % 26) as u8;
                if let Some(current) = current {
                    if current != key {
                        return None;
                    }
                } else {
                    current = Some(key as u8);
                }
            }
            (false, false) => continue,
            _ => return None,
        }
    }
    current
}

#[derive(Clone, Data, Lens, Default)]
pub struct ShiftState {
    plaintext: String,
    ciphertext: String,
    count: Option<u8>,
    mode: usize,
}

struct ShiftController;

impl<W: Widget<ShiftState>> Controller<ShiftState, W> for ShiftController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ShiftState,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);
        match data.mode {
            0 => {
                // Encrypt
                if let Some(count) = data.count {
                    data.ciphertext = shift(&data.plaintext, count);
                }
            }
            1 => {
                // Decrypt
                if let Some(count) = data.count {
                    data.plaintext = shift(&data.ciphertext, 26 - count);
                }
            }
            2 => {
                // Find Key
                data.count = find_key(&data.plaintext, &data.ciphertext);
            }
            _ => panic!("shift: wrong mode"),
        }
    }
}

struct DisableWithMode(usize);

impl<W: Widget<ShiftState>> Controller<ShiftState, W> for DisableWithMode {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ShiftState,
        env: &Env,
    ) {
        if data.mode != self.0 {
            child.event(ctx, event, data, env);
            return;
        }
        match event {
            Event::MouseDown(_) | Event::MouseUp(_) | Event::KeyDown(_) | Event::KeyUp(_) => return,
            _ => child.event(ctx, event, data, env),
        }
    }
}

pub fn build_shift_widget() -> impl Widget<ShiftState> {
    let mode_selector = mode_selector(&[
        ("Encrypt", ModeColour::Green),
        ("Decrypt", ModeColour::Red),
        ("Find Key", ModeColour::Blue),
    ])
    .lens(ShiftState::mode);

    let plaintext = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(input_label("PLAINTEXT").lens(ShiftState::mode.map(|x| *x == 1, |_, _| {})))
        .with_child(TextBox::new().lens(ShiftState::plaintext).expand_width())
        .controller(DisableWithMode(1));

    let count = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(input_label("COUNT").lens(ShiftState::mode.map(|x| *x == 2, |_, _| {})))
        .with_child(
            Flex::row()
                .with_child(TextBox::new().lens(ShiftState::count.map(
                    |x| {
                        x.as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| String::from("N/A"))
                    },
                    |x, y| *x = y.parse().ok(),
                )))
                .with_child(
                    Stepper::new()
                        .with_range(0., 25.)
                        .with_wraparound(true)
                        .lens(
                            ShiftState::count
                                .map(|x| x.unwrap_or(0) as f64, |x, y| *x = Some(y as u8)),
                        ),
                ),
        )
        .controller(DisableWithMode(2));

    let ciphertext = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(input_label("CIPHERTEXT").lens(ShiftState::mode.map(|x| *x == 0, |_, _| {})))
        .with_child(TextBox::new().lens(ShiftState::ciphertext).expand_width())
        .controller(DisableWithMode(0));

    let column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(mode_selector)
        .with_spacer(2.0)
        .with_child(plaintext)
        .with_spacer(2.0)
        .with_child(count)
        .with_spacer(2.0)
        .with_child(ciphertext)
        .expand_height();

    titled_panel(
        "Shift Cipher",
        " - Shifts each character along the alphabet.",
        column,
    )
    .controller(ShiftController)
}
