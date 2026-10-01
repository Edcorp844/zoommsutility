/* components/main_menu_button.rs
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
use relm4::{Component, prelude::*};

pub struct MainMenuButton;

#[relm4::component(pub)]
impl Component for MainMenuButton {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            set_icon_name: "open-menu-symbolic",
            set_tooltip_text: Some("Main Menu"),
            add_css_class: "flat",

            #[wrap(Some)]
            set_popover = &gtk::PopoverMenu::from_model(Some(&{
                let menu = gtk::gio::Menu::new();

                let section = gtk::gio::Menu::new();

                section.append(Some("About Zoom MS Utility"), Some("app.about"));

                menu.append_section(None, &section);

                let quit_window_section = gtk::gio::Menu::new();

                let quit_window_item = gtk::gio::MenuItem::new(Some("Quit"), Some("app.quit"));
                quit_window_item.set_attribute_value("accel", Some(&"<Primary>Q".to_variant()));

                quit_window_section.append_item(&quit_window_item);

                menu.append_section(None, &quit_window_section);

                menu
            })) {}
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MainMenuButton {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}