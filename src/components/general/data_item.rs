use rust_on_rails::prelude::*;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Circle;
use crate::components::button::Button;
use crate::layout::{Column, Row, Stack, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct DataItem(Row, Option<Number>, DataItemContent);
impl OnEvent for DataItem {}

impl DataItem {
    pub fn new(
        ctx: &mut Context,
        number: Option<&'static str>,
        label: &'static str,
        text: Option<&'static str>,
        secondary: Option<&'static str>,
        table: Option<Vec<(&'static str, &'static str)>>,
        quick_actions: Option<Vec<Button>>,
    ) -> Self {
        DataItem (
            Row(32.0, Offset::Start, Size::Fit, Padding::default()),
            number.map(|n| Number::new(ctx, n)),
            DataItemContent::new(ctx, label, text, secondary, table, quick_actions)
        )
    }
}

#[derive(Debug, Component)]
struct Number(Stack, Shape, Text);
impl OnEvent for Number {}

impl Number {
    pub fn new(ctx: &mut Context, txt: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.background.secondary, theme.fonts.size.h5);
        Number(
            Stack::center(),
            Circle::new(32.0, color),
            Text::new(ctx, txt, TextStyle::Heading, font_size, Align::Left), 
        )
    }
}

#[derive(Debug, Component)]
struct DataItemContent(Column, Text, Option<Text>, Option<Text>, Option<Table>, Option<QuickActions>);
impl OnEvent for DataItemContent {}

impl DataItemContent {
    fn new(
        ctx: &mut Context,
        label: &'static str,
        text: Option<&'static str>,
        secondary: Option<&'static str>,
        table: Option<Vec<(&'static str, &'static str)>>,
        quick_actions: Option<Vec<Button>>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        DataItemContent(
            Column::new(16.0, Offset::Start, Size::fill(), Padding::default()),
            Text::new(ctx, label, TextStyle::Heading, font_size.h5, Align::Left),
            text.map(|t| Text::new(ctx, t, TextStyle::Primary, font_size.md, Align::Left)),
            secondary.map(|t|Text::new(ctx, t, TextStyle::Secondary, font_size.sm, Align::Left)),
            table.map(|tabulars| Table::new(ctx, tabulars)),
            quick_actions.map(|actions| QuickActions::new(actions)),
        )
    }
}

#[derive(Debug, Component)]
struct Table(pub Column, pub Vec<Tabular>);
impl OnEvent for Table {}

impl Table {
    pub fn new(ctx: &mut Context, items: Vec<(&'static str, &'static str)>) -> Self {
        Table (
            Column::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            items.iter().map(|(name, data)| Tabular::new(ctx, name, data)).collect()
        )
    }
}

#[derive(Debug, Component)]
struct Tabular(Row, Text, Text);
impl OnEvent for Tabular {}

impl Tabular {
    fn new(ctx: &mut Context, name: &'static str, data: &'static str) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        Tabular (
            Row(8.0, Offset::Start, Size::Fit, Padding(0.0, 4.0, 0.0, 4.0)),
            Text::new(ctx, name, TextStyle::Primary, font_size, Align::Left),
            Text::new(ctx, data, TextStyle::Primary, font_size, Align::Left),
        )
    }
}

#[derive(Debug, Component)]
struct QuickActions(Row, Vec<Button>); // Row should be wrap
impl OnEvent for QuickActions {}

impl QuickActions {
    fn new(buttons: Vec<Button>) -> Self {
        QuickActions(Row(8.0, Offset::Start, Size::Fit, Padding::default()), buttons)
    }
}