use std::env;
use gtk4::{Align, Application, ApplicationWindow, Orientation};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, IsA, RangeExt};
use crate::services::{audioin, audioout, brightness};
use crate::widgets::combobox::combobox;
use crate::widgets::Icon;
use crate::widgets::icon_slider::{icon_slider, icon_slider_x};
use crate::widgets::pad_box::pad_box;
use crate::widgets::twos_box::twos_box;

pub mod widgets;
pub mod services;
pub mod utils;

const PAD: i32 = 16;
const WIDTH: i32 = 320 - PAD * 2;

fn main() {
    let _ = gtk4::init();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "-theme" {use crate::widgets::pad_box::pad_box;

            let theme = args.next().expect("Expected theme name after '-theme'!");
            let settings = gtk4::Settings::default().unwrap_or_else(|| gtk4::Settings::builder().build());
            settings.set_gtk_icon_theme_name(Some(theme.as_str()));
        }
    }

    let app = Application::builder().application_id("dev.v22.ControlCenter").build();
    app.connect_activate(build_ui_wrapper);

    let args: Vec<&str> = vec!["-quiet"];
    app.run_with_args(&args);
}

fn build_ui_wrapper(app: &Application) {
    match build_ui() {
        Ok(root) => {
            let root = pad_box(root, 16);

            let window = ApplicationWindow::builder()
                .application(app)
                .title("Control Center")
                .default_width(WIDTH)
                .default_height(320)
                .child(&root)
                .build();

            window.present();
        },
        Err(s) => {
            let label = gtk4::Label::builder()
                .hexpand(true)
                .vexpand(true)
                .halign(Align::Center)
                .valign(Align::Center)
                .label(s)
                .build();

            let window = ApplicationWindow::builder()
                .application(app)
                .title("Control Center")
                .default_width(320)
                .default_height(320)
                .child(&label)
                .build();

            window.present();
        }
    };
}

fn percent(v: u32, max: u32) -> f64 {
    (v as f64) / (max as f64) * 100.0
}

fn unpercent(v: f64, max: u32) -> u32 {
    ((v * 0.01) * (max as f64)) as u32
}

fn build_ui() -> Result<impl IsA<gtk4::Widget>, String> {
    let root = gtk4::Box::new(Orientation::Vertical, 5);
    root.set_homogeneous(false);

    if let Ok((brightness, max_brightness)) = brightness::get_brightness() {
        let b = percent(brightness, max_brightness);
        let slider = icon_slider_x(&["display-brightness-symbolic"], b, move |val| {
            let _ = brightness::set_brightness(unpercent(val, max_brightness));
        });
        root.append(&slider);
    }

    if let Ok(devices) = audioout::get_device_list() {
        let dev = audioout::get_selected_device()?;
        let names: Vec<_> = devices.iter().map(|s| s.as_str()).collect();
        let combo = combobox(&names, &dev, |item| {
            let _ = audioout::set_selected_device(item);
        });

        const ICONS: &'static [Icon] = &[
            "audio-volume-muted-symbolic",
            "audio-volume-low-symbolic",
            "audio-volume-medium-symbolic",
            "audio-volume-high-symbolic",
        ];

        let volume = audioout::get_volume()?;
        let slider = icon_slider_x(ICONS, volume, move |val| {
            let _ = audioout::set_volume((val as u8).min(100));
        });

        let twos = twos_box(slider, combo, WIDTH, 0.8);
        root.append(&twos);
    }

    if let Ok(devices) = audioin::get_input_device_list() {
        let dev = audioin::get_selected_input_device()?;
        let names: Vec<_> = devices.iter().map(|s| s.as_str()).collect();
        let combo = combobox(&names, &dev, |item| {
            println!("selecting microphone: {item}");
            let _ = audioin::set_selected_input_device(item);
        });

        const ICONS: &'static [Icon] = &[
            "microphone-sensitivity-muted-symbolic",
            "microphone-sensitivity-low-symbolic",
            "microphone-sensitivity-medium-symbolic",
            "microphone-sensitivity-high-symbolic",
        ];

        let volume = audioin::get_input_volume()?;
        let slider = icon_slider_x(ICONS, volume, move |val| {
            let _ = audioin::set_input_volume((val as u8).min(100));
        });

        let twos = twos_box(slider, combo, WIDTH, 0.8);
        root.append(&twos);
    }

    //let slider2 = icon_slider("audio-input-microphone-symbolic", 50.0);
    //root.append(&slider2);
//
    //
    //root.append(&slider3);

    Ok(root)
}