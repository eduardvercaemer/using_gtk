use gio::prelude::*;
mod ui;
mod app;
mod state;
mod actions;

fn main() {
    // create gtk application
    let app = gtk::ApplicationBuilder::new()
        .application_id("com.eduard.demo")
        .build();
    
    // run startup code on the app
    app.connect_startup(|app| {
        app::new(app.clone());
    });

    // run
    app.run(&[]);

    // _that's it_
}