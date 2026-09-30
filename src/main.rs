/* main.rs
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

mod application;
mod application_menu;
mod components;
mod config;
mod main_content;
mod workers;

#[macro_use]
extern crate log;

use application::ApplicationModel;
use config::{APPLICATION_ID, GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::gio;
use relm4::RelmApp;

use crate::application_menu::ApplicationMenu;
fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    let app_menu = ApplicationMenu::new();
    app_menu.register();

    let app = RelmApp::new(APPLICATION_ID);

    // Load and register resources
    let resource_file = format!("{}/zoom_ms_utility.gresource", PKGDATADIR);
    let resources = gio::Resource::load(&resource_file).expect("Could not load resources");
    gio::resources_register(&resources);

    // Debug the resourcesto see if they were built ad registered
    let resource_path = "/io/github/edcorp844/zoommsutility/images/";

    if let Ok(children) =
        gio::resources_enumerate_children(resource_path, gio::ResourceLookupFlags::NONE)
    {
        debug!("=== Registered resources under {resource_path} ===");
        for path in children {
            debug!("  → {}", path);
        }
    } else {
        debug!("No resources found under {resource_path}");
    }

    // Run the app
    app.run::<ApplicationModel>(());
}
