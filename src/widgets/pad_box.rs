use gtk4::Align;
use gtk4::prelude::{BoxExt, IsA};

pub fn pad_box(inner: impl IsA<gtk4::Widget>, padding: i32) -> gtk4::Box {
    let b = gtk4::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .halign(Align::Fill) // Prevents centering; lets it fill its slot
        .valign(Align::Fill) // Prevents centering; lets it fill its slot
        .margin_bottom(padding)
        .margin_top(padding)
        .margin_start(padding)
        .margin_end(padding)
        .build();

    b.append(&inner);
    b
}