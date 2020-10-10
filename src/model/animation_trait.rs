use crate::model::model_utils::SpriteAnimationMetaData;

pub trait SpriteAnimation
{

    fn get_sprite_animations(&self) -> &std::collections::HashMap<&'static str, SpriteAnimationMetaData>;
    fn get_current_animation(&self) -> &'static str;
    fn set_current_animation(&mut self, new_current_animation: &'static str);
    fn get_animation_time(&self) -> f32;
    fn set_animation_time(&mut self, new_animation_time: f32);



    fn update_animation(&mut self, animation_name: &'static str, delta_time: f32)
    {
        if animation_name == self.get_current_animation()
        {
            self.set_animation_time(self.get_animation_time() + delta_time);
            let animation = self.get_sprite_animations().get(&self.get_current_animation()).unwrap();
            self.set_animation_time(self.get_animation_time() % ((animation.to_index - animation.from_index) as f32 * animation.timeout));
        }
        else
        {
            self.set_animation_time(0.0);
            self.set_current_animation(animation_name);
        }
    }

    fn get_sprite(&self) -> i32
    {
        let animation = self.get_sprite_animations().get(&self.get_current_animation()).unwrap();
        animation.from_index + (self.get_animation_time() / animation.timeout).floor() as i32
    }
}