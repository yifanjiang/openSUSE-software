use crate::util::SearchInfo;
use gtk::prelude::*;
use libhandy::prelude::*;

pub struct SearchRow {
    row: libhandy::ActionRow,
    button: gtk::Button,
}

impl SearchRow {
    pub fn new(info: SearchInfo) -> Self {
        let builder = gtk::Builder::from_resource("/org/openSUSE/software/ui/search_row.ui");
        let row: libhandy::ActionRow = builder.get_object("row").unwrap();
        let button: gtk::Button = builder.get_object("operation_button").unwrap();
        if info.info == "installed" {
            button.set_label("Remove");
        } else {
            button.set_label("Install");
        }

        Self {
            row: row,
            button: button,
        }
    }

    pub fn row(&self) -> &libhandy::ActionRow {
        &self.row
    }

    pub fn button(&self) -> &gtk::Button {
        &self.button
    }

    pub fn set_title(&self, name: String) {
        self.row.set_title(Some(&name.to_string()));
    }

    pub fn set_subtitle(&self, subtitle: String) {
        self.row.set_subtitle(Some(&subtitle.to_string()));
    }
}
