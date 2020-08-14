use druid::{
    widget::{Label, ViewSwitcher},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};

mod theme;
mod tools;
mod widget;

#[derive(Clone, Data, Lens, Default)]
pub struct State {
    shift: tools::shift::ShiftState,
    vigenere: tools::vigenere::VigenereState,
    base64: tools::base64::Base64State,
    selected_tab: usize,
}

fn main() {
    env_logger::init();

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
        ViewSwitcher::new(|data: &State, _env| {
            data.selected_tab
        }, |value, _data, _env| {
            match value {
                0 => tools::shift::build_shift_widget().lens(State::shift).boxed(),
                1 => tools::vigenere::build_vigenere_widget().lens(State::vigenere).boxed(),
                2 => tools::base64::build_base64_widget().lens(State::base64).boxed(),
                _ => Label::new("Unimplemented").boxed(),
            }
        }).expand_height()
    ).padding(4.0)
}
