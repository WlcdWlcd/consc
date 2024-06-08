mod tests;
pub mod bar_config;

use std::io::{stdout, Write};
use crate::bar_config::{BarConfig,ChangeBarConfig};




pub struct Bar{
    config: BarConfig,
    filled_percent:f32,
    out:std::io::Stdout,

}

impl Bar {
    pub fn default() -> Bar  {
        let out = stdout();
        let config = BarConfig::default_config();

        Bar{
            config,
            filled_percent:0.0,
            out
        }
    }

    pub fn from_config(config: BarConfig) -> Bar{
        let out = stdout(); 

        Bar{
            config,
            filled_percent:0.0,
            out
        }
    }

    fn render(&mut self){ 
        print!("\r{}{}%", self.bar(self.filled_percent), self.filled_percent);
        self.out.flush().unwrap();
        //println!();
    }

    fn bar(&self,percent:f32) -> String{
        assert!((0.0..=100.0).contains(&percent),"unexpected percentage count, percent should be between 0 and 100, got {}",percent);
        let repeat_count:usize = (percent * self.config.lengh as f32 / 100.0) as usize;
        // let bracket = self.config.bracket_char.to_owned();
        let complete = self.config.filled_char.
            to_string().repeat(repeat_count);
        let remain = self.config.empty_char.
            to_string().repeat(self.config.lengh-repeat_count);

        let left_bracket = self.config.left_bracket_char;
        let right_bracket = self.config.right_bracket_char;

        let out = format!("{left_bracket}{complete}{remain}{right_bracket}");
        out
    }

    pub fn set_percent(&mut self, percent:f32) {
        self.filled_percent = percent;
        self.render();

    }
}
impl ChangeBarConfig for Bar{
    fn set_lengh(&mut self,new_lengh:usize){
        self.config.lengh = new_lengh;
    }
    
    fn set_filled_char(&mut self,new_filled_char:char) {
        self.config.filled_char = new_filled_char;
    }
    
    fn set_empty_char(&mut self,new_empty_char:char) {
        self.config.empty_char = new_empty_char;
    }
    
    fn set_left_bracket(&mut self,new_left_bracket:char) {
        self.config.left_bracket_char = new_left_bracket;
    }
    
    fn set_right_bracket(&mut self,new_right_bracket:char) {
        self.config.right_bracket_char = new_right_bracket;
    }
       
}









// todo: create struct Bracket
// pub struct Bracket{
    
// }