use cgmath;
use crate::model::model_utils::SpriteAnimation;

pub struct Fox
{
    pub pos: cgmath::Vector2<f32>,
    sprite_animations: std::collections::HashMap<&'static str, SpriteAnimation>,
    current_animation: &'static str,
    animation_time: f32,
}

impl Fox
{
    pub const MOVE_LEFT: &'static str = "MoveLeft";
    pub const MOVE_RIGHT: &'static str = "MoveRight";
    pub const MOVE_DOWN: &'static str = "MoveDown";
    pub const MOVE_UP: &'static str = "MoveUp";

    pub fn new(pos: cgmath::Vector2<f32>) -> Fox
    {
        let mut animations = std::collections::HashMap::new();
        animations.insert(Fox::MOVE_LEFT, SpriteAnimation{ from_index: 3, to_index: 6, timeout: 0.1 });
        animations.insert(Fox::MOVE_RIGHT, SpriteAnimation{ from_index: 6, to_index: 9, timeout: 0.1 });
        animations.insert(Fox::MOVE_DOWN, SpriteAnimation{ from_index: 0, to_index: 3, timeout: 0.1 });
        animations.insert(Fox::MOVE_UP, SpriteAnimation{ from_index: 9, to_index: 12, timeout: 0.1 });

        Fox { pos: pos, sprite_animations: animations, current_animation: Fox::MOVE_LEFT, animation_time: 0.0 }
    }

    pub fn update_animation(&mut self, animation_name: &'static str, delta_time: f32)
    {
        if animation_name == self.current_animation
        {
            self.animation_time = self.animation_time + delta_time;
            let animation = self.sprite_animations.get(&self.current_animation).unwrap();
            self.animation_time = self.animation_time % ((animation.to_index - animation.from_index) as f32 * animation.timeout);
        }
        else
        {
            self.animation_time = 0.0;
            self.current_animation = animation_name;
        }
    }

    pub fn get_sprite(&self) -> i32
    {
        let animation = self.sprite_animations.get(&self.current_animation).unwrap();
        animation.from_index + (self.animation_time / animation.timeout).floor() as i32
    }
}