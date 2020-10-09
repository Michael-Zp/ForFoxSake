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

pub struct Model
{
    player_pos: cgmath::Vector2<f32>,
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
            player_pos: cgmath::Vector2 { x: 0.0, y: 0.0 },
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
            add_sprite!(0.2, 0.2, hole.entry, 1, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
            add_sprite!(0.2, 0.2, hole.exit, 1, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);

            if hole.used
            {
                add_sprite!(0.15, 0.15, hole.entry, 2, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
                add_sprite!(0.15, 0.15, hole.exit, 2, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
            }
        }

        for wolf in self.wolves.iter()
        {
            add_sprite!(0.2, 0.2, wolf.pos, 3, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);
        }

        add_sprite!(0.2, 0.2, self.player_pos, 0, sprite_sizes, sprite_positions, sprite_tile_map_indices, current_index);

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

        self.player_pos = model_utils::grid_to_position(level.get_start_pos(), width, height);

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

        if input.is_input_down("MoveLeft") || input.is_input_pressed("MoveLeft")
        {
            self.player_pos.x -= SPEED * delta_time;
        }
        
        if input.is_input_down("MoveRight") || input.is_input_pressed("MoveRight")
        {
            self.player_pos.x += SPEED * delta_time;
        }

        if input.is_input_down("MoveDown") || input.is_input_pressed("MoveDown")
        {
            self.player_pos.y -= SPEED * delta_time;
        }

        if input.is_input_down("MoveUp") || input.is_input_pressed("MoveUp")
        {
            self.player_pos.y += SPEED * delta_time;
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

                if (self.player_pos - hole.entry).magnitude() < 0.1
                {
                    self.player_pos = hole.exit;
                    hole.used = true;
                }
            }
        }
    }

    fn check_wolves(&mut self)
    {
        for wolf in &mut self.wolves
        {
            if (self.player_pos - wolf.pos).magnitude() < 0.15
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