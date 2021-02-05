use gtk::prelude::*;
use crate::ui;
use crate::app;
use crate::actions;

/*
macro_rules! update {
    ($fn:ident, ($($x:ident),*)) => {
        $( let $x = $x.clone(); )*
        crate::app::get_app_runtime().update_state_with(move |state| {
            state.fn($($x),*);
        });
    };
    ($fn:ident) => {
        update!($fn, ());
    };
}
*/

pub struct State {
    pub app_runtime: app::AppRuntime,
    pub ui: ui::UI,
}

impl State {
    /// create the initial state
    pub fn new(ui: ui::UI, app_runtime: app::AppRuntime) -> Self {
        Self {
            app_runtime,
            ui,
        }
    }

    /// init process of state
    pub fn init(&mut self) {
        self.actions();
        self.main_menu();
        self.ui.win.show();
        self.ui.win.present();
    }

    /// load the main menu
    pub fn main_menu(&mut self) {
        if let Some(child) = self
            .ui
            .win
            .get_children()
            .get(0) {
            self
            .ui
            .win
            .remove(child);
        }

        let root: gtk::Box = self
            .ui
            .builder
            .get_object("main_menu")
            .expect("object: main_menu");
        self
            .ui
            .win
            .add(&root);
    }

    /// load the about page
    pub fn about(&mut self) {
        if let Some(child) = self
            .ui
            .win
            .get_children()
            .get(0) {
            self
            .ui
            .win
            .remove(child);
        }

        let root: gtk::Box = self
            .ui
            .builder
            .get_object("about")
            .expect("objcet: about");
        self
            .ui
            .win
            .add(&root);
    }

    /// set up the global actions
    fn actions(&mut self) {
        actions::new(&self);
    }
}