use crate::{soft_label, theme::HOT_COLOUR};
use druid::{
    theme::{BUTTON_BORDER_RADIUS, BUTTON_DARK, PRIMARY_DARK},
    widget::{Flex, Label, LabelText, MainAxisAlignment},
    Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx,
    Point, Rect, RenderContext, Selector, Size, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod,
};

pub enum Entry {
    Tab(&'static str),
    Category(&'static str),
}

pub fn tab_selector<T: Data>(
    entries: Vec<Entry>,
    lens: impl Lens<T, usize> + 'static,
    content: impl Widget<T> + 'static,
) -> impl Widget<T> {
    let mut main = Flex::row().must_fill_main_axis(true);

    let mut col = Flex::column().main_axis_alignment(MainAxisAlignment::Start);
    let mut tab_id = 0;
    for entry in entries {
        match entry {
            Entry::Tab(name) => {
                col.add_child(Tab::new(name, tab_id));
                tab_id += 1;
            }
            Entry::Category(name) => col.add_child(soft_label(&name)),
        }
    }
    let col = col.fix_width(128.0);

    let content = content.padding(4.0).border(PRIMARY_DARK, 1.0).rounded(1.0).expand();

    main.add_child(col.lens(lens));
    main.add_child(content);

    main
}

pub struct Tab {
    label: Label<()>,
    label_size: Size,
    id: usize,
}

impl Tab {
    pub fn new(text: impl Into<LabelText<()>>, id: usize) -> Self {
        Tab {
            label: Label::new(text).with_text_size(20.0),
            label_size: Size::ZERO,
            id,
        }
    }
}

impl Widget<usize> for Tab {
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
        if *old_data == self.id || *data == self.id {
            ctx.request_paint();
        }
        self.label.update(ctx, &(), &(), env)
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

        bc.constrain(Size::new(128., self.label_size.height + padding.height))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &usize, env: &Env) {
        let size = ctx.size();
        let colour = match (self.id == *data, ctx.is_hot()) {
            (true, _) => env.get(PRIMARY_DARK),
            (false, true) => env.get(HOT_COLOUR),
            (false, false) => env.get(BUTTON_DARK),
        };

        let background_rounded = Rect::from_origin_size(Point::ORIGIN, size)
            .to_rounded_rect(env.get(BUTTON_BORDER_RADIUS));

        let mut half_size = size;
        half_size.width /= 2.;
        let mut midpoint = Point::ORIGIN;
        midpoint.x += half_size.width;
        let unround_right_corners = Rect::from_origin_size(midpoint, half_size);

        ctx.fill(background_rounded, &colour);
        ctx.fill(unround_right_corners, &colour);

        // right-align text
        let label_offset = size.to_vec2() - self.label_size.to_vec2() - Vec2::from((0., 2.));

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, &(), env);
        });
    }
}
