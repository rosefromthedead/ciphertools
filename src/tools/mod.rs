use druid::{
    theme::{FOREGROUND_DARK, FOREGROUND_LIGHT},
    widget::{CrossAxisAlignment, Flex, Label, TextBox},
    Data, Lens, Widget, WidgetExt,
};

pub mod base64;
pub mod shift;
pub mod vigenere;

pub use shift::ShiftState;
// pub use vigenere::VigenereState;
// pub use base64::Base64State;

fn text_input<T: Data, L: Lens<T, String> + 'static>(
    name: &'static str,
    lens: L,
) -> impl Widget<T> {
    Flex::<T>::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(crate::soft_label(name))
        .with_child(TextBox::new().lens(lens).expand_width())
        .expand_width()
}

fn titled_panel<D, W>(title: &'static str, subtitle: &'static str, inner: W) -> impl Widget<D>
where
    D: Data,
    W: Widget<D> + 'static,
{
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::row()
                .with_child(
                    Label::new(title)
                        .with_text_size(20.0)
                        .with_text_color(FOREGROUND_LIGHT),
                )
                .with_child(
                    Label::new(subtitle)
                        .with_text_size(20.0)
                        .with_text_color(FOREGROUND_DARK),
                ),
        )
        .with_flex_child(inner, 1.0)
}
