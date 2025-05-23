use rust_on_rails::prelude::*;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{ExpandableText, Text, TextStyle};
use crate::components::button::IconButton;
use crate::events::{KeyboardActiveEvent, SetActiveInput};
use crate::layout::{EitherOr, Padding, Column, Stack, Offset, Size, Row, Bin};
use crate::PelicanUI;

use std::sync::mpsc::{self, Receiver};


#[derive(Debug, Component)]
pub struct TextInput(Column, Option<Text>, InputField, Option<Text>, Option<Text>);

impl TextInput {
    pub fn new(
        ctx: &mut Context,
        value: Option<&'static str>,
        label: Option<&'static str>,
        placeholder: &'static str,
        help_text: Option<&'static str>,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        TextInput(
            Column::new(16.0, Offset::Start, Size::fill(), Padding::default()),
            label.map(|text| Text::new(ctx, text, TextStyle::Heading, font_size.h5, Align::Left)),
            InputField::new(ctx, value, placeholder, icon_button),
            help_text.map(|t| Text::new(ctx, t, TextStyle::Secondary, font_size.sm, Align::Left)),
            None
            // SubText::new(ctx, help_text)
        )
    }

    pub fn set_error(&mut self, ctx: &mut Context, error: &'static str) {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        self.4 = Some(Text::new(ctx, error, TextStyle::Error, font_size, Align::Left));
        self.3 = None;
    }

    pub fn set_help(&mut self, ctx: &mut Context, help: &'static str) {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        self.3 = Some(Text::new(ctx, help, TextStyle::Secondary, font_size, Align::Left));
        self.4 = None;
    }

    pub fn get_error(&mut self) -> &mut bool {self.2.error()}
    pub fn get_value(&mut self) -> &mut String {self.2.input()}
}

impl OnEvent for TextInput {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            *self.2.error() = self.4.is_some();
        }
        true
    }
}

#[derive(Debug, Component)]
struct InputField(Stack, OutlinedRectangle, InputContent, #[skip] InputState, #[skip] bool);

impl InputField {
    pub fn new(
        ctx: &mut Context,
        value: Option<&'static str>,
        placeholder: &'static str,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let (background, outline) = InputState::Default.get_color(ctx);
        let content = InputContent::new(ctx, value, placeholder, icon_button);
        let background = OutlinedRectangle::new(background, outline, 8.0, 1.0);

        InputField(Stack(
            Offset::Start, Offset::Start, Size::fill(),
            Size::custom(|heights: Vec<(f32, f32)>| (
                if heights[1].0 > 48.0 {heights[1].0} else {48.0},
                if heights[1].1 > 48.0 {heights[1].1} else {48.0}
            )),
            Padding::default()
        ), background, content, InputState::Default, false)
    }

    pub fn error(&mut self) -> &mut bool { &mut self.4 }
    pub fn input(&mut self) -> &mut String { &mut self.2.text().text().spans[0].text }
}

impl OnEvent for InputField {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            self.2.text().cursor().as_mut().map(|c| c.display(self.3 == InputState::Focus));
            self.3 = match self.3 {
                InputState::Default if self.4 => Some(InputState::Error),
                InputState::Error if !self.4 => Some(InputState::Default),
                _ => None
            }.unwrap_or(self.3);

            let (background, outline) = self.3.get_color(ctx);
            *self.1.background() = background;
            *self.1.outline() = outline;
            *self.2.focus() = self.3 == InputState::Focus;
        } else if let Some(SetActiveInput(s)) = event.downcast_ref::<SetActiveInput>() {
            *self.input() = s.to_string();
        } else if let Some(KeyboardActiveEvent(enabled)) = event.downcast_ref::<KeyboardActiveEvent>() {
            if !enabled && self.3 == InputState::Focus {
                if self.4 { self.3 = InputState::Error } else { self.3 = InputState::Default }
            }
        } else if let Some(event) = event.downcast_ref::<MouseEvent>() {
            self.3 = match self.3 {
                InputState::Default => {
                    match event {
                        MouseEvent{state: MouseState::Pressed, position: Some(_)} => {ctx.trigger_event(KeyboardActiveEvent(true)); Some(InputState::Focus)},
                        MouseEvent{state: MouseState::Moved, position: Some(_)} => Some(InputState::Hover),
                        _ => None
                    }
                },
                InputState::Hover => {
                    match event {
                        MouseEvent{state: MouseState::Pressed, position: Some(_)} => Some(InputState::Focus),
                        MouseEvent{state: MouseState::Moved, position: None} if self.4 => Some(InputState::Error),
                        MouseEvent{state: MouseState::Moved, position: None} => Some(InputState::Default),
                        _ => None
                    }
                },
                InputState::Focus => {
                    match event {
                        MouseEvent{state: MouseState::Pressed, position: None} if self.4 && !crate::config::IS_MOBILE => Some(InputState::Error),
                        MouseEvent{state: MouseState::Pressed, position: None} if !crate::config::IS_MOBILE => Some(InputState::Default),
                        _ => None
                    }
                },
                InputState::Error => {
                    match event {
                        MouseEvent{state: MouseState::Pressed, position: Some(_)} => Some(InputState::Focus),
                        MouseEvent{state: MouseState::Moved, position: Some(_)} => Some(InputState::Hover),
                        _ => None
                    }
                }
            }.unwrap_or(self.3);
        } else if let Some(KeyboardEvent{state: KeyboardState::Pressed, key}) = event.downcast_ref() {
            if self.3 == InputState::Focus {
                if let Some((i, _)) = self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::GetIndex) {
                    match key {
                        Key::Named(NamedKey::Enter) => {
                            self.2.text().text().spans[0].text.insert_str(i as usize, "\n");
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveNewline);
                        },
                        Key::Named(NamedKey::Space) => {
                            self.2.text().text().spans[0].text.insert_str(i as usize, " ");
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveRight);
                        },
                        Key::Named(NamedKey::Delete | NamedKey::Backspace) if (!self.2.text().text().spans[0].text.is_empty() && i as usize != 0) => {
                            self.2.text().text().spans[0].text.remove(i as usize -1);
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveLeft);
                        },
                        Key::Character(c) => {
                            println!("MOVING RIGHT {:?}", c);
                            self.2.text().text().spans[0].text.insert_str(i as usize , c);
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveRight);
                        },
                        _ => {}
                    };
                }
            }
        }
        true
    }
}

pub type SubmitCallback = Box<dyn FnMut(&mut Context, &mut String)>;

#[derive(Component)]
struct InputContent(
    Row, Bin<Stack, EitherOr<ExpandableText, ExpandableText>>, Option<IconButton>,
    #[skip] bool, #[skip] Option<(Receiver<u8>, SubmitCallback)>
);

impl InputContent {
    pub fn new(
        ctx: &mut Context,
        value: Option<&'static str>,
        placeholder: &'static str,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        let (icon_button, callback) = icon_button.map(|(icon, on_click)| {
            let (sender, receiver) = mpsc::channel();
            (
                Some(IconButton::input(ctx, icon, move |_| {sender.send(0).unwrap();})),
                Some((receiver, Box::new(on_click) as SubmitCallback)),
            )
        }).unwrap_or((None, None));

        InputContent(
            Row(0.0, Offset::End, Size::Fit, Padding(16.0, 8.0, 8.0, 8.0)),
            Bin(
                Stack(Offset::default(), Offset::End, Size::fill(), Size::Fit, Padding(8.0, 8.0, 8.0, 8.0)),
                EitherOr::new(
                    ExpandableText::new_with_cursor(ctx, value.unwrap_or(""), TextStyle::Primary, font_size, Align::Left),
                    ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size, Align::Left)
                )
            ),
            icon_button,
            false,
            callback,
        )
    }

    pub fn text(&mut self) -> &mut ExpandableText { self.1.inner().left() }
    pub fn focus(&mut self) -> &mut bool {&mut self.3}
}

impl OnEvent for InputContent {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            if let Some((receiver, on_submit)) = self.4.as_mut() {
                if receiver.try_recv().is_ok() {
                    on_submit(ctx, &mut self.1.inner().left().0.text().spans[0].text)
                }
            }

            let input = !self.1.inner().left().0.text().spans[0].text.is_empty();
            self.1.inner().display_left(input || self.3)
        }
        true
    }
}

impl std::fmt::Debug for InputContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InputContent(...)")
    }
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
