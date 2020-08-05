use super::{text_input, titled_panel};
use crate::{
    soft_label,
    widget::mode_selector::{mode_selector, ModeColour},
};
use druid::{
    theme::PRIMARY_DARK,
    widget::{
        Container, Controller, CrossAxisAlignment, Flex, MainAxisAlignment, Stepper, TextBox,
    },
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

#[derive(Clone, Data, Lens, Default)]
pub struct ShiftState {
    plaintext: String,
    ciphertext: String,
    count: u8,
    mode: usize,
}

impl ShiftState {
    pub fn new() -> Self {
        ShiftState {
            plaintext: String::new(),
            ciphertext: String::new(),
            count: 0,
            mode: 0,
        }
    }
}

struct PlaintextController;

impl<W: Widget<ShiftState>> Controller<ShiftState, W> for PlaintextController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ShiftState,
        env: &Env,
    ) {
        let old_plaintext = data.plaintext.clone();
        child.event(ctx, event, data, env);
        if old_plaintext != data.plaintext {
            data.ciphertext = shift(&data.plaintext, data.count);
        }
    }
}

struct CountController;

impl<W: Widget<ShiftState>> Controller<ShiftState, W> for CountController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ShiftState,
        env: &Env,
    ) {
        let old_count = data.count;
        child.event(ctx, event, data, env);
        if old_count != data.count {
            data.ciphertext = shift(&data.plaintext, data.count);
        }
    }
}

pub fn build_shift_widget() -> impl Widget<ShiftState> {
    let plaintext = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(crate::soft_label("PLAINTEXT"))
        .with_child(TextBox::new().lens(ShiftState::plaintext))
        .controller(PlaintextController);
    let ciphertext = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(crate::soft_label("CIPHERTEXT"))
        .with_child(TextBox::new().lens(ShiftState::ciphertext));

    let input_row = Flex::row()
        .with_flex_child(plaintext, 1.0)
        .with_spacer(4.0)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(soft_label("COUNT"))
                .with_child(
                    Flex::row()
                        .with_child(
                            TextBox::new().parse().lens(
                                ShiftState::count.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0)),
                            ),
                        )
                        .with_child(
                            Stepper::new()
                                .with_range(0., 25.)
                                .with_wraparound(true)
                                .lens(ShiftState::count.map(|x| *x as f64, |x, y| *x = y as u8)),
                        ),
                ),
        );

    let mode_selector = mode_selector(&[
            ("Decrypt", ModeColour::Red),
            ("Encrypt", ModeColour::Green),
            ("Find Key", ModeColour::Blue),
        ])
        .lens(ShiftState::mode);

    let column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(mode_selector)
        .with_spacer(16.0)
        .with_child(input_row)
        .with_spacer(16.0)
        .with_child(ciphertext);

    titled_panel(
        "Shift Cipher",
        " - Shifts each character along the alphabet.",
        column
    )
}
