extern crate gtk;

pub struct UI {
    pub app: gtk::Application,
    pub win: gtk::ApplicationWindow,
    pub builder: gtk::Builder,
}

impl UI {
    pub fn new(app: gtk::Application) -> Self {
        let win = gtk::ApplicationWindowBuilder::new()
            .application(&app)
            .title("EdLib")
            .default_width(400)
            .default_height(200)
            .build();

        let builder = gtk::Builder::from_file("builder.ui");

        Self {
            app,
            win,
            builder,
        }
    }
}