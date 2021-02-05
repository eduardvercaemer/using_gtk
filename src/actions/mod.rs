use gio::prelude::*;
use gtk::prelude::*;
use glib::clone;
use crate::state;

/// Set up all the global actions for the application
pub fn global(state: &state::State) {
    let app = &state.ui.app;
    let runtime = state.app_runtime.clone();

    /* simple actions */
    let quit = gio::SimpleAction::new("quit", None);
    let about = gio::SimpleAction::new("about", None);
    let game = gio::SimpleAction::new("game", None);
    let main_menu = gio::SimpleAction::new("main_menu", None);

    app.add_action(&quit);
    app.add_action(&about);
    app.add_action(&game);
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
    game.connect_activate(clone!(
        @strong runtime,
        @weak app => move |_action, _args| {
            runtime.update_state_with(|state| {
                state.game();
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

/// set up all the actions for the _game_ section
pub fn game(state: &state::State) {
    let app = &state.ui.app;
    let runtime = state.app_runtime.clone();

    let hit = gio::SimpleAction::new("game.hit", None);
    let heal = gio::SimpleAction::new("game.heal", None);

    app.add_action(&hit);
    app.add_action(&heal);

    hit.connect_activate(clone!(
        @strong runtime
        => move |_, _| {
            runtime.update_state_with(|state| {
                state.game_hit();
            });
        }
    ));
    heal.connect_activate(clone!(
        @strong runtime
        => move |_, _| {
            runtime.update_state_with(|state| {
                state.game_heal();
            });
        }
    ));
}
