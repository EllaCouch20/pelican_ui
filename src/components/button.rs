use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use std::collections::HashMap;
use crate::elements::icon::Icon;
use crate::elements::shapes::{RoundedRectangle, Outline};
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::circle_icon::{CircleIcon, CircleIconContent};
use crate::layout::{Stack, Offset, Size, Row, RowOffset};
use crate::PelicanUI;

// Rules:
// Exported structs and enums prefixed with name of the "top-layer" component.
// If a struct or enum isn’t exported, start its name with _.
// First item in a file should be top-layer component struct or enum
// 'User' should never touch the struct, only new functions

pub struct Button(pub _ButtonBackground, pub _ButtonContent, pub ButtonStyle, pub ButtonWidth, pub u32, pub u32);

impl Button {
    pub fn new(
        ctx: &mut Context,
        label: Option<&'static str>,
        photo: Option<CircleIconContent>,
        icon_l: Option<&'static str>,
        icon_r: Option<&'static str>,
        size: ButtonSize,
        width: ButtonWidth,
        style: ButtonStyle,
        state: ButtonState,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        let colors = ButtonColorMap::new(ctx).colors_from(style, state);

        let (text_size, height, icon_size, padding, spacing) = match size {
            ButtonSize::Medium => (font_size.md, 32, 16, 12, 4),
            ButtonSize::Large => (font_size.lg, 48, 24, 24, 12)
        };

        Button(
            _ButtonBackground::new(colors.background, colors.outline, height / 2, height),
            _ButtonContent(
                photo.map(|circle_icon| CircleIcon::new(ctx, circle_icon, None, false, icon_size)),
                icon_l.map(|icon| Icon::new(ctx, icon, colors.label, icon_size)),
                label.map(|text| Text::new(ctx, text, TextStyle::Label(colors.label), text_size)),
                icon_r.map(|icon| Icon::new(ctx, icon, colors.label, icon_size)),
                spacing
            ),
            style, width, height, padding, 
        )
    }
}

impl Component for Button {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let width = match self.3 {
            ButtonWidth::Hug => Component::size(&mut self.0, ctx, max_size).0+(self.5*2),
            ButtonWidth::Expand => max_size.0,
        };

        Container::new(Stack(Offset::Center, Size::Static(width, self.4)), vec![&mut self.0, &mut self.1])
    }

    fn on_click(&mut self, ctx: &mut Context, _max_size: (u32, u32), position: (u32, u32)) {
        println!("been clicked: {:?}", position);
        let colors = ButtonColorMap::new(ctx).colors_from(self.2, ButtonState::Selected);
        if let Shape(ShapeType::RoundedRectangle(_, (_, _), _), c) = &mut self.0.0 {
            *c = colors.background;
        }
    }
}

pub struct _ButtonContent(Option<CircleIcon>, Option<Icon>, Option<BasicText>, Option<Icon>, u32);

impl Component for _ButtonContent {
    fn build(&mut self, _ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![];

        if let Some(icon) = &mut self.3 { children.push(icon); }
        if let Some(label) = &mut self.2 { children.push(label); }
        if let Some(icon) = &mut self.1 { children.push(icon); }
        if let Some(image) = &mut self.0 { children.push(image); }

        Container::new(Row(self.4, RowOffset::Center), children)
        // Container::new(Stack(Offset::default(), Size::default()), children)
    }
}

pub struct _ButtonBackground(pub Shape, pub Shape);

impl _ButtonBackground {
    pub fn new(bg: Color, oc: Color, r: u32, h: u32) -> Self {
        _ButtonBackground(
            RoundedRectangle::new(100, h, r, bg),
            Outline::rounded_rectangle(100, h, r, 1, oc)
        )
    }
}

impl Component for _ButtonBackground {
    fn build(&mut self, _ctx: &mut Context, max_size: (u32, u32)) -> Container {
        if let ShapeType::RoundedRectangle(_, (w, _), _) = &mut self.0.0 {
            *w = max_size.0;
        }
        if let ShapeType::RoundedRectangle(_, (w, _), _) = &mut self.1.0 {
            *w = max_size.0;
        }
        // self.0.resize(max_size.0, 48);
        container![&mut self.0, &mut self.1]
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    Large,
    Medium,
}

#[derive(Default, Clone)]
pub struct ButtonColorMap {
    color_map: HashMap<(ButtonState, ButtonStyle), ButtonColorScheme>,
}

impl ButtonColorMap {
    pub fn new(ctx: &mut Context) -> Self {
        let schemes = &ctx.get::<PelicanUI>().theme.colors.button;
        let mut color_map = HashMap::new();

        color_map.insert((ButtonState::Default, ButtonStyle::Primary), schemes.primary_default);
        color_map.insert((ButtonState::Disabled, ButtonStyle::Primary), schemes.primary_disabled);
        color_map.insert((ButtonState::Hover, ButtonStyle::Primary), schemes.primary_hover);
        color_map.insert((ButtonState::Selected, ButtonStyle::Primary), schemes.primary_selected);

        color_map.insert((ButtonState::Default, ButtonStyle::Secondary), schemes.secondary_default);
        color_map.insert((ButtonState::Disabled, ButtonStyle::Secondary), schemes.secondary_disabled);
        color_map.insert((ButtonState::Hover, ButtonStyle::Secondary), schemes.secondary_hover);
        color_map.insert((ButtonState::Selected, ButtonStyle::Secondary), schemes.secondary_selected);

        color_map.insert((ButtonState::Default, ButtonStyle::Ghost), schemes.ghost_default);
        color_map.insert((ButtonState::Disabled, ButtonStyle::Ghost), schemes.ghost_disabled);
        color_map.insert((ButtonState::Hover, ButtonStyle::Ghost), schemes.ghost_hover);
        color_map.insert((ButtonState::Selected, ButtonStyle::Ghost), schemes.ghost_selected);

        ButtonColorMap{ color_map }
    }

    pub fn colors_from(&self, style: ButtonStyle, state: ButtonState) -> ButtonColorScheme {
        self.color_map.get(&(state, style)).copied().expect("ColorScheme Not Found")
    }
}


impl Button {
    pub fn primary(ctx: &mut Context, label: &'static str) -> Self {
        Button::new(
            ctx,
            Some(label),
            None,
            None,
            None,
            ButtonSize::Large,
            ButtonWidth::Expand,
            ButtonStyle::Primary,
            ButtonState::Default,
        )
    }

    pub fn secondary(
        ctx: &mut Context, 
        icon_l: Option<&'static str>,
        label: &'static str,
        icon_r: Option<&'static str>,
    ) -> Self {
        Button::new(
            ctx,
            Some(label),
            None,
            icon_l,
            icon_r,
            ButtonSize::Medium,
            ButtonWidth::Hug,
            ButtonStyle::Secondary,
            ButtonState::Default,
        )
    }

    // pub fn secondary(
        // icon_l: Option<&'static str>, 
        // label: &'static str, 
        // icon_r: Option<&'static str>
    // ) -> Self {
    //     Self {
    //         label,
    //         size: ButtonSize::Medium,
    //         width: ButtonWidth::Hug,
    //         style: ButtonStyle::Secondary,
    //         photo: None,
    //         icon_l,
    //         icon_r,
    //     }
    // }

    // pub fn ghost(label: &'static str) -> Self {
    //     Self {
    //         label,
    //         size: ButtonSize::Medium,
    //         width: ButtonWidth::Hug,
    //         style: ButtonStyle::Ghost,
    //         photo: None,
    //         icon_l: None,
    //         icon_r: None
    //     }
    // }

    // pub fn keypad(label: &'static str, icon_l: Option<&'static str>) -> Self {
    //     Self {
    //         label,
    //         size: ButtonSize::Large,
    //         width: ButtonWidth::Expand,
    //         style: ButtonStyle::Ghost,
    //         photo: None,
    //         icon_l,
    //         icon_r: None
    //     }
    // }

    // pub fn photo(
    //     label: &'static str, 
    //     photo: resources::Image,
    // ) -> Self {
    //     Self {
    //         label,
    //         size: ButtonSize::Medium,
    //         width: ButtonWidth::Hug,
    //         style: ButtonStyle::Secondary,
    //         photo: Some(photo),
    //         icon_l: None,
    //         icon_r: None
    //     }
    // }

    // pub fn button_row(a: &'static str, b: &'static str) -> Row {
    //     Row(ZERO, 16, Align::Center, vec![Self::primary(a), Self::primary(b)])
    // }

    // pub fn quick_actions(colorss: Vec<(Icon, &'static str)>) -> Wrap {
    //     let children = colorss
    //         .into_iter()
    //         .map(|colors| {
    //             Self::secondary(colors.1, Some(colors.1), None)
    //         }).collect();

    //     Wrap(ZERO, 8, Align::Left, children)
    // }

    // pub fn quick_deselect(colorss: Vec<&'static str>) -> Wrap {
    //     let children = colorss
    //         .into_iter()
    //         .map(|label| {
    //             Self::secondary(label, None, Some(Icon::Close))
    //         }).collect();

    //     Wrap(ZERO, 8, Align::Left, children)
    // }
}