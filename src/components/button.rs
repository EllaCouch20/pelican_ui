use rust_on_rails::prelude::*;
use crate::theme::colors::ButtonColorScheme;
use crate::PelicanUI;

mod button;
pub use button::{Button, ButtonWidth};

mod icon;
pub use icon::IconButton;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

impl ButtonState {
    pub fn handle(&mut self, _ctx: &mut Context, event: MouseEvent) -> Option<Self> {
        let state = match self {
            ButtonState::Default if event.position.is_some() => {
                match event.state {
                    MouseState::Pressed => Some(ButtonState::Selected),
                    MouseState::Moved => Some(ButtonState::Hover),
                    _ => None
                }
            },
            ButtonState::Selected => {
                match event.state {
                    MouseState::Released if event.position.is_some() => Some(ButtonState::Hover),
                    MouseState::Moved if event.position.is_none() => Some(ButtonState::Default),
                    _ => None
                }
            },
            ButtonState::Hover => {
                match event.state {
                    MouseState::Pressed if event.position.is_some() => Some(ButtonState::Selected),
                    MouseState::Moved if event.position.is_none() => Some(ButtonState::Default),
                    _ => None
                }
            }
            _ => None
        };
        if let Some(state) = state { *self = state; }
        state
    }

    fn color(&self, ctx: &mut Context, style: ButtonStyle) -> ButtonColorScheme {
        let schemes = &ctx.get::<PelicanUI>().theme.colors.button;
        match (style, self) {
            (ButtonStyle::Primary, ButtonState::Default) => schemes.primary_default,
            (ButtonStyle::Primary, ButtonState::Disabled) => schemes.primary_disabled,
            (ButtonStyle::Primary, ButtonState::Hover) => schemes.primary_hover,
            (ButtonStyle::Primary, ButtonState::Selected) => schemes.primary_selected,

            (ButtonStyle::Secondary, ButtonState::Default) => schemes.secondary_default,
            (ButtonStyle::Secondary, ButtonState::Disabled) => schemes.secondary_disabled,
            (ButtonStyle::Secondary, ButtonState::Hover) => schemes.secondary_hover,
            (ButtonStyle::Secondary, ButtonState::Selected) => schemes.secondary_selected,

            (ButtonStyle::Ghost, ButtonState::Default) => schemes.ghost_default,
            (ButtonStyle::Ghost, ButtonState::Disabled) => schemes.ghost_disabled,
            (ButtonStyle::Ghost, ButtonState::Hover) => schemes.ghost_hover,
            (ButtonStyle::Ghost, ButtonState::Selected) => schemes.ghost_selected,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    Large,
    Medium,
}

impl ButtonSize {
    fn content(&self, ctx: &mut Context) -> (u32, u32, u32) { // text size, icon size, spacing
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        match self {
            ButtonSize::Medium => (font_size.md, 16, 4),
            ButtonSize::Large => (font_size.lg, 24, 12)
        }
    }
    fn background(&self) -> (u32, u32) { // height, padding
        match self {
            ButtonSize::Medium => (32, 12),
            ButtonSize::Large => (48, 24)
        }
    }
}
