use cgmath;
use crate::model::model_utils::SpriteAnimationMetaData;
use crate::model::animation_trait::SpriteAnimation;

pub struct Fox
{
    pub pos: cgmath::Vector2<f32>,
    sprite_animations: std::collections::HashMap<&'static str, SpriteAnimationMetaData>,
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
        animations.insert(Fox::MOVE_LEFT, SpriteAnimationMetaData{ from_index: 3, to_index: 6, timeout: 0.1 });
        animations.insert(Fox::MOVE_RIGHT, SpriteAnimationMetaData{ from_index: 6, to_index: 9, timeout: 0.1 });
        animations.insert(Fox::MOVE_DOWN, SpriteAnimationMetaData{ from_index: 0, to_index: 3, timeout: 0.1 });
        animations.insert(Fox::MOVE_UP, SpriteAnimationMetaData{ from_index: 9, to_index: 12, timeout: 0.1 });

        Fox { pos: pos, sprite_animations: animations, current_animation: Fox::MOVE_LEFT, animation_time: 0.0 }
    }
}


impl SpriteAnimation for Fox
{
    fn get_sprite_animations(&self) -> &std::collections::HashMap<&'static str, SpriteAnimationMetaData>
    {
        &self.sprite_animations
    }

    fn get_current_animation(&self) -> &'static str
    {
        self.current_animation
    }

    fn set_current_animation(&mut self, new_current_animation: &'static str)
    {
        self.current_animation = new_current_animation;
    }

    fn get_animation_time(&self) -> f32
    {
        self.animation_time
    }

    fn set_animation_time(&mut self, new_animation_time: f32)
    {
        self.animation_time = new_animation_time;
    }
}