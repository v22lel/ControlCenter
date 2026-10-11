use gtk4::pango;
use gtk4::prelude::{Cast, CastNone, ListItemExt, WidgetExt};

pub fn combobox<F: Fn(&str) + 'static>(options: &[&str], default_selected: &str, listener: F, ) -> gtk4::DropDown {
    let string_list = gtk4::StringList::new(options);

    let default_idx = options
        .iter()
        .position(|&opt| opt == default_selected)
        .unwrap_or(0) as u32;

    let button_factory = gtk4::SignalListItemFactory::new();
    button_factory.connect_setup(|_, list_item| {
        let label = gtk4::Label::builder()
            .xalign(0.0)
            .ellipsize(pango::EllipsizeMode::End)
            .build();
        list_item.set_child(Some(&label));
    });
    button_factory.connect_bind(|_, list_item| {
        if let Some(str_obj) = list_item.item().and_downcast::<gtk4::StringObject>() {
            if let Some(label) = list_item.child().and_downcast::<gtk4::Label>() {
                label.set_text(&str_obj.string());
            }
        }
    });

    let popup_factory = gtk4::SignalListItemFactory::new();
    popup_factory.connect_setup(|_, list_item| {
        let label = gtk4::Label::builder()
            .xalign(0.0)
            .build();
        list_item.set_child(Some(&label));
    });
    popup_factory.connect_bind(|_, list_item| {
        if let Some(str_obj) = list_item.item().and_downcast::<gtk4::StringObject>() {
            if let Some(label) = list_item.child().and_downcast::<gtk4::Label>() {
                label.set_text(&str_obj.string());
            }
        }
    });

    let dropdown = gtk4::DropDown::builder()
        .model(&string_list)
        .factory(&button_factory)
        .list_factory(&popup_factory)
        .selected(default_idx)
        .hexpand(true)
        .halign(gtk4::Align::Fill)
        .build();

    dropdown.set_size_request(120, -1);

    dropdown.connect_selected_notify(move |dd| {
        if let Some(item) = dd.selected_item() {
            if let Some(str_obj) = item.downcast_ref::<gtk4::StringObject>() {
                listener(&str_obj.string());
            }
        }
    });

    dropdown
}