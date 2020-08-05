use crate::theme::HOT_COLOUR;
use druid::{
    theme::{BUTTON_BORDER_RADIUS, BUTTON_DARK, PRIMARY_DARK, BACKGROUND_DARK},
    widget::{Flex, Label, LabelText, MainAxisAlignment},
    Affine, BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, Point, Rect,
    RenderContext, Size, UpdateCtx, Widget, Key, Color,
};

#[derive(Copy, Clone)]
pub enum ModeColour {
    Red,
    Green,
    Blue,
}

impl ModeColour {
    fn get_colour(&self, is_selected: bool) -> Key<Color> {
        use crate::theme::*;
        use ModeColour::*;
        match (self, is_selected) {
            (Red, true) => RED,
            (Red, false) => PALE_RED,
            (Green, true) => GREEN,
            (Green, false) => PALE_GREEN,
            (Blue, true) => BLUE,
            (Blue, false) => PALE_BLUE,
        }
    }
}

pub fn mode_selector(modes: &[(&'static str, ModeColour)]) -> impl Widget<usize> {
    let mut flex = Flex::row().main_axis_alignment(MainAxisAlignment::Center);
    for (idx, (name, colour)) in modes.into_iter().enumerate() {
        flex.add_flex_child(Mode::new(name.clone(), *colour, idx, idx == 0, idx == modes.len() - 1), 1.0);
    }
    flex
}

pub struct Mode {
    label: Label<()>,
    label_size: Size,
    colour: ModeColour,
    id: usize,
    is_left: bool,
    is_right: bool,
}

impl Mode {
    pub fn new(
        text: impl Into<LabelText<()>>,
        colour: ModeColour,
        id: usize,
        is_left: bool,
        is_right: bool,
    ) -> Self {
        Mode {
            label: Label::new(text).with_text_size(20.0),
            label_size: Size::ZERO,
            colour,
            id,
            is_left,
            is_right,
        }
    }
}

impl Widget<usize> for Mode {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut usize, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                *data = self.id;
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &usize, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, &(), env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &usize, data: &usize, env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &usize,
        env: &Env,
    ) -> Size {
        let padding = Size::new(8., 4.);
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, &(), env);

        bc.constrain(Size::new(96.0, self.label_size.height + padding.height))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &usize, env: &Env) {
        let size = ctx.size();
        let colour = env.get(if self.id == *data {
            self.colour.get_colour(true)
        } else if ctx.is_hot() {
            self.colour.get_colour(false)
        } else {
            BACKGROUND_DARK
        });

        let background_rounded = Rect::from_origin_size(Point::ORIGIN, size)
            .to_rounded_rect(env.get(BUTTON_BORDER_RADIUS));
        ctx.fill(background_rounded, &colour);

        let mut half_size = size;
        half_size.width /= 2.0;
        let mut midpoint = Point::ORIGIN;
        midpoint.x += half_size.width;
        
        if !self.is_left {
            let unround_left_corners = Rect::from_origin_size(Point::ORIGIN, half_size);
            ctx.fill(unround_left_corners, &colour);
        }

        if !self.is_right {
            let unround_right_corners = Rect::from_origin_size(midpoint, half_size);
            ctx.fill(unround_right_corners, &colour);
        }

        // center-align text
        let label_offset = size.to_vec2() / 2.0 - self.label_size.to_vec2() / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, &(), env);
        });
    }
}
