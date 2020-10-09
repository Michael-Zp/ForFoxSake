use crate::model::fox_hole::FoxHole;
use crate::model::model_utils::GridPosition;
use crate::model::wolf::Wolf;

pub struct Level
{
    start_pos: GridPosition,
    data: std::vec::Vec<std::vec::Vec<i32>>,
    fox_holes: std::vec::Vec<FoxHole<GridPosition>>,
    wolves: std::vec::Vec<Wolf<GridPosition>>,
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

    pub fn get_fox_holes(&self) -> &std::vec::Vec<FoxHole<GridPosition>>
    {
        &self.fox_holes
    }
    
    pub fn get_wolves(&self) -> &std::vec::Vec<Wolf<GridPosition>>
    {
        &self.wolves
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
                vec![ 1, 1, 1, 1, 1, ],
            ],
            fox_holes: vec![ FoxHole::new(GridPosition { column: 1, row: 2 }, GridPosition { column: 2, row: 1 }, None), ],
            wolves: vec![ Wolf::new(GridPosition { column: 2, row: 4 }) ],
        }
    }
}
