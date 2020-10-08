use crate::model::fox_hole::{FoxHoleInGrid};
use crate::model::model_utils::{GridPosition};

pub struct Level
{
    start_pos: GridPosition,
    data: std::vec::Vec<std::vec::Vec<i32>>,
    fox_holes: std::vec::Vec<FoxHoleInGrid>,
}

impl Level
{
    pub fn get_start_pos(&self) -> &GridPosition
    {
        &self.start_pos
    }

    pub fn get_data(&self) -> &std::vec::Vec<std::vec::Vec<i32>>
    {
        &self.data
    }

    pub fn get_fox_holes(&self) -> &std::vec::Vec<FoxHoleInGrid>
    {
        &self.fox_holes
    }
}


//Empty shell to give Levels a namespace
pub struct Levels { }

impl Levels 
{
    pub fn level_0() -> Level
    {
        Level 
        {
            start_pos: GridPosition { column: 2, row: 2 },
            data: vec![ 
                vec![ 1, 1, 1, 1, 1, ],
                vec![ 1, 0, 1, 0, 1, ],
                vec![ 1, 1, 1, 1, 1, ],
                vec![ 1, 0, 1, 0, 1, ],
                vec![ 1, 1, 3, 1, 1, ],
            ],
            fox_holes: vec![ FoxHoleInGrid { entry: GridPosition { column: 1, row: 2 }, exit: GridPosition { column: 2, row: 1 } } ],
        }
    }
}
