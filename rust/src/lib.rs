use game_manager::GameManager;
use godot::{engine::Label, prelude::*};

mod card;
mod game_manager;
mod text_box;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct TextGame {
    base: Base<Node2D>,
    game_manager: Gd<game_manager::GameManager>,
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
        let game_manager = GameManager::new_alloc();
        // let game_manager = GameManager::new_alloc();

        // let mut label = Label::new_alloc();
        // let text = format!("Item: {}", x).to_godot();
        // label.set_text(text);
        // self.base.to_gd().add_child(label.upcast());
        // base.to_gd().add_child(game_manager.upcast());

        let instance = Self { base, game_manager };
        instance.base.to_gd().add_child(game_manager.upcast());

        instance
    }

    fn ready(&mut self) {
        // let child = game_manager::GameManager::new_alloc();
        // self.base.to_gd().add_child(child.upcast());

        // self.base_mut().add_child(
        //     *self
        //         .game_manager
        //         .get_node_as::<game_manager::GameManager>("GameManager"),
        // );
        godot_print!("TextGame ready");
    }
}
