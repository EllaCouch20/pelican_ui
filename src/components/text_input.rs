use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::shapes::RoundedRectangle;
use crate::elements::text::{Text,TextStyle, ExpandableText};
use crate::components::button::IconButton;
use crate::layout::{Padding, Column, Stack, Offset, Size, Row};
use crate::PelicanUI;

#[derive(Debug, Clone, Component)]
pub struct TextInput(Column, Option<BasicText>, InputField, Option<BasicText>);
impl Events for TextInput {}

impl TextInput {
    pub fn new(
        ctx: &mut Context,
        label: Option<&'static str>,
        placeholder: &'static str,
        help_text: Option<&'static str>,
        error: Option<&'static str>,
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let subtext = if let Some(err) = error {
            Some(Text::new(ctx, err, TextStyle::Error, font_size.sm))
        } else if let Some(help) = help_text {
            Some(Text::new(ctx, help, TextStyle::Secondary, font_size.sm))
        } else { None };

        let state = match error.is_some() {
            true => InputState::Error,
            false => InputState::Default,
        }; // Error status goes away on hover, on focus (shouldn't)

        TextInput(
            Column(16, Offset::Start, Size::Fit, Padding::default()),
            label.map(|text| Text::new(ctx, text, TextStyle::Heading, font_size.h5)),
            InputField::new(ctx, state, placeholder, icon_button), 
            subtext
        )
    }
}



#[derive(Clone, Debug, Component)]
struct InputField(Stack, InputBackground, InputContent, #[skip] InputState);

impl InputField {
    pub fn new(
        ctx: &mut Context,
        state: InputState,
        placeholder: &'static str,
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>,
    ) -> Self {
        let (background, outline) = state.get_color(ctx);
        let content = InputContent::new(ctx, placeholder, icon_button);
        let width = Size::Fill(content.size(ctx).min_width(), MaxSize::MAX);
        let height = content.size(ctx).min_height().0;
        let background = InputBackground::new(background, outline, width, height);

        InputField(Stack::center(), background, content, state)
    }

    fn set_state(&mut self, ctx: &mut Context, state: InputState) {
        if self.3 != state {
            self.3 = state;
            let (background, outline) = state.get_color(ctx);
            self.1.set_color(background, outline);
        }
    }
}

impl Events for InputField {
    fn on_click(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        match (position.is_some(), self.3) {
            (true, _) => self.set_state(ctx, InputState::Focus),
            (false, InputState::Focus) => self.set_state(ctx, InputState::Default),
            _ => {}
        };

        match self.3 {
            InputState::Focus => *self.2.input().placeholder() = None,
            _ => {
                let len = self.2.input().value().len();
                let pt = self.2.input().get_placeholder();
                match len>0{
                    true => *self.2.input().placeholder() = None,
                    false => *self.2.input().placeholder() = Some(pt)
                }
            }
        }

        // Remove after implementing cursor system 
        *self.2.input().cursor() = match self.3 {
            InputState::Focus => "|".to_string(),
            _ => String::new(),
        };

        true
    }
    fn on_move(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        match (position.is_some(), self.3) {
            (true, InputState::Default) => self.set_state(ctx, InputState::Hover),
            (false, InputState::Hover) => self.set_state(ctx, InputState::Default),
            _ => {}
        };
        true
    }
    fn on_press(&mut self, _ctx: &mut Context, text: String) -> bool {
        match self.3 {
            InputState::Focus => {
                let t = self.2.input().value();
                *t = match text.as_str() {
                    "\u{7f}" => if t.len()>0 {(&t[0..t.len() - 1]).to_string()} else {String::new()}, // delete char
                    _ => t.clone().to_owned()+&text // add character
                };
            },
            _ => {}
        }
        true
    }
}

#[derive(Clone, Debug, Component)]
struct InputBackground(Stack, RoundedRectangle, RoundedRectangle);
impl Events for InputBackground {}

impl InputBackground {
    fn new(bg: Color, oc: Color, width: Size, height: u32) -> Self {
        InputBackground(
            Stack(Offset::Center, Offset::Center, width, Size::Fit, Padding::default()),
            RoundedRectangle::new(0, None, Some(height), 8, bg),
            RoundedRectangle::new(1, None, Some(height), 8, oc)
        )
    }

    fn set_color(&mut self, bg: Color, oc: Color) {
        *self.1.shape().color() = bg;
        *self.2.shape().color() = oc;
    }
}

#[derive(Clone, Debug, Component)]
struct InputContent(Row, Input, Option<IconButton>);
impl Events for InputContent {}

impl InputContent {
    fn new(
        ctx: &mut Context, 
        placeholder: &'static str, 
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>
    ) -> Self {
        InputContent(
            Row(8, Offset::Center, Size::Fit, Padding(8, 8, 8, 8)),
            Input::new(ctx, placeholder),
            icon_button.map(|(icon, on_click)| IconButton::input(ctx, icon, on_click))
        )
    }

    fn input(&mut self) -> &mut Input { &mut self.1 }
}

#[derive(Clone, Debug, Component)]
struct Input(Stack, InputValue, Option<ExpandableText>, #[skip] ExpandableText);
impl Events for Input {}

impl Input {
    fn new(ctx: &mut Context, placeholder: &'static str) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        let placeholder = ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size);
        Input(
            Stack(Offset::Start, Offset::Center, Size::Fit, Size::Fit, Padding(8, 6, 8, 6)),
            InputValue::new(ctx),
            Some(placeholder.clone()),
            placeholder,
        )
    }

    pub fn cursor(&mut self) -> &mut String { self.1.cursor().value() }
    pub fn value(&mut self) -> &mut String { self.1.text().value() }
    pub fn placeholder(&mut self) -> &mut Option<ExpandableText> { &mut self.2 }
    pub fn get_placeholder(&self) -> ExpandableText { self.3.clone() }
}

#[derive(Clone, Debug, Component)]
struct InputValue(Row, BasicText, ExpandableText);
impl Events for InputValue {}

impl InputValue {
    fn new(ctx: &mut Context) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        InputValue(
            Row::center(0),
            Text::new(ctx, "", TextStyle::Primary, font_size),
            ExpandableText::new(ctx, "", TextStyle::White, font_size),
        )
    }

    pub fn text(&mut self) -> &mut BasicText { &mut self.1 }
    pub fn cursor(&mut self) -> &mut ExpandableText {&mut self.2 }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum InputState {
    Default,
    Hover,
    Focus,
    Error
}

impl InputState {
    fn get_color(&self, ctx: &mut Context) -> (Color, Color) { // background, outline
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            InputState::Default => (colors.shades.transparent, colors.outline.secondary),
            InputState::Hover => (colors.background.secondary, colors.outline.secondary),
            InputState::Focus => (colors.shades.transparent, colors.outline.primary),
            InputState::Error => (colors.shades.transparent, colors.status.danger)
        }
    }
}
