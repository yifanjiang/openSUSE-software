use glib;
use gtk::prelude::*;
use log::debug;
use std::thread;

use crate::notification;
use crate::package_meta::PackageMeta;
use crate::packagekit;
use crate::packagekit::PackagekitState;
use crate::search_row::SearchRow;
use crate::util::{PKmessage, SearchInfo};

#[derive(Clone)]
pub struct SearchPackage {
    search_entry: gtk::SearchEntry,
    list_box: gtk::ListBox,
    stack_box: gtk::Stack,
    pub search_box: gtk::ScrolledWindow,
    progress_bar: gtk::ProgressBar,
    progress: gtk::Box,
    progress_label: gtk::Label,
    notification: notification::Notification,
    packagekit_state: PackagekitState,
    package_meta: PackageMeta,
}

impl SearchPackage {
    pub fn new(
        builder: &gtk::Builder,
        progress_bar: gtk::ProgressBar,
        progress: gtk::Box,
        progress_label: gtk::Label,
        notification: notification::Notification,
        packagekit_state: PackagekitState,
    ) -> Self {
        let search_entry: gtk::SearchEntry = builder.get_object("search_entry").unwrap();
        let list_box: gtk::ListBox = builder.get_object("search_list_box").unwrap();
        let stack_box: gtk::Stack = builder.get_object("stack_box").unwrap();
        let search_box: gtk::ScrolledWindow = builder.get_object("search_box").unwrap();

        let package_meta = PackageMeta::new(search_entry.clone());

        let search = Self {
            search_entry,
            list_box,
            stack_box,
            search_box,
            progress_bar,
            progress,
            progress_label,
            notification,
            packagekit_state,
            package_meta: package_meta,
        };

        search.connect_signal();
        search
    }

    fn connect_signal(&self) {
        let entry: gtk::SearchEntry = self.search_entry.clone();
        let this = self.clone();
        entry.connect_activate(move |entry| {
            let text = entry.get_text();
            if this.packagekit_state.busy() {
                return;
            }
            //this.packagekit_state.set_state(true);
            //this.search_names(text);
            this.search_meta(text);
        });
    }

    fn row_connect_signal(&self, button: &gtk::Button, info: SearchInfo) {
        let installed = info.info == "installed";
        let id = info.id;
        let this = self.clone();
        button.connect_clicked(move |_| {
            if this.packagekit_state.busy() {
                return;
            }
            this.packagekit_state.set_state(true);
            if installed {
                this.remove_packages(id.clone());
            } else {
                this.install_packages(id.clone());
            }
        });
    }

    pub fn update_package_meta(&self) {
        self.package_meta.update_data();
    }

    fn update_list(&self, list: Vec<SearchInfo>) {
        self.clear_list();
        let list_box = &self.list_box;
        for info in list {
            let row = SearchRow::new(info.clone());
            self.row_connect_signal(row.button(), info.clone());
            row.set_title(info.name);
            let v: Vec<&str> = info.id.split(';').collect();
            let subtitle = format!("{}  {}  {}\n{}", v[1], v[2], v[3], info.summary);
            row.set_subtitle(subtitle);
            list_box.add(&row.row().to_owned());
        }
    }

    fn clear_list(&self) {
        let list_box = &self.list_box;
        let children = list_box.get_children();
        for child in children {
            list_box.remove(&child);
        }
    }

    fn update_search_list(&self, list: Vec<SearchInfo>) {
        self.packagekit_state.set_state(false);
        self.update_list(list);
        self.stack_box.set_visible_child(&self.search_box);
    }

    fn update_progress(&self, percentage: i32) {
        self.stack_box.set_visible_child(&self.progress);
        self.progress_bar.set_fraction(percentage as f64 / 100.0);
    }

    fn update_progress_text(&self, text: Option<String>) {
        let s = if text.is_some() {
            text.unwrap()
        } else {
            String::new()
        };
        let v: Vec<&str> = s.split(';').collect();
        if v.len() < 2 {
            self.progress_label.set_text(String::from("").as_str());
            return;
        }
        let fmt = format!("<b>{}</b>  {}", v[0], v[1]);
        self.progress_label.set_markup(fmt.as_str());
    }

    fn show_notification(&self, text: String) {
        self.notification.set_label(text);
        self.update_search_list(vec![]);
    }

    /*
    fn search_names(&self, text: glib::GString) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::search_names(tx, text);
        });

        let this = self.clone();
        self.update_progress(0);
        rx.attach(None, move |message| {
            match message {
                PKmessage::SearchListNew(list) => {
                    debug!("SearchFinish len={}", list.len());
                    this.update_search_list(list);
                }
                PKmessage::Progress((percentage, id)) => {
                    this.update_progress(percentage);
                    this.update_progress_text(id);
                }
                PKmessage::Error(text) => {
                    this.show_notification(text);
                }
                _ => {}
            }
            glib::Continue(true)
        });
    }
    */

    fn search_meta(&self, text: glib::GString) {
        let list = self.package_meta.search(text.to_string());
        debug!("SearchFinish len={}", list.len());
        self.update_search_list(list);
    }

    fn install_packages(&self, id: String) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::install_packages(tx, id);
        });

        let this = self.clone();
        this.update_progress(0);
        rx.attach(None, move |message| {
            match message {
                PKmessage::InstallFinish => {
                    debug!("InstallFinish");
                    this.update_search_list(vec![]);
                }
                PKmessage::Progress((percentage, id)) => {
                    this.update_progress(percentage);
                    this.update_progress_text(id);
                }
                PKmessage::Error(text) => {
                    this.show_notification(text);
                    this.update_search_list(vec![]);
                }
                _ => {}
            }
            glib::Continue(true)
        });
    }

    fn remove_packages(&self, id: String) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::remove_packages(tx, id);
        });

        let this = self.clone();
        this.update_progress(0);
        rx.attach(None, move |message| {
            match message {
                PKmessage::RemoveFinish => {
                    debug!("RemoveFinish");
                    this.update_search_list(vec![]);
                }
                PKmessage::Progress((percentage, id)) => {
                    this.update_progress(percentage);
                    this.update_progress_text(id);
                }
                PKmessage::Error(text) => {
                    this.show_notification(text);
                    this.update_search_list(vec![]);
                }
                _ => {}
            }
            glib::Continue(true)
        });
    }
}
