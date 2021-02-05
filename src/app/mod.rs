use gio::prelude::*;
use glib::clone;
use crate::state;
use crate::ui;

/// Represents the global app runtime with which we can interact
/// with the main application context
static mut APP_RUNTIME: Option<AppRuntime> = None;

#[derive(Clone)]
pub struct AppRuntime(glib::Sender<Box<dyn FnOnce(&mut state::State)>>);

impl AppRuntime {
    /// Create the application runtime from our UI
    fn init(ui: ui::UI) -> Self {
        // start the runtime
        let (tx, rx) = glib::MainContext::channel(Default::default());
        let app_runtime = Self(tx);
        unsafe {
            APP_RUNTIME = Some(app_runtime.clone());
        }

        // start the state
        let mut state = state::State::new(ui, app_runtime.clone());

        // on the receiveing end of the main context channel, we
        // receive functions, which we call on the state of the application
        // to update the state
        rx.attach(None, move |update_fn| {
            // here the main context _owns_ the applciatio state,
            // that we get to mutate via the _functions_ we send
            update_fn(&mut state);
            glib::Continue(true)
        });

        app_runtime
    }

    /// update application with update function
    pub fn update_state_with(&self, update_fn: impl FnOnce(&mut state::State) + 'static) {
        let _ = self.0.send(Box::new(update_fn));
    }
}

/// Sets up an app runtime on the given gtk application
pub fn new(app: gtk::Application) -> AppRuntime {
    let ui = ui::UI::new(app.clone());
    let runtime = AppRuntime::init(ui);

    app.connect_activate(clone!(@strong runtime => move |_| {
        runtime.update_state_with(|state| {
            state.init();
        });
    }));

    runtime
}

/*
/// Gets a hold of the app runtime into a 'static' reference
/// # Panics
/// - panics if called when there is no actual app runtime
pub fn get_app_runtime() -> &'static AppRuntime {
    unsafe {
        APP_RUNTIME
            .as_ref()
            .expect("Fatal: AppRuntime has not been initialized")
    }
}
*/