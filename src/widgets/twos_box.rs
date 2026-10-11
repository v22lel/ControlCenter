use gtk4::{Orientation, PolicyType};
use gtk4::prelude::*;
use gtk4::prelude::IsA;


pub fn twos_box(left: impl IsA<gtk4::Widget>, right: impl IsA<gtk4::Widget>, width: i32, ratio: f32) -> gtk4::Box {
    let wleft = (width as f32 * ratio) as i32;
    let cleft = gtk4::ScrolledWindow::builder()
        .width_request(wleft)
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Never)
        .hexpand(false)
        .child(&left)
        .build();

    let wright = width - wleft;
    let cright = gtk4::ScrolledWindow::builder()
        .width_request(wright)
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Never)
        .hexpand(false)
        .child(&right)
        .build();

    let b = gtk4::Box::new(Orientation::Horizontal, 5);
    b.set_homogeneous(false);
    b.set_width_request(width);

    b.append(&cleft);
    b.append(&cright);
    b
}