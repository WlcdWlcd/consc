use std::io::{stdout, Write};

mod tests;

pub struct Bar{
    len: usize,
    filled_percent:f32,
    out:std::io::Stdout,

}

impl Bar {
    pub fn new(len: usize) -> Bar  {
        let out = stdout();

        Bar{
            len,
            filled_percent:0.0,
            out
        }
    }

    fn render(&mut self){ 
        print!("\r{}{}%", self.bar(self.filled_percent), self.filled_percent);
        self.out.flush().unwrap();
        println!();
    }

    fn bar(&self,percent:f32) -> String{
        assert!((0.0..=100.0).contains(&percent),"unexpected percentage count, percent should be between 0 and 100, got {}",percent);
        let repeat_count:usize = (percent * self.len as f32 / 100.0) as usize;

        let complete = "|".repeat(repeat_count);
        let remain = " ".repeat(self.len-repeat_count);
        let out = format!("[{}{}]",complete,remain,);
        out
    }

    pub fn set_percent(&mut self, percent:f32) {
        self.filled_percent = percent;
        self.render();

    }
}
