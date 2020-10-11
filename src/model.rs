use crate::for_fox_sake::read_only_input::ReadOnlyInput;
use crate::view_models::{LevelViewModel, SpritesViewModel, ParticlesViewModel, PostProcessViewModel, PostProcessEffect, PostProcessEffects};
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
mod particle_system;
use particle_system::ParticleSystemMetaData;
mod post_process_effect_meta_data;
use post_process_effect_meta_data::PostProcessEffectMetaData;

pub struct Model
{
    player: Fox,
    fox_holes: std::vec::Vec<FoxHole<cgmath::Vector2<f32>>>,
    wolves: std::vec::Vec<Wolf<cgmath::Vector2<f32>>>,
    particle_systems: std::vec::Vec<ParticleSystemMetaData>,
    post_process_effects: std::vec::Vec<PostProcessEffectMetaData>,
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
macro_rules! add_particle_effect {
    ($pos:expr, $max_speed:expr, $time_passed:expr, $max_running_time:expr, $psp:ident, $psms:ident, $pstp:ident, $psmrt:ident, $ci:ident) => {
        $psp[$ci] = $pos;
        $psms[$ci] = $max_speed;
        $pstp[$ci] = $time_passed;
        $psmrt[$ci] = $max_running_time;
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
            particle_systems: std::vec::Vec::new(),
            post_process_effects: std::vec::Vec::new(),
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
            sizes: sprite_sizes,
            positions: sprite_positions,
            tile_map_indices: sprite_tile_map_indices,
            count: current_index as i32,
        }
    }

    
    pub fn to_particles_view_model(&self) -> ParticlesViewModel
    {
        let mut positions: [cgmath::Vector2<f32>;10] = [cgmath::Vector2{ x: 0.0, y: 0.0 };10];
        let mut max_speeds: [f32;10] = [0.0;10];
        let mut running_times: [f32;10] = [0.0;10];
        let mut max_running_times: [f32;10] = [0.0;10];

        let mut current_index = 0;

        for system in self.particle_systems.iter()
        {
            add_particle_effect!(system.position, system.max_speed, system.running_time, system.max_running_time, positions, max_speeds, running_times, max_running_times, current_index);
        }
        

        ParticlesViewModel {
            positions: positions,
            max_speeds: max_speeds,
            running_times: running_times,
            max_running_times: max_running_times,
            count: current_index as i32,
        }
    }

    pub fn to_post_process_view_model(&self) -> PostProcessViewModel
    {
        let mut post_process_effects: std::vec::Vec<PostProcessEffect> = std::vec::Vec::new();

        for effect in self.post_process_effects.iter()
        {
            post_process_effects.push( PostProcessEffect {
                name: effect.name,
                running_time: effect.running_time,
                max_running_time: effect.max_running_time,
            });
        }

        PostProcessViewModel { 
            effects: post_process_effects
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

    fn spawn_fox_hole_entry_particle_system(&mut self, start_position: cgmath::Vector2<f32>)
    {
        self.particle_systems.push(ParticleSystemMetaData{
            position: start_position,
            max_speed: 0.1,
            running_time: 0.0,
            max_running_time: 3.0,
        });
    }

    fn spawn_fox_hole_entry_post_process_effect(&mut self)
    {
        self.post_process_effects.push(PostProcessEffectMetaData{ 
            name: PostProcessEffects::VIGNETTE,
            running_time: 0.0,
            max_running_time: 1.5,
        });
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
        let mut used_entry_position: Option<cgmath::Vector2<f32>> = None;
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
                    used_entry_position = Some(hole.entry);
                }
            }
        }

        if let Some(pos) = used_entry_position
        {
            self.spawn_fox_hole_entry_particle_system(pos);
            self.spawn_fox_hole_entry_post_process_effect();
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

    fn update_particle_systems(&mut self, delta_time: f32)
    {
        for system in self.particle_systems.iter_mut()
        {
            system.running_time = system.running_time + delta_time;
        }

        self.particle_systems.retain(|x| x.running_time < x.max_running_time);
    }

    fn update_post_process_effects(&mut self, delta_time: f32)
    {
        for effect in self.post_process_effects.iter_mut()
        {
            effect.running_time = effect.running_time + delta_time;
        }

        self.post_process_effects.retain(|x| x.running_time < x.max_running_time);
    }

    pub fn update(&mut self, input: ReadOnlyInput, delta_time: f32)
    {
        if self.alive
        {
            self.check_fox_hole_usage(&input);
            self.move_player(&input, delta_time);
            self.check_wolves();
            self.update_particle_systems(delta_time);
            self.update_post_process_effects(delta_time);
        }
    }
}