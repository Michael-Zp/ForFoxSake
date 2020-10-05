mod stuff;

pub struct View 
{
    testVal: stuff::TestViewVal,
}

impl View
{
    
    pub fn new() -> Result<View, String>
    {
        Ok(View{
            testVal: stuff::TestViewVal{ }
        })
    }

    pub fn test_interface(&self)
    {
        stuff::test_impl();
    }
}