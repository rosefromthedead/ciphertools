use crate::widget::{soft_label, titled_panel};
use druid::{
    widget::{CrossAxisAlignment, Flex, MainAxisAlignment, TextBox, Controller},
    Data, Lens, Widget, WidgetExt, Env, EventCtx, Event,
};

#[derive(Clone, Data, Lens, Default)]
pub struct Base64State {
    plaintext: String,
    base64: String,
    mode: usize,
}

struct Base64Controller;

impl<W: Widget<Base64State>> Controller<Base64State, W> for Base64Controller {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut Base64State, env: &Env) {
        let old_plain = data.plaintext.clone();
        let old_base64 = data.base64.clone();
        child.event(ctx, event, data, env);
        if data.plaintext != old_plain {
            data.base64 = base64::encode(&data.plaintext);
            return;
        } else if data.base64 != old_base64 {
            data.plaintext = base64::decode(&data.base64).ok().map(|vec| String::from_utf8(vec).ok()).flatten().unwrap_or_else(|| String::from("Invalid"));
        }
    }
}

pub fn build_base64_widget() -> impl Widget<Base64State> {
    let plaintext = Flex::column()
        .with_child(soft_label("PLAINTEXT"))
        .with_child(TextBox::new().lens(Base64State::plaintext).expand_width());

    let base64 = Flex::column()
        .with_child(soft_label("BASE64"))
        .with_child(TextBox::new().lens(Base64State::base64).expand_width());

    let column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(plaintext)
        .with_spacer(2.0)
        .with_child(base64)
        .expand_height();

    titled_panel(
        "Base64",
        " - Transforms text into its Base64 representation.",
        column,
    ).controller(Base64Controller)
}
