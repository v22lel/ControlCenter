use crate::widgets::Icon;
use gtk4::prelude::{BoxExt, ButtonExt, ObjectExt, RangeExt, ToValue, WidgetExt};
use gtk4::{Adjustment, Align, IconSize, Orientation, ScrollType};

const SPACING: i32 = 5;

pub fn icon_slider<F: Fn(f64) + 'static>(icon: Icon, initial: f64, listener: F) -> gtk4::Box {
    let b = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(SPACING)
        .build();

    let image = gtk4::Image::builder()
        .icon_name(icon)
        .icon_size(IconSize::Large)
        .build();

    let slider = gtk4::Scale::builder()
        .draw_value(false)
        .hexpand(true)
        .adjustment(&Adjustment::new(initial, 0.0, 100.0, 1.0, 10.0, 0.0))
        .build();

    slider.connect_move_slider(move |s, _| {
        listener(s.value());
    });

    b.append(&image);
    b.append(&slider);
    b
}

fn choose_icon(icons: &[Icon], value: f64) -> Icon {
    let value = value * 0.01;
    let len = icons.len();
    let mut i = (len as f64 * value).ceil() as usize;
    if value < 0.001 {
        i = 0;
    }
    let i = i.min(len - 1);
    icons[i]
}

pub fn icon_slider_x<F: Fn(f64) + 'static>(icons: &'static [Icon], initial: f64, listener: F) -> gtk4::Box {
    let b = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(SPACING)
        .build();

    let image = gtk4::Image::builder()
        .icon_name(choose_icon(icons, initial))
        .icon_size(IconSize::Large)
        .build();

    let button = gtk4::Button::builder()
        .child(&image)
        .has_frame(false)
        .build();


    let slider = gtk4::Scale::builder()
        .draw_value(false)
        .hexpand(true)
        .adjustment(&Adjustment::new(initial, 0.0, 100.0, 1.0, 10.0, 0.0))
        .build();

    let slider_clone = slider.clone();
    button.connect_clicked(move |_| {
        let value = slider_clone.value();

        let value = if value == 0.0 {
            //SAFETY: value has been set earlier and i trust the gtk library
            if let Some(prev) = unsafe { slider_clone.steal_data::<f64>("previous") } {
                prev
            } else {
                1.0
            }
        } else {
            //SAFETY: i dont even know what can go wrong!
            unsafe { slider_clone.set_data("previous", value) };
            0.0
        };

        slider_clone.set_value(value);
    });

    let image_clone = image.clone();

    slider.connect_value_changed(move |s| {
        let mut val = s.value();
        if val < 0.001 {
            val = 0.0;
        }

        let icon = choose_icon(icons, val);
        image_clone.set_icon_name(Some(icon));

        listener(val);
    });

    b.append(&button);
    b.append(&slider);
    b
}