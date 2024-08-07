use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameManager {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for GameManager {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("as1d");
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("asd2");
    }
}
