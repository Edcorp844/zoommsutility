/* application_menu.rs
 *
 * Copyright 2026 Frost
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

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