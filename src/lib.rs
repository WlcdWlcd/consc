use std::io::{stdout, Write};

mod tests;

 
pub struct Bar{
    len: usize,
    filled_percent:f32,
    out:std::io::Stdout,

}

impl Bar {
    pub fn new(len: usize) -> Bar  {
        let mut out = stdout();

        Bar{
            len: len,
            filled_percent:0.0,
            out
        }
    }

    fn render(&mut self){ // TODO add value of completed
        print!("\r{}{}%", self.bar(self.filled_percent), self.filled_percent);
        self.out.flush().unwrap();
        println!();
    }

    fn bar(&self,percent:f32) -> String{
        assert!(percent<=100.0 && percent>=0.0,"unexpected percentage count, percent should be between 0 and 100, got {}",percent);
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


#[cfg(test)]
pub mod lib_tests{
    use super::*;
    #[test]
    fn test_generate_out_len_100(){
            let bar = Bar::new(100);
            let one_per_bar =  "[|                                                                                                   ]";
            let zero_per_bar = "[                                                                                                    ]";

            assert_eq!(zero_per_bar,bar.bar(0.0));
            assert_eq!(zero_per_bar,bar.bar(0.5));
            assert_eq!(zero_per_bar,bar.bar(0.9));
            assert_eq!(zero_per_bar,bar.bar(0.999));
            assert_eq!(zero_per_bar,bar.bar(0.999999));
            assert_eq!(one_per_bar,bar.bar(1.0));
            assert_eq!(one_per_bar,bar.bar(1.1));
            assert_eq!(one_per_bar,bar.bar(1.2));
            assert_eq!(one_per_bar,bar.bar(1.3));
            assert_eq!(one_per_bar,bar.bar(1.4));
            assert_eq!(one_per_bar,bar.bar(1.5));
            assert_eq!(one_per_bar,bar.bar(1.6));
            assert_eq!(one_per_bar,bar.bar(1.7));
            assert_eq!(one_per_bar,bar.bar(1.8));
            assert_eq!(one_per_bar,bar.bar(1.9));
            assert_eq!("[||                                                                                                  ]",bar.bar(2.0));
            assert_eq!("[||||||||||||||||||||||||||||||||||||||||||||||||||                                                  ]",bar.bar(50.0));
            assert_eq!("[||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||| ]",bar.bar(99.0));
            assert_eq!("[||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||]",bar.bar(100.0));
        }

    #[test]
    fn test_generate_out_len_0(){
        let bar = Bar::new(0);

        assert_eq!("[]",bar.bar(1.0));
        assert_eq!("[]",bar.bar(5.0));
        assert_eq!("[]",bar.bar(100.0));
        assert_eq!("[]",bar.bar(32.0));
        assert_eq!("[]",bar.bar(12.0));
    }
    #[test]
    fn test_generate_out_len_50(){
        let bar = Bar::new(50);
        let from_one_to_two_bar = "[                                                  ]";
        let from_two_to_four_bar = "[|                                                 ]";
        let full_bar = "[||||||||||||||||||||||||||||||||||||||||||||||||||]";
        let n98to99_bar = "[||||||||||||||||||||||||||||||||||||||||||||||||| ]";

        assert_eq!(from_one_to_two_bar,bar.bar(0.0));
        assert_eq!(from_one_to_two_bar,bar.bar(0.5));
        assert_eq!(from_one_to_two_bar,bar.bar(1.0));
        assert_eq!(from_one_to_two_bar,bar.bar(1.5));
        assert_eq!(from_one_to_two_bar,bar.bar(1.9));
        assert_eq!(from_two_to_four_bar,bar.bar(2.0));
        assert_eq!(from_two_to_four_bar,bar.bar(2.5));
        assert_eq!(from_two_to_four_bar,bar.bar(3.0));
        assert_eq!(from_two_to_four_bar,bar.bar(3.5));
        assert_eq!(from_two_to_four_bar,bar.bar(3.9));
        assert_eq!(full_bar,bar.bar(100.0));
        assert_eq!(n98to99_bar,bar.bar(98.0));
        assert_eq!(n98to99_bar,bar.bar(99.0));
        assert_eq!(n98to99_bar,bar.bar(99.9));


    }
    #[test]
    fn test_generate_out_len_10(){
        let bar = Bar::new(10);
        assert_eq!("[          ]",bar.bar(0.0));
        assert_eq!("[          ]",bar.bar(1.0));
        assert_eq!("[          ]",bar.bar(2.0));
        assert_eq!("[          ]",bar.bar(3.0));
        assert_eq!("[          ]",bar.bar(4.0));
        assert_eq!("[          ]",bar.bar(5.0));
        assert_eq!("[          ]",bar.bar(6.0));
        assert_eq!("[          ]",bar.bar(7.0));
        assert_eq!("[          ]",bar.bar(8.0));
        assert_eq!("[          ]",bar.bar(9.0));
        assert_eq!("[          ]",bar.bar(9.9));
        assert_eq!("[|         ]",bar.bar(10.0));
        assert_eq!("[||        ]",bar.bar(20.0));
        assert_eq!("[|||       ]",bar.bar(30.0));
        assert_eq!("[||||      ]",bar.bar(40.0));
        assert_eq!("[|||||     ]",bar.bar(50.0));
        assert_eq!("[||||||    ]",bar.bar(60.0));
        assert_eq!("[|||||||   ]",bar.bar(70.0));
        assert_eq!("[||||||||  ]",bar.bar(80.0));
        assert_eq!("[||||||||| ]",bar.bar(90.0));
        assert_eq!("[||||||||||]",bar.bar(100.0));
    }

    #[test]
    fn test_generate_out_len_200(){
        let bar = Bar::new(200);
        let n0to0dot5 = "[                                                                                                                                                                                                        ]";
        let n0dot5to1 = "[|                                                                                                                                                                                                       ]";
        let mut t:f32=0.0;
        let step:f32 = 0.00001;
        while t<=0.5{
            assert_eq!(n0to0dot5,bar.bar(t));
            t+=step;
        }
        let mut t:f32 = 0.5;
        while t<=1.0{
            assert_eq!(n0dot5to1,bar.bar(t));
            t+=step;
        }
    }

    #[test]
    #[should_panic]
    fn test_generate_out_will_panic_negative(){
        let bar = Bar::new(10);
        bar.bar(-1.0);    
    }
    #[test]
    #[should_panic]
    fn test_generate_out_will_panic_larger_than_100(){
        let bar = Bar::new(10);
        bar.bar(100.1);    
    }
}