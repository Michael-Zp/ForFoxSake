use lazy_static;

pub struct Input
{
    keys_already_pressed: std::collections::HashMap<i32, bool>,
    keys_pressed: std::collections::HashMap<String, i8>,
    keys_pressed_last_frame: std::collections::HashMap<String, i8>,
}

lazy_static::lazy_static!
{
    static ref KEY_MAP: std::collections::HashMap<i32, String> = 
    [
        (38, format!("MoveUp")),
        (39, format!("MoveRight")),
        (40, format!("MoveDown")),
        (37, format!("MoveLeft")),
        (87, format!("MoveUp")),
        (68, format!("MoveRight")),
        (83, format!("MoveDown")),
        (65, format!("MoveLeft")),
        (69, format!("Use")),
        (32, format!("Use")),
    ].iter().cloned().collect();
}


// macro_rules! key_pressed_function {
//     ($name:ident, $collection:ident, $adder:expr) => {
//         pub fn $name(&mut self, key_code: i32)
//         {
//             if let Some(input_string) = Input::key_to_input_string(key_code)
//             {
//                 if let Some(x) = self.$collection.get_mut(input_string)
//                 {
//                     *x = *x + $adder;
//                 }
//             }
//         }
//     }
// }

impl Input
{
    pub fn new() -> Input
    {

        let mut keys_pressed : std::collections::HashMap<String, i8> = std::collections::HashMap::new();        
        let mut keys_pressed_last_frame : std::collections::HashMap<String, i8> = std::collections::HashMap::new();

        for (_, v) in KEY_MAP.iter()
        {
            keys_pressed.insert(v.clone(), 0);    
            keys_pressed_last_frame.insert(v.clone(), 0);        
        }

        let keys_already_pressed : std::collections::HashMap<i32, bool> = std::collections::HashMap::new();

        Input {
            keys_already_pressed: keys_already_pressed,
            keys_pressed: keys_pressed,
            keys_pressed_last_frame: keys_pressed_last_frame,
        }
    }

    pub fn finalize(&mut self)
    {
        for key in self.keys_pressed.clone().keys()
        {
            *self.keys_pressed_last_frame.get_mut(key).unwrap() = self.keys_pressed.get(key).unwrap().clone();
        }
    }

    fn key_to_input_string(key_code: i32) -> Option<&'static String>
    {
        KEY_MAP.get(&key_code)
    }

    pub fn key_down(&mut self, key_code: i32)
    {
        if !self.keys_already_pressed.contains_key(&key_code)
        {
            self.keys_already_pressed.insert(key_code, false);
        }


        if !*self.keys_already_pressed.get(&key_code).unwrap()
        {
            if let Some(input_string) = Input::key_to_input_string(key_code)
            {
                if let Some(x) = self.keys_pressed.get_mut(input_string)
                {
                    *x = *x + 1;
                }
            }
            
            self.keys_already_pressed.insert(key_code, true);
        }
    }

    pub fn key_up(&mut self, key_code: i32)
    {
        if !self.keys_already_pressed.contains_key(&key_code)
        {
            self.keys_already_pressed.insert(key_code, false);
        }

        if *self.keys_already_pressed.get(&key_code).unwrap()
        {
            if let Some(input_string) = Input::key_to_input_string(key_code)
            {
                if let Some(x) = self.keys_pressed.get_mut(input_string)
                {
                    *x = *x - 1;
                }
            }

            self.keys_already_pressed.insert(key_code, false);
        }
    }

    pub fn is_input_pressed_this_frame(&self, input_string: &String) -> bool
    {
        *self.keys_pressed.get(input_string).unwrap() > 0
    }
    
    pub fn is_input_pressed_last_frame(&self, input_string: &String) -> bool
    {
        *self.keys_pressed_last_frame.get(input_string).unwrap() > 0
    }

}