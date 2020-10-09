use crate::for_fox_sake::Input;

pub struct ReadOnlyInput<'a>
{
    input: &'a Input,
}

impl<'a> ReadOnlyInput<'a>
{
    pub fn new(input: &'a Input) -> ReadOnlyInput
    {
        ReadOnlyInput {
            input: input,
        }
    }

    pub fn is_input_down(&self, input_string: &str) -> bool
    {
        self.input.is_input_pressed_this_frame(&format!("{}", input_string)) && !self.input.is_input_pressed_last_frame(&format!("{}", input_string))
    }

    pub fn is_input_pressed(&self, input_string: &str) -> bool
    {
        self.input.is_input_pressed_this_frame(&format!("{}", input_string)) && self.input.is_input_pressed_last_frame(&format!("{}", input_string))
    }

    pub fn is_input_up(&self, input_string: &str) -> bool
    {
        !self.input.is_input_pressed_this_frame(&format!("{}", input_string)) && self.input.is_input_pressed_last_frame(&format!("{}", input_string))
    }
}