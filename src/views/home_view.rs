use gtk::{prelude::*, Expression, StringList};
use gtk4 as gtk;

pub fn build_home_view(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("CSS"));

    // The container container.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let button = gtk::Button::with_label("hover me!");
    button.add_css_class("button1");

    let entry = gtk::Entry::new();
    entry.add_css_class("entry1");
    entry.set_text("Some text");

    let model = StringList::new(&["option 1", "option 2", "option 3"]);
    let drop_down = gtk::DropDown::new(Some(model), Expression::NONE);

    vbox.append(&button);
    vbox.append(&entry);
    vbox.append(&drop_down);
    // Then we add the container inside our window.
    window.set_child(Some(&vbox));

    application.connect_activate(move |_| {
        window.present();
    });
}
