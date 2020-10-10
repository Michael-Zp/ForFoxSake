use crate::for_fox_sake::read_only_input::ReadOnlyInput;
use crate::view_models::{LevelViewModel, SpritesViewModel};
use cgmath;
use cgmath::InnerSpace;
pub mod level;

mod fox_hole;
use fox_hole::{FoxHole};
mod model_utils;
mod wolf;
use wolf::Wolf;
mod fox;
use fox::Fox;
mod animation_trait;
use animation_trait::SpriteAnimation;

pub struct Model
{
    player: Fox,
    fox_holes: std::vec::Vec<FoxHole<cgmath::Vector2<f32>>>,
    wolves: std::vec::Vec<Wolf<cgmath::Vector2<f32>>>,
    alive: bool,
}

//Yes this is clunky with the identifiers at the back, but local variables are not supported by macros anymore
//Could move the identifiers into model, but they don´t really fit there either...
macro_rules! add_sprite {
    ($x:expr, $y:expr, $pos:expr, $sprite:expr, $ss:ident, $sp:ident, $stmi:ident, $ci:ident) => {
        $ss[$ci] = cgmath::Vector2{ x: $x, y: $y };
        $sp[$ci] = $pos;
        $stmi[$ci] = $sprite;
        $ci = $ci + 1;
    };
}

impl Model
{
    pub fn new() -> Result<Model, String>
    {
        Ok(Model{ 
            player: Fox::new(cgmath::Vector2 { x: 0.0, y: 0.0 }),
            fox_holes: std::vec::Vec::new(),
            wolves: std::vec::Vec::new(),
            alive: true,
        })
    }


    pub fn to_sprites_view_model(&self) -> SpritesViewModel
    {
        let mut sprite_sizes: [cgmath::Vector2<f32>;10] = [cgmath::Vector2{ x: 0.0, y: 0.0 };10];
        let mut sprite_positions: [cgmath::Vector2<f32>;10] = [cgmath::Vector2{ x: 0.0, y: 0.0 };10];
        let mut sprite_tile_map_indices: [i32;10] = [0;10];

        let mut current_index = 0;

        for hole in self.fox_holes.iter()
        {
            add_sprite!(0.2, 0.2, hole.entry, hole.entry_sprite, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
            add_sprite!(0.2, 0.2, hole.exit, hole.exit_sprite, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);

            if hole.used
            {
                add_sprite!(0.15, 0.15, hole.entry, hole.closed_sprite, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
                add_sprite!(0.15, 0.15, hole.exit, hole.closed_sprite, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
            }
        }

        for wolf in self.wolves.iter()
        {
            add_sprite!(0.2, 0.2, wolf.pos, wolf.sprite, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
        }

        add_sprite!(0.2, 0.2, self.player.pos, self.player.get_sprite(), sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);

        SpritesViewModel {
            sprite_sizes: sprite_sizes,
            sprite_positions: sprite_positions,
            sprite_tile_map_indices: sprite_tile_map_indices,
            sprite_count: current_index as i32,
        }
    }

    pub fn load_level(&mut self, level_code: u8) -> Result<LevelViewModel, String>
    {
        let level = match level_code
        {
            0 => Ok(level::Levels::level_0()),
            _ => Err(format!("Level not found")),
        }?;
        
        let check_width = level.get_data()[0].len();
        let width = check_width as f32;
        let height = level.get_data().len() as f32;

        let mut flat_map: std::vec::Vec<i32> = std::vec::Vec::new();
        for row in level.get_data().iter()
        {
            assert_eq!(check_width, row.len());
            for tile in row.iter()
            {
                flat_map.push(tile.clone());
            }
        }

        self.player.pos = model_utils::grid_to_position(level.get_start_pos(), width, height);

        for hole in level.get_fox_holes().iter()
        {
            self.fox_holes.push(FoxHole::from(hole, width, height));
        }

        for wolf in level.get_wolves().iter()
        {
            self.wolves.push(Wolf::from(wolf, width, height));
        }

        Ok(LevelViewModel {
            data: flat_map,
            width: width,
            height: height,
        })
    }

    fn move_player(&mut self, input: &ReadOnlyInput, delta_time: f32)
    {
        const SPEED: f32 = 0.2;

        let move_left = input.is_input_down("MoveLeft") || input.is_input_pressed("MoveLeft");
        let move_right = input.is_input_down("MoveRight") || input.is_input_pressed("MoveRight");
        let move_down = input.is_input_down("MoveDown") || input.is_input_pressed("MoveDown");
        let move_up = input.is_input_down("MoveUp") || input.is_input_pressed("MoveUp");

        if move_left && !move_right
        {
            self.player.pos.x -= SPEED * delta_time;
            self.player.update_animation(Fox::MOVE_LEFT, delta_time);
        } 
        else if move_right && !move_left
        {
            self.player.pos.x += SPEED * delta_time;
            self.player.update_animation(Fox::MOVE_RIGHT, delta_time);
        } 
        else if move_down && !move_up
        {
            self.player.pos.y -= SPEED * delta_time;
            self.player.update_animation(Fox::MOVE_DOWN, delta_time);
        }
        else if move_up && !move_down
        {
            self.player.pos.y += SPEED * delta_time;
            self.player.update_animation(Fox::MOVE_UP, delta_time);
        }
    }

    fn check_fox_hole_usage(&mut self, input: &ReadOnlyInput)
    {
        if input.is_input_down("Use") || input.is_input_pressed("Use")
        {
            for hole in &mut self.fox_holes
            {
                if hole.used
                {
                    continue;
                }

                if (self.player.pos - hole.entry).magnitude() < 0.1
                {
                    self.player.pos = hole.exit;
                    hole.used = true;
                }
            }
        }
    }

    fn check_wolves(&mut self)
    {
        for wolf in &mut self.wolves
        {
            if (self.player.pos - wolf.pos).magnitude() < 0.15
            {
                self.alive = false;
            }
        }
    }

    pub fn update(&mut self, input: ReadOnlyInput, delta_time: f32)
    {
        if self.alive
        {
            self.check_fox_hole_usage(&input);
            self.move_player(&input, delta_time);
            self.check_wolves();
        }
    }
}