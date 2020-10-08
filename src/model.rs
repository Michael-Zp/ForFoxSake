use crate::for_fox_sake::read_only_input::ReadOnlyInput;
use crate::view_models::{LevelViewModel, SpritesViewModel};
use cgmath;
use cgmath::InnerSpace;
pub mod level;

mod fox_hole;
use fox_hole::{FoxHoleWithPosition};
mod model_utils;

pub struct Model
{
    player_pos: cgmath::Vector2<f32>,
    fox_holes: std::vec::Vec<FoxHoleWithPosition>,
}

impl<'a> Model
{
    pub fn new() -> Result<Model, String>
    {
        Ok(Model{ 
            player_pos: cgmath::Vector2 { x: 0.0, y: 0.0 },
            fox_holes: std::vec::Vec::new(),
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
            sprite_sizes[current_index] = cgmath::Vector2{ x: 0.2, y: 0.2 };
            sprite_positions[current_index] = hole.entry;
            sprite_tile_map_indices[current_index] = 1;
            current_index = current_index + 1;
            
            sprite_sizes[current_index] = cgmath::Vector2{ x: 0.2, y: 0.2 };
            sprite_positions[current_index] = hole.exit;
            sprite_tile_map_indices[current_index] = 1;
            current_index = current_index + 1;

            if hole.used
            {
                sprite_sizes[current_index] = cgmath::Vector2{ x: 0.15, y: 0.15 };
                sprite_positions[current_index] = hole.entry;
                sprite_tile_map_indices[current_index] = 2;
                current_index = current_index + 1;
                
                sprite_sizes[current_index] = cgmath::Vector2{ x: 0.15, y: 0.15 };
                sprite_positions[current_index] = hole.exit;
                sprite_tile_map_indices[current_index] = 2;
                current_index = current_index + 1;
            }
        }

        sprite_sizes[current_index] = cgmath::Vector2{ x: 0.2, y: 0.2 };
        sprite_positions[current_index] = self.player_pos;
        sprite_tile_map_indices[current_index] = 0;
        current_index = current_index + 1;

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
            self.fox_holes.push(FoxHoleWithPosition::from(hole, width, height));
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

    pub fn update(&mut self, input: ReadOnlyInput, delta_time: f32)
    {
        self.check_fox_hole_usage(&input);
        self.move_player(&input, delta_time);
    }
}