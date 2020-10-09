use cgmath;

pub struct Fox
{
    pub pos: cgmath::Vector2<f32>,
    pub sprite: i32,
}

impl Fox
{
    pub fn new(pos: cgmath::Vector2<f32>) -> Fox
    {
        Fox { pos: pos, sprite: 0 }
    }

    pub fn set_move_left_sprite(&mut self)
    {
        self.sprite = 3;
    }
    
    pub fn set_move_right_sprite(&mut self)
    {
        self.sprite = 6;
    }
    
    pub fn set_move_down_sprite(&mut self)
    {
        self.sprite = 0;
    }
    
    pub fn set_move_up_sprite(&mut self)
    {
        self.sprite = 9;
    }
}