mod libs;
mod views;

use gtk::{gdk, glib, prelude::*};
use gtk4 as gtk;
use libs::files::{ensure_configs, ensure_styles};
use views::home_view::build_home_view;

const APPLICATION_ID: &str = "br.com.unixlab.AppLauncher";

fn main() {
    let _configs = ensure_configs();
    let styles_path = ensure_styles();

    build_application(styles_path);
}

fn build_application(path: String) -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    application.connect_startup(move |app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(path.as_str());
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        build_home_view(app)
    });

    application.run()
}
