pub struct Level
{
    data: [i32; 25],
    width: u32,
    height: u32,
}

impl Level
{
    pub fn get_data(&self) -> [i32;25]
    {
        self.data
    }

    pub fn get_width(&self) -> u32
    {
        self.width
    }

    pub fn get_height(&self) -> u32
    {
        self.height
    }
}


pub const LEVEL0: Level = Level {
    data: [ 1, 1, 1, 1, 1,  1, 0, 1, 0, 1,  1, 1, 1, 1, 1,  1, 0, 1, 0, 1,  1, 1, 3, 1, 1 ],
    width: 5,
    height: 5,
};
