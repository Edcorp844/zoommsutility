use adw::prelude::*;
use gettextrs::gettext;
use gtk::{ Application};

use crate::config;

pub(crate) struct ApplicationMenu {
    app: Application,
}

impl ApplicationMenu {
    pub(crate) fn new() -> ApplicationMenu {
        let app = relm4::main_application();
        ApplicationMenu { app }
    }

    pub(crate) fn register(&self) {
        // --- About ---
        let about_action = gtk::gio::SimpleAction::new("about", None);
        let app_for_about = self.app.clone();
        about_action.connect_activate(move |_, _| {
            Self::show_about_window(&app_for_about);
        });
        self.app.add_action(&about_action);

        // --- Quit ---
        let quit_action = gtk::gio::SimpleAction::new("quit", None);
        let app_clone = self.app.clone();
        quit_action.connect_activate(move |_, _| {
            app_clone.quit();
        });
        self.app.add_action(&quit_action);
        self.app.set_accels_for_action("app.quit", &["<Primary>q"]);
    }

    pub(crate) fn show_about_window(app: &Application) {
        let Some(active_window) = app.active_window() else {
            return;
        };

        let about = adw::AboutDialog::builder()
            .application_name("Zoom MS Utility")
            .application_icon("io.github.edcorp844.zoommsutility")
            .comments(gettext("Zoom MS Utility MS-50G / MS-60B / MS-70CDR for Linux").as_str())
            .version(config::VERSION)
            .developer_name("Edson Frost")
            .website("https://github.com/Edcorp844/gnome_podcasts.git")
            .issue_url("https://github.com/Edcorp844/gnome_podcasts/issues")
            .copyright("© 2026 Edson Frost")
            .license_type(gtk::License::Gpl30)
            .developers(vec!["Frost Edson"])
            .artists(vec!["Frost Edson"])
            .build();

        about.add_acknowledgement_section(Some("g200kg"), &["g200kg"]);

        about.present(Some(&active_window));
    }
}