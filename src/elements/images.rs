use rust_on_rails::prelude::*;
use crate::PelicanUI;

#[derive(Clone, Debug)]
pub struct Icon;

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Image {
        let icon = ctx.get::<PelicanUI>().theme.icons.get(name);
        Image{shape: ShapeType::Rectangle(0, (size, size)), image: icon, color: Some(color)}
    }
}
impl Events for Icon {}

#[derive(Clone, Debug)]
pub struct Brand;

impl Brand {
    pub fn new(ctx: &mut Context, image: resources::Image, size: (u32, u32)) -> Image {
        let color = ctx.get::<PelicanUI>().theme.colors.shades.white;
        Image{shape: ShapeType::Rectangle(0, (size.0, size.1)), image, color: Some(color)}
    }
}
impl Events for Brand {}
