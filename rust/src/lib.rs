use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct TextGame {
    base: Base<Node2D>,
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
        godot_print!("Hello, world!");
        godot_print!("end!");

        Self { base }
    }
}
