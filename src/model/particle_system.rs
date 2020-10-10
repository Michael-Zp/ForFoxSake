use cgmath;

pub struct ParticleSystemMetaData
{
    pub position: cgmath::Vector2<f32>,
    pub max_speed: f32,
    pub running_time: f32,
    pub max_running_time: f32,
}