use godot::{obj::NewGd, prelude::*};

mod card;
mod game_manager;
mod text_box;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct TextGame {
    base: Base<Node2D>,

    game_manager: Gd<game_manager::GameManager>,
    text_box: Gd<text_box::TextBox>,
}

#[gdextension]
unsafe impl ExtensionLibrary for TextGame {
    fn on_level_init(_level: InitLevel) {
        println!("[Rust]      Init level {:?}", _level);
    }

    fn on_level_deinit(_level: InitLevel) {
        println!("[Rust]      Deinit level {:?}", _level);
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[godot_api]
impl INode2D for TextGame {
    fn init(base: Base<Node2D>) -> Self {
        let game_manager = game_manager::GameManager::new_alloc();
        let text_box = text_box::TextBox::new_alloc();

        Self {
            base,
            game_manager,
            text_box,
        }
    }

    fn ready(&mut self) {}
}
