pub trait ChangeBarConfig{
    fn set_lengh(&mut self,new_lengh:usize);
    fn set_filled_char(&mut self,new_filled_char:char);
    fn set_empty_char(&mut self,new_empty_char:char);
    fn set_left_bracket(&mut self,new_left_bracket:char);
    fn set_right_bracket(&mut self,new_right_bracket:char);

}

pub struct BarConfig{
    pub lengh: usize,
    pub filled_char: char,
    pub empty_char: char,
    pub left_bracket_char: char,
    pub right_bracket_char: char,
}

impl BarConfig{
    pub fn new(lengh: usize, filled_char: char, empty_char:char, left_bracket_char: char,right_bracket_char: char)->BarConfig{

        BarConfig{
            lengh,
            filled_char,
            empty_char,
            left_bracket_char,
            right_bracket_char,

        }
    }

    pub fn default_config() -> BarConfig{
        BarConfig{
            lengh: 20,
            filled_char: '|',
            empty_char:' ',
            left_bracket_char: '[',
            right_bracket_char: ']',
        }
    }

}
impl ChangeBarConfig for BarConfig{
    fn set_lengh(&mut self,new_lengh:usize){
        self.lengh = new_lengh;
    }
    
    fn set_filled_char(&mut self,new_filled_char:char) {
        self.filled_char = new_filled_char;
    }
    
    fn set_empty_char(&mut self,new_empty_char:char) {
        self.empty_char = new_empty_char;
    }
    
    fn set_left_bracket(&mut self,new_left_bracket:char) {
        self.left_bracket_char = new_left_bracket;
    }
    
    fn set_right_bracket(&mut self,new_right_bracket:char) {
        self.right_bracket_char = new_right_bracket;
    }
        

        
}

