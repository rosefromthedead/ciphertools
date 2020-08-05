use druid::{
    widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, ViewSwitcher, Scroll},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, theme::FOREGROUND_DARK,
};

mod theme;
mod tools;
mod widget;

#[derive(Clone, Data, Lens, Default)]
pub struct State {
    shift: tools::ShiftState,
    selected_tab: usize,
}

fn main() {
    let window = WindowDesc::new(app)
        .title("CipherTools")
        .window_size((800., 600.));

    AppLauncher::with_window(window)
        .configure_env(theme::theme)
        .launch(Default::default())
        .unwrap();
}

fn app() -> impl Widget<State> {
    use widget::tab_selector::{Entry, tab_selector};
    tab_selector(
        vec![Entry::Category("CIPHERS"), Entry::Tab("Shift"), Entry::Tab("Vigenère"), Entry::Category("ENCODING"), Entry::Tab("Base64")],
        State::selected_tab,
        ViewSwitcher::new(|data: &State, env| {
            data.selected_tab
        }, |value, _data, env| {
            match value {
                0 => tools::shift::build_shift_widget().lens(State::shift).boxed(),
                _ => Label::new("Unimplemented").boxed(),
            }
        })
    ).padding(4.0).debug_paint_layout()
}

fn soft_label<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text.to_string())
        .with_text_color(FOREGROUND_DARK)
        .with_text_size(14.0)
        .align_left()
        .padding(2.0)
}
