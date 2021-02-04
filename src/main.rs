extern crate gtk;
extern crate gio;
use gio::prelude::*;
mod ui;
mod app;

fn main() {
    let app = gtk::ApplicationBuilder::new()
        .application_id("com.eduard.demo")
        .build();
    
    app.connect_startup(|app| {
        app::on_startup(app);
    });

    app.run(&[]);
}