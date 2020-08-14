use druid::{
    theme::{FOREGROUND_DARK, FOREGROUND_LIGHT},
    widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment},
    Data, Widget, WidgetExt
};

pub mod mode_selector;
pub mod tab_selector;

pub fn soft_label<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text.to_string())
        .with_text_color(FOREGROUND_DARK)
        .with_text_size(14.0)
        .align_left()
        .padding(2.0)
}

pub fn titled_panel<D, W>(title: &'static str, subtitle: &'static str, inner: W) -> impl Widget<D>
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

pub fn input_label(name: &'static str) -> impl Widget<bool> {
    Flex::row()
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(soft_label(name))
        .with_child(
            Label::dynamic(|locked, _| {
                if *locked {
                    String::from("locked")
                } else {
                    String::new()
                }
            })
            .with_text_color(FOREGROUND_DARK)
            .with_text_size(14.0),
        )
}
