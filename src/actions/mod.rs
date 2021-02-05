use gio::prelude::*;
use gtk::prelude::*;
use glib::clone;
use crate::state;

/// Set up all the global actions for the application
pub fn new(state: &state::State) {
    let app = &state.ui.app;
    let runtime = state.app_runtime.clone();

    /* simple actions */
    let quit = gio::SimpleAction::new("quit", None);
    let about = gio::SimpleAction::new("about", None);
    let main_menu = gio::SimpleAction::new("main_menu", None);

    app.add_action(&quit);
    app.add_action(&about);
    app.add_action(&main_menu);

    /* callbacks */
    quit.connect_activate(clone!(@weak app => move |_action, _args| {
        app.quit();
    }));
    about.connect_activate(clone!(
        @strong runtime,
        @weak app => move |_action, _args| {
            runtime.update_state_with(|state| {
                state.about();
            });
    }));
    main_menu.connect_activate(clone!(
        @strong runtime,
        @weak app => move |_action, _args| {
            runtime.update_state_with(|state| {
                state.main_menu();
            });
    }));

    /* bindings */
    app.set_accels_for_action("app.quit", &["<Ctrl>Q"]);
    app.set_accels_for_action("app.main_menu", &["Escape"]);
}