use gtk4 as gtk;
use gtk::gdk;
use gtk::pango;
use gtk::prelude::*;
use std::{cell::RefCell, env, rc::Rc};

const DEFAULT_WINDOW_HEIGHT: i32 = 620;

struct I18n {
    title: String,
    filter_placeholder: String,
    size_tooltip: String,
    sample_text: String,
}

fn detect_language_code() -> &'static str {
    let raw = env::var("FONT_SELECTOR_LANG")
        .ok()
        .or_else(|| env::var("LC_ALL").ok())
        .or_else(|| env::var("LANG").ok())
        .unwrap_or_default()
        .to_lowercase();

    if raw.starts_with("en") {
        "en"
    } else if raw.starts_with("de") {
        "de"
    } else if raw.starts_with("fr") {
        "fr"
    } else if raw.starts_with("es") {
        "es"
    } else if raw.starts_with("eo") {
        "eo"
    } else {
        "ru"
    }
}

fn i18n_source(code: &str) -> &'static str {
    match code {
        "en" => include_str!("../i18n/en.lang"),
        "de" => include_str!("../i18n/de.lang"),
        "fr" => include_str!("../i18n/fr.lang"),
        "es" => include_str!("../i18n/es.lang"),
        "eo" => include_str!("../i18n/eo.lang"),
        _ => include_str!("../i18n/ru.lang"),
    }
}

fn parse_i18n(source: &str) -> I18n {
    let mut i18n = I18n {
        title: "Font Selector".to_string(),
        filter_placeholder: "Фильтр шрифтов".to_string(),
        size_tooltip: "Размер шрифта (pt)".to_string(),
        sample_text: "Съешь ещё этих французских булок да выпей чаю".to_string(),
    };

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        match key {
            "title" => i18n.title = value.to_string(),
            "filter_placeholder" => i18n.filter_placeholder = value.to_string(),
            "size_tooltip" => i18n.size_tooltip = value.to_string(),
            "sample_text" => i18n.sample_text = value.to_string(),
            _ => {}
        }
    }

    i18n
}

fn load_i18n() -> I18n {
    let code = detect_language_code();
    parse_i18n(i18n_source(code))
}

fn apply_preview_font(label: &gtk::Label, family: &str, size_pt: i32) {
    let mut desc = pango::FontDescription::new();
    desc.set_family(family);
    desc.set_size(size_pt * pango::SCALE);

    let attrs = pango::AttrList::new();
    attrs.insert(pango::AttrFontDesc::new(&desc));
    label.set_attributes(Some(&attrs));
}

fn select_first_visible_font(font_list: &gtk::ListBox) {
    let mut index = 0;
    while let Some(row) = font_list.row_at_index(index) {
        if row.is_visible() {
            font_list.select_row(Some(&row));
            return;
        }
        index += 1;
    }
    font_list.unselect_all();
}

fn move_font_selection(font_list: &gtk::ListBox, step: i32) {
    let start_index = font_list
        .selected_row()
        .map(|row| row.index() + step)
        .unwrap_or_else(|| if step >= 0 { 0 } else { i32::MAX });

    if step >= 0 {
        let mut index = start_index.max(0);
        while let Some(row) = font_list.row_at_index(index) {
            if row.is_visible() {
                font_list.select_row(Some(&row));
                return;
            }
            index += 1;
        }
    } else {
        let mut index = if start_index == i32::MAX {
            let mut last = -1;
            let mut i = 0;
            while let Some(row) = font_list.row_at_index(i) {
                if row.is_visible() {
                    last = i;
                }
                i += 1;
            }
            last
        } else {
            start_index
        };

        while index >= 0 {
            if let Some(row) = font_list.row_at_index(index) {
                if row.is_visible() {
                    font_list.select_row(Some(&row));
                    return;
                }
            }
            index -= 1;
        }
    }
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("io.example.fontselector")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let i18n = load_i18n();

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(
        ".preview-canvas { background-color: #ffffff; border: 1px solid #d8d8d8; border-radius: 6px; }",
    );
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(i18n.title.as_str())
        .default_width(960)
        .default_height(DEFAULT_WINDOW_HEIGHT)
        .build();

    let root = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    root.set_margin_top(12);
    root.set_margin_bottom(12);
    root.set_margin_start(12);
    root.set_margin_end(12);

    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 8);
    sidebar.set_size_request(280, -1);

    let filter_entry = gtk::Entry::new();
    filter_entry.set_placeholder_text(Some(i18n.filter_placeholder.as_str()));

    let size_adjustment = gtk::Adjustment::new(24.0, 8.0, 96.0, 1.0, 4.0, 0.0);
    let size_spin = gtk::SpinButton::new(Some(&size_adjustment), 1.0, 0);
    size_spin.set_tooltip_text(Some(i18n.size_tooltip.as_str()));
    size_spin.set_width_chars(4);

    let font_list = gtk::ListBox::new();
    font_list.set_selection_mode(gtk::SelectionMode::Single);

    let font_scroll = gtk::ScrolledWindow::new();
    font_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    font_scroll.set_vexpand(true);
    font_scroll.set_child(Some(&font_list));

    sidebar.append(&filter_entry);
    sidebar.append(&size_spin);
    sidebar.append(&font_scroll);

    let content = gtk::Paned::new(gtk::Orientation::Vertical);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.set_resize_start_child(false);
    content.set_shrink_start_child(false);
    content.set_resize_end_child(true);
    content.set_shrink_end_child(true);
    content.set_position(DEFAULT_WINDOW_HEIGHT / 4);

    let sample_buffer = gtk::TextBuffer::new(None);
    sample_buffer.set_text(i18n.sample_text.as_str());

    let sample_text = gtk::TextView::with_buffer(&sample_buffer);
    sample_text.set_wrap_mode(gtk::WrapMode::WordChar);
    sample_text.set_monospace(false);
    sample_text.set_vexpand(true);

    let sample_scroll = gtk::ScrolledWindow::new();
    sample_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    sample_scroll.set_child(Some(&sample_text));

    let sample_frame = gtk::Frame::new(None);
    sample_frame.set_child(Some(&sample_scroll));

    let preview_frame = gtk::Frame::new(None);
    preview_frame.set_hexpand(true);
    preview_frame.set_vexpand(true);

    let preview_scroll = gtk::ScrolledWindow::new();
    preview_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    preview_scroll.set_hexpand(true);
    preview_scroll.set_vexpand(true);

    let preview_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    preview_box.add_css_class("preview-canvas");
    preview_box.set_margin_top(16);
    preview_box.set_margin_bottom(16);
    preview_box.set_margin_start(16);
    preview_box.set_margin_end(16);

    let preview_label = gtk::Label::new(Some(i18n.sample_text.as_str()));
    preview_label.set_wrap(true);
    preview_label.set_wrap_mode(pango::WrapMode::WordChar);
    preview_label.set_xalign(0.0);
    preview_label.set_yalign(0.0);
    preview_label.set_selectable(true);
    preview_label.set_hexpand(true);
    preview_label.set_vexpand(true);

    preview_box.append(&preview_label);
    preview_scroll.set_child(Some(&preview_box));
    preview_frame.set_child(Some(&preview_scroll));

    content.set_start_child(Some(&sample_frame));
    content.set_end_child(Some(&preview_frame));

    root.append(&sidebar);
    root.append(&content);
    window.set_child(Some(&root));

    {
        let content = content.clone();
        window.connect_notify_local(Some("height"), move |win, _| {
            let height = win.height();
            if height > 0 {
                content.set_position((height / 4).max(120));
            }
        });
    }

    let mut families: Vec<String> = window
        .pango_context()
        .list_families()
        .iter()
        .map(|f| f.name().to_string())
        .collect();
    families.sort_unstable();
    families.dedup();

    if families.is_empty() {
        families.push("Sans".to_string());
    }

    for family in &families {
        let row = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some(family));
        label.set_xalign(0.0);
        label.set_margin_top(4);
        label.set_margin_bottom(4);
        label.set_margin_start(6);
        label.set_margin_end(6);
        row.set_child(Some(&label));
        font_list.append(&row);
    }

    let filter_text = Rc::new(RefCell::new(String::new()));
    {
        let filter_text = filter_text.clone();
        font_list.set_filter_func(move |row| {
            let query = filter_text.borrow();
            if query.is_empty() {
                return true;
            }

            row.child()
                .and_then(|child| child.downcast::<gtk::Label>().ok())
                .map(|label| label.text().to_string().to_lowercase().contains(query.as_str()))
                .unwrap_or(false)
        });
    }

    if let Some(row) = font_list.row_at_index(0) {
        font_list.select_row(Some(&row));
    }

    let update_font = Rc::new({
        let preview_label = preview_label.clone();
        let font_list = font_list.clone();
        let size_spin = size_spin.clone();
        move || {
            let family = font_list
                .selected_row()
                .and_then(|row| row.child())
                .and_then(|child| child.downcast::<gtk::Label>().ok())
                .map(|label| label.text().to_string())
                .unwrap_or_else(|| "Sans".to_string());
            let size = size_spin.value_as_int();
            apply_preview_font(&preview_label, &family, size);
        }
    });

    {
        let preview_label = preview_label.clone();
        sample_buffer.connect_changed(move |buffer| {
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let text = buffer.text(&start, &end, false);
            preview_label.set_text(text.as_str());
        });
    }

    {
        let update_font = update_font.clone();
        font_list.connect_row_selected(move |_, _| {
            update_font();
        });
    }

    {
        let font_list = font_list.clone();
        let filter_text = filter_text.clone();
        let update_font = update_font.clone();
        filter_entry.connect_changed(move |entry| {
            *filter_text.borrow_mut() = entry.text().to_string().to_lowercase();
            font_list.invalidate_filter();
            let selected_hidden = font_list
                .selected_row()
                .map(|row| !row.is_visible())
                .unwrap_or(true);
            if selected_hidden {
                select_first_visible_font(&font_list);
            }
            update_font();
        });
    }

    {
        let filter_entry = filter_entry.clone();
        let key = gtk::EventControllerKey::new();
        key.connect_key_pressed(move |_, key, _, state| {
            if state.contains(gdk::ModifierType::CONTROL_MASK)
                && (key == gdk::Key::f || key == gdk::Key::F)
            {
                filter_entry.grab_focus();
                filter_entry.select_region(0, -1);
                return gtk::glib::Propagation::Stop;
            }
            gtk::glib::Propagation::Proceed
        });
        window.add_controller(key);
    }

    {
        let filter_entry = filter_entry.clone();
        let font_list = font_list.clone();
        let key = gtk::EventControllerKey::new();
        let filter_entry_for_key = filter_entry.clone();
        key.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                filter_entry_for_key.set_text("");
                return gtk::glib::Propagation::Stop;
            }
            if key == gdk::Key::Down || key == gdk::Key::KP_Down {
                move_font_selection(&font_list, 1);
                return gtk::glib::Propagation::Stop;
            }
            if key == gdk::Key::Up || key == gdk::Key::KP_Up {
                move_font_selection(&font_list, -1);
                return gtk::glib::Propagation::Stop;
            }
            gtk::glib::Propagation::Proceed
        });
        filter_entry.add_controller(key);
    }

    {
        let update_font = update_font.clone();
        size_spin.connect_value_changed(move |_| {
            update_font();
        });
    }

    update_font();
    window.present();
}
