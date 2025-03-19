use rust_on_rails::prelude::*;
use rust_on_rails::canvas::CanvasItem;

// use std::collections::HashMap;
use std::fs;
// use std::path::Path;
use std::path::PathBuf;
use image::RgbaImage;
use image::DynamicImage;
use fast_image_resize::ResizeAlg;
use fast_image_resize::ResizeOptions;
use fast_image_resize::Resizer;
use fast_image_resize::FilterType;

#[derive(Clone, Copy)]
pub enum IconName {
    Left,
    Close,
    Wallet,
    Profile,
    Group,
    Door,
}

impl IconName {
    fn get(&self) -> &'static str {
        match self {
            IconName::Left => "left",
            IconName::Close => "close",
            IconName::Wallet => "wallet",
            IconName::Profile => "profile",
            IconName::Group => "group",
            IconName::Door => "door"
        }
    }
}

pub struct Icon(Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: IconName, color: &'static str, size: u32) -> Self {
        let svg = resources::Image::svg(ctx, &ctx.load_file(&format!("icons/{}.svg", name.get())).unwrap(), 8.0);
        Icon(Image(ShapeType::Rectangle(0, size), svg, Some(Color::from_hex(color, 255))))
    }
}

impl ComponentBuilder for Icon {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)
    }
}


// let mut dst_image = DynamicImage::new_rgba8(size, size);
// Resizer::new().resize(
//     &DynamicImage::from(RgbaImage::clone(&raw)), 
//     &mut dst_image,
//     &ResizeOptions::new()
//         .resize_alg(ResizeAlg::SuperSampling(FilterType::Bilinear, 8))
//         .fit_into_destination(Some((0.5, 0.5))),
// ).expect("Resize failed");

impl IconResources {
    pub fn init(ctx: &mut Context) -> Self {
        let mut map = HashMap::new();

        map.insert("accounts", ctx.load_image("icons/accounts.svg").unwrap());
        map.insert("add", ctx.load_image("icons/add.svg").unwrap());
        map.insert("app_store", ctx.load_image("icons/app_store.svg").unwrap());
        map.insert("back", ctx.load_image("icons/back.svg").unwrap());
        map.insert("backspace", ctx.load_image("icons/backspace.svg").unwrap());
        map.insert("bitcoin", ctx.load_image("icons/bitcoin.svg").unwrap());
        map.insert("camera", ctx.load_image("icons/camera.svg").unwrap());
        map.insert("cancel", ctx.load_image("icons/cancel.svg").unwrap());
        map.insert("capslock", ctx.load_image("icons/capslock.svg").unwrap());
        map.insert("checkmark", ctx.load_image("icons/checkmark.svg").unwrap());
        map.insert("close", ctx.load_image("icons/close.svg").unwrap());
        map.insert("copy", ctx.load_image("icons/copy.svg").unwrap());
        map.insert("credential", ctx.load_image("icons/credential.svg").unwrap());
        map.insert("delete", ctx.load_image("icons/delete.svg").unwrap());
        map.insert("door", ctx.load_image("icons/door.svg").unwrap());
        map.insert("down", ctx.load_image("icons/down.svg").unwrap());
        map.insert("edit", ctx.load_image("icons/edit.svg").unwrap());
        map.insert("emoji", ctx.load_image("icons/emoji.svg").unwrap());
        map.insert("error", ctx.load_image("icons/error.svg").unwrap());
        map.insert("explore", ctx.load_image("icons/explore.svg").unwrap());
        map.insert("facebook", ctx.load_image("icons/facebook.svg").unwrap());
        map.insert("forward", ctx.load_image("icons/forward.svg").unwrap());
        map.insert("gif", ctx.load_image("icons/gif.svg").unwrap());
        map.insert("group", ctx.load_image("icons/group.svg").unwrap());
        map.insert("heart", ctx.load_image("icons/heart.svg").unwrap());
        map.insert("home", ctx.load_image("icons/home.svg").unwrap());
        map.insert("infinite", ctx.load_image("icons/infinite.svg").unwrap());
        map.insert("info", ctx.load_image("icons/info.svg").unwrap());
        map.insert("instagram", ctx.load_image("icons/instagram.svg").unwrap());
        map.insert("left", ctx.load_image("icons/left.svg").unwrap());
        map.insert("link", ctx.load_image("icons/link.svg").unwrap());
        map.insert("megaphone", ctx.load_image("icons/megaphone.svg").unwrap());
        map.insert("messages", ctx.load_image("icons/messages.svg").unwrap());
        map.insert("microphone", ctx.load_image("icons/microphone.svg").unwrap());
        map.insert("monitor", ctx.load_image("icons/monitor.svg").unwrap());
        map.insert("paste", ctx.load_image("icons/paste.svg").unwrap());
        map.insert("photos", ctx.load_image("icons/photos.svg").unwrap());
        map.insert("play_store", ctx.load_image("icons/play_store.svg").unwrap());
        map.insert("profile", ctx.load_image("icons/profile.svg").unwrap());
        map.insert("qr_code", ctx.load_image("icons/qr_code.svg").unwrap());
        map.insert("radio_filled", ctx.load_image("icons/radio_filled.svg").unwrap());
        map.insert("radio", ctx.load_image("icons/radio.svg").unwrap());
        map.insert("right", ctx.load_image("icons/right.svg").unwrap());
        map.insert("scan", ctx.load_image("icons/scan.svg").unwrap());
        map.insert("search", ctx.load_image("icons/search.svg").unwrap());
        map.insert("send", ctx.load_image("icons/send.svg").unwrap());
        map.insert("settings", ctx.load_image("icons/settings.svg").unwrap());
        map.insert("up", ctx.load_image("icons/up.svg").unwrap());
        map.insert("wallet", ctx.load_image("icons/wallet.svg").unwrap());
        map.insert("warning", ctx.load_image("icons/warning.svg").unwrap());
        map.insert("x", ctx.load_image("icons/x.svg").unwrap());

        Self{map}
    }

    // pub fn get(&self, name: &'static str) -> Handle {
    //     self.map.get(&name).expect("Icon Not Found").clone()
    // }
}
