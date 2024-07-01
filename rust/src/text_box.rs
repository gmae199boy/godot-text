use godot::{
    engine::{Image, Sprite2D, Texture2D},
    prelude::*,
};

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct TextBox {
    base: Base<Node2D>,
    sprite: Gd<Sprite2D>,
}

#[godot_api]
impl INode2D for TextBox {
    fn init(base: Base<Node2D>) -> Self {
        let texture = try_load::<Texture2D>("res://assets/text_box/text_box.png");
        let mut sprite = Sprite2D::new_alloc();

        sprite.set_texture(texture.unwrap());

        Self { base, sprite }
    }

    fn ready(&mut self) {
        godot_print!("1qwq");
        godot_print!("{:?}", self);
    }
}

impl TextBox {
    pub fn a() -> String {
        "qqq".into()
    }
}
