extern crate gtk;
use gtk::prelude::*;

pub struct UI {
    pub app: gtk::Application,
    pub win: gtk::ApplicationWindow,
}

impl UI {
    pub fn new(app: gtk::Application) -> Self {
        println!("creating app window");
        let win = gtk::ApplicationWindowBuilder::new()
            .application(&app)
            .title("GTK-Demo")
            .default_width(400)
            .default_height(200)
            .build();
        let grid = gtk::GridBuilder::new().build();
        grid.show();
        win.add(&grid);
        let button = gtk::ButtonBuilder::new()
            .label("hit")
            .visible(true)
            .build();
        grid.attach(&button, 0, 0, 1, 1);

        Self {
            app,
            win,
        }
    }
}