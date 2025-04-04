use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::shapes::Circle;
use crate::components::button::Button;
use crate::layout::{Column, Row, Stack, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct DataItem(Row, Option<Number>, DataItemContent);
impl Events for DataItem {}

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
            Row(32, Offset::Start, Size::Fit, Padding::default()),
            number.map(|n| Number::new(ctx, n)),
            DataItemContent::new(ctx, label, text, secondary, table, quick_actions)
        )
    }
}

#[derive(Debug, Component)]
struct Number(Stack, BasicText, Shape);
impl Events for Number {}

impl Number {
    pub fn new(ctx: &mut Context, txt: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.background.secondary, theme.fonts.size.h5);
        Number(
            Stack::center(),
            Text::new(ctx, txt, TextStyle::Heading, font_size),
            Circle::new(32, color), 
        )
    }
}

#[derive(Debug, Component)]
struct DataItemContent(Column, BasicText, Option<BasicText>, Option<BasicText>, Option<Table>, Option<QuickActions>);
impl Events for DataItemContent {}

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
            Column(16, Offset::Start, Size::Fit, Padding::default()),
            Text::new(ctx, label, TextStyle::Heading, font_size.h5),
            text.map(|t| Text::new(ctx, t, TextStyle::Primary, font_size.md)),
            secondary.map(|t| Text::new(ctx, t, TextStyle::Secondary, font_size.sm)),
            table.map(|tabulars| Table::new(ctx, tabulars)),
            quick_actions.map(|actions| QuickActions::new(ctx, actions)),
        )
    }
}

#[derive(Debug, Component)]
struct Table(pub Column, pub Vec<Tabular>);
impl Events for Table {}

impl Table {
    pub fn new(ctx: &mut Context, items: Vec<(&'static str, &'static str)>) -> Self {
        Table (
            Column(0, Offset::Start, Size::Fit, Padding::default()),
            items.iter().map(|(name, data)| Tabular::new(ctx, name, data)).collect()
        )
    }
}

#[derive(Debug, Component)]
struct Tabular(Row, ExpandableText, BasicText);
impl Events for Tabular {}

impl Tabular {
    fn new(ctx: &mut Context, name: &'static str, data: &'static str) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        Tabular (
            Row(0, Offset::Start, Size::Fit, Padding::default()),
            ExpandableText::new(ctx, name, TextStyle::Primary, font_size),
            Text::new(ctx, name, TextStyle::Primary, font_size),
        )
    }
}

#[derive(Debug, Component)]
struct QuickActions(Row, Vec<Button>); // Row should be wrap
impl Events for QuickActions {}

impl QuickActions {
    fn new(ctx: &mut Context, buttons: Vec<Button>) -> Self {
        QuickActions(Row(8, Offset::Start, Size::Fit, Padding::default()), buttons)
    }
}


// let confirm_amount = DataItem {
//     number: Some("2"),
//     label: "Confirm amount",
//     table: vec![
//         ("date", "12/25/20"),
//         ("time", "11:45 PM")
//     ],
//     text: None,
//     secondary_text: None,
//     quick_actions: vec!["Edit amount", "Edit speed"]
// }

// let confirm_address = DataItem {
//     number: Some("1"),
//     label: "Confirm adress",
//     table: Vec::new(),
//     text: Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
//     secondary_text: Some("Bitcoin sent to the wrong address can never be recovered."),
//     quick_actions: vec!["Edit address"]
// }