use crate::widget::{mode_selector::{ModeColour, mode_selector}, titled_panel, input_label};
use druid::{
    widget::{CrossAxisAlignment, Flex, MainAxisAlignment, TextBox, Controller},
    Data, Lens, LensExt, Widget, WidgetExt, EventCtx, Event, Env
};

fn encrypt(plaintext: &str, key: &str) -> Option<String> {
    let mut ciphertext = String::new();
    for (plain, key) in plaintext.chars().zip(key.chars().cycle()) {
        let key_val;
        match key {
            'a'..='z' => {
                key_val = key as u32 - 97;
            },
            'A'..='Z' => {
                key_val = key as u32 - 65;
            },
            _ => return None,
        }

        let mut x = plain as u32;
        match plain {
            'a'..='z' => {
                x %= 97;
                x += key_val as u32;
                x %= 26;
                x += 97;
            },
            'A'..='Z' => {
                x %= 65;
                x += key_val as u32;
                x %= 26;
                x += 65;
            },
            _ => {},
        }
        ciphertext.push(std::char::from_u32(x).unwrap());
    }
    Some(ciphertext)
}

fn decrypt(ciphertext: &str, key: &str) -> Option<String> {
    let mut plaintext = String::new();
    for (cipher, key) in ciphertext.chars().zip(key.chars().cycle()) {
        let key_val;
        match key {
            'a'..='z' => {
                key_val = 26 - (key as u32 - 97);
            },
            'A'..='Z' => {
                key_val = 26 - (key as u32 - 65);
            },
            _ => return None,
        }

        let mut x = cipher as u32;
        match cipher {
            'a'..='z' => {
                x %= 97;
                x += key_val as u32;
                x %= 26;
                x += 97;
            },
            'A'..='Z' => {
                x %= 65;
                x += key_val as u32;
                x %= 26;
                x += 65;
            },
            _ => {},
        }
        plaintext.push(std::char::from_u32(x).unwrap());
    }
    Some(plaintext)
}

fn find_key(plaintext: &str, ciphertext: &str) -> Option<String> {
    let mut key = String::new();
    for (plain, cipher) in plaintext.chars().zip(ciphertext.chars()) {
        match (plain.is_ascii_alphabetic(), cipher.is_ascii_alphabetic()) {
            (true, true) => {
                let difference = (cipher as i32 - plain as i32) % 26;
                let c = std::char::from_u32((difference + 97) as u32).unwrap();
                key.push(c);
            },
            (false, false) => if plain == cipher { continue } else { return None },
            _ => return None,
        }
    }

    'outer: for len in 1..=key.len() / 2 {
        // the key should be entirely ascii, it doesn't make sense for it to not be
        assert!(key.is_char_boundary(len));
        let first = &key[0..len];
        let num_chunks = key.len() / len;
        for i in 1..num_chunks {
            let chunk = &key[i * len..(i + 1) * len];
            if !first.starts_with(chunk) {
                continue 'outer;
            }
        }
        key.truncate(len);
    }

    Some(key)
}

#[derive(Clone, Data, Lens, Default)]
pub struct VigenereState {
    plaintext: String,
    ciphertext: String,
    key: String,
    mode: usize,
}

struct VigenereController;

impl<W: Widget<VigenereState>> Controller<VigenereState, W> for VigenereController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut VigenereState,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);
        match data.mode {
            0 => {
                // Encrypt
                data.ciphertext = encrypt(&data.plaintext, &data.key).unwrap_or_else(|| String::from("Invalid key"));
            }
            1 => {
                // Decrypt
                data.plaintext = decrypt(&data.ciphertext, &data.key).unwrap_or_else(|| String::from("Invalid key"));
            }
            2 => {
                // Find Key
                data.key = find_key(&data.plaintext, &data.ciphertext).unwrap_or_else(|| String::from("Invalid text"));
            }
            _ => panic!("vigenère: wrong mode"),
        }
    }
}

struct DisableWithMode(usize);

impl<W: Widget<VigenereState>> Controller<VigenereState, W> for DisableWithMode {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut VigenereState,
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

pub fn build_vigenere_widget() -> impl Widget<VigenereState> {
    let mode_selector = mode_selector(&[
        ("Encrypt", ModeColour::Green),
        ("Decrypt", ModeColour::Red),
        ("Find Key", ModeColour::Blue),
    ])
    .lens(VigenereState::mode);

    let plaintext = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(input_label("PLAINTEXT").lens(VigenereState::mode.map(|x| *x == 1, |_, _| {})))
        .with_child(TextBox::new().lens(VigenereState::plaintext).expand_width())
        .controller(DisableWithMode(1));

    let key = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(input_label("KEY").lens(VigenereState::mode.map(|x| *x == 2, |_, _| {})))
        .with_child(TextBox::new().lens(VigenereState::key).expand_width())
        .controller(DisableWithMode(2));

    let ciphertext = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(input_label("CIPHERTEXT").lens(VigenereState::mode.map(|x| *x == 0, |_, _| {})))
        .with_child(TextBox::new().lens(VigenereState::ciphertext).expand_width())
        .controller(DisableWithMode(0));

    let column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(mode_selector)
        .with_spacer(2.0)
        .with_child(plaintext)
        .with_spacer(2.0)
        .with_child(key)
        .with_spacer(2.0)
        .with_child(ciphertext)
        .expand_height();

    titled_panel(
        "Vigenère Cipher",
        " - Shifts each character using the repeated key.",
        column,
    )
    .controller(VigenereController)
}
