use gtk::{
    glib::{self},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::utils::process::ProcessItem;

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{
        gio::{Icon, ThemedIcon},
        glib::{ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectImplExt, ObjectSubclass},
    };

    use super::*;

    #[derive(Properties)]
    #[properties(wrapper_type = super::ProcessEntry)]
    pub struct ProcessEntry {
        #[property(get = Self::name, set = Self::set_name, type = glib::GString)]
        name: Cell<glib::GString>,
        #[property(get = Self::commandline, set = Self::set_commandline, type = glib::GString)]
        commandline: Cell<glib::GString>,
        #[property(get = Self::user, set = Self::set_user, type = glib::GString)]
        user: Cell<glib::GString>,
        #[property(get = Self::icon, set = Self::set_icon, type = Icon)]
        icon: RefCell<Icon>,
        #[property(get, set)]
        pid: Cell<i32>,

        #[property(get, set)]
        cpu_usage: Cell<f32>,
        #[property(get, set)]
        memory_usage: Cell<u64>,

        pub process_item: RefCell<Option<ProcessItem>>,
    }

    impl Default for ProcessEntry {
        fn default() -> Self {
            Self {
                name: Cell::new(glib::GString::default()),
                commandline: Cell::new(glib::GString::default()),
                user: Cell::new(glib::GString::default()),
                icon: RefCell::new(ThemedIcon::new("generic-process").into()),
                pid: Cell::new(0),

                cpu_usage: Cell::new(0.0),
                memory_usage: Cell::new(0),

                process_item: RefCell::new(None),
            }
        }
    }

    impl ProcessEntry {
        pub fn name(&self) -> glib::GString {
            let name = self.name.take();
            let result = name.clone();
            self.name.set(name);
            result
        }

        pub fn set_name(&self, name: &str) {
            self.name.set(glib::GString::from(name));
        }

        pub fn commandline(&self) -> glib::GString {
            let commandline = self.commandline.take();
            let result = commandline.clone();
            self.commandline.set(commandline);
            result
        }

        pub fn set_commandline(&self, commandline: &str) {
            self.commandline.set(glib::GString::from(commandline));
        }

        pub fn user(&self) -> glib::GString {
            let user = self.user.take();
            let result = user.clone();
            self.user.set(user);
            result
        }

        pub fn set_user(&self, user: &str) {
            self.user.set(glib::GString::from(user));
        }

        pub fn icon(&self) -> Icon {
            let icon = self
                .icon
                .replace_with(|_| ThemedIcon::new("generic-process").into());
            let result = icon.clone();
            self.icon.set(icon);
            result
        }

        pub fn set_icon(&self, icon: &Icon) {
            self.icon.set(icon.clone());
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProcessEntry {
        const NAME: &'static str = "ProcessEntry";
        type Type = super::ProcessEntry;
    }

    impl ObjectImpl for ProcessEntry {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec);
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct ProcessEntry(ObjectSubclass<imp::ProcessEntry>);
}

impl ProcessEntry {
    pub fn new(process_item: ProcessItem, user: &str) -> Self {
        let this: Self = glib::Object::builder()
            .property("name", &process_item.display_name)
            .property("commandline", &process_item.commandline)
            .property("user", user)
            .property("icon", &process_item.icon)
            .property("pid", process_item.pid)
            .build();
        this.set_cpu_usage(process_item.cpu_time_ratio);
        this.set_memory_usage(process_item.memory_usage as u64);
        this.imp().process_item.replace(Some(process_item));
        this
    }

    pub fn update(&self, process_item: ProcessItem) {
        self.set_cpu_usage(process_item.cpu_time_ratio);
        self.set_memory_usage(process_item.memory_usage as u64);
        self.imp().process_item.replace(Some(process_item));
    }

    pub fn process_item(&self) -> Option<ProcessItem> {
        let imp = self.imp();
        let item = imp.process_item.take();
        imp.process_item.replace(item.clone());
        item
    }
}
