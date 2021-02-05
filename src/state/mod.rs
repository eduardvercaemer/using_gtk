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

/// General application state
pub struct State {
    pub app_runtime: app::AppRuntime,
    pub ui: ui::UI,

    pub game: GameState,
}

/// More specific state (for some section of the app)
pub struct GameState {
    pub health: usize,
}

impl GameState {
    /// init the game state
    pub fn new() -> Self {
        Self {
            health: 50,
        }
    }

}

impl State {
    /// create the initial state
    pub fn new(ui: ui::UI, app_runtime: app::AppRuntime) -> Self {
        let state = Self {
            app_runtime,
            ui,

            game: GameState::new(),
        };

        state
    }

    /// for the app activate callback
    pub fn init(&mut self) {
        actions::global(self);
        self.main_menu();
        self.ui.win.show();
        self.ui.win.present();
    }
}

/// State functions for loading the different subsections
impl State {
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

    /// load the about subsection
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

    /// load the game subsection
    pub fn game(&mut self) {
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
            .get_object("game")
            .expect("objcet: game");
        self
            .ui
            .win
            .add(&root);

        // set up game actions
        actions::game(self);

        // init game state
        self.game = GameState::new();
        self.game_display_health();
    }
}

/// State functions for _game_ subsection
impl State {
    /// for action app.game.hit
    pub fn game_hit(&mut self) {
        println!("dbg hit");
        self
            .game
            .health -= 2;
        self.game_display_health();
    }

    /// for action app.game.heal
    pub fn game_heal(&mut self) {
        println!("dbg heal");
        self
            .game
            .health += 5;
        self.game_display_health();
    }

    /// displays the current health in the label
    fn game_display_health(&mut self) {
        let label: gtk::Label = self
            .ui
            .builder
            .get_object("game_health_label")
            .expect("object: game_health_label");

        label.set_label(format!("Health: {}", self.game.health).as_ref());
    }
}