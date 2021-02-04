extern crate glib;
extern crate gio;
use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;

use crate::ui;

/// Represents the global app runtime
static mut APP_RUNTIME: Option<AppRuntime> = None;

/// An AppRuntime allows interacting with the gtk main context
/// via a channel, where we functions that work over an AppOp
#[derive(Clone)]
pub struct AppRuntime(glib::Sender<Box<dyn FnOnce(&mut AppOp)>>);

/// Represents the applciation state after a single operation
pub struct AppOp {
    pub runtime: AppRuntime,
    pub ui: ui::UI,
}

impl AppRuntime {
    /// Create the application runtime from our UI
    fn init(ui: ui::UI) -> Self {
        println!("initializing app runtime");
        // channel for main context communication (default priority)
        let (tx, rx) = glib::MainContext::channel(Default::default());
        // create the runtime from our channel end of the main context
        let runtime = Self(tx);
        // init application state
        let mut state = AppOp::new(ui, runtime.clone());

        unsafe {
            // we set the global state to the new runtime
            APP_RUNTIME = Some(runtime.clone());
        }

        // on the receiveing end of the main context channel, we
        // receive functions, which we call on the state of the application
        // to update the state
        rx.attach(None, move |update_state| {
            // here the main context _owns_ the applciatio state,
            // that we get to mutate via the _functions_ we send
            println!("updating app state");
            update_state(&mut state);
            glib::Continue(true)
        });

        runtime
    }

    /// update application with update function
    fn update_state_width(&self, update_fn: impl FnOnce(&mut AppOp) + 'static) {
        let _ = self.0.send(Box::new(update_fn));
    }
}

impl AppOp {
    pub fn new(ui: ui::UI, runtime: AppRuntime) -> Self {
        Self {
            runtime,
            ui,
        }
    }
}

/// Sets up an app runtime from the given gtk application
fn new(app: gtk::Application) -> AppRuntime {
    println!("creating app");
    // init the ui
    let ui = ui::UI::new(app.clone());
    // init the runtime
    let runtime = AppRuntime::init(ui);

    runtime.update_state_width(move |_| {
        // update application state
        // . . .
    });

    runtime
}

/// Gets a hold of the app runtime into a 'static' reference
/// # Panics
/// - panics if called when there is no actual app runtime
pub fn get_app_runtime() -> &'static AppRuntime {
    println!("getting app runtime");
    unsafe {
        APP_RUNTIME
            .as_ref()
            .expect("Fatal: AppRuntime has not been initialized")
    }
}

/// Application initialization code
/// (callbacks, etc)
pub fn on_startup(app: &gtk::Application) {
    println!("app on_startup");
    // create applciation
    let runtime = new(app.clone());

    // app activate callback
    app.connect_activate(clone!(@strong runtime => move |_| {
        runtime.update_state_width(|state| {
            on_activate(state);
        });
    }));
}

fn on_activate(appop: &mut AppOp) {
    println!("app on_activate");
    appop.ui.win.show();
    appop.ui.win.present();
}