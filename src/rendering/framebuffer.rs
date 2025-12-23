
use termion::color::{self, Color};



pub struct Pixel {
    pub val: String,
}

pub struct Framebuffer {
    pub buf: Vec<Vec<Pixel>>,
    size: (usize, usize)
}


impl Pixel {

    pub fn new() -> Self {
        Pixel {
            val: String::from(" ")
        }
    }

    pub fn set(&mut self, val: String) {
        
        // Ad-hoc render the pixel into its own buffer
        self.val = val;
    }
}

impl Clone for Pixel {
    fn clone(&self) -> Self {
        Pixel {
            val: self.val.clone()
        }
    }
}


impl Framebuffer {

    pub fn new() -> Self {

        let size = termion::terminal_size().expect("Unable to obtain terminal size.");


        let x = Framebuffer {
            buf: vec![vec![Pixel::new(); size.1.into()]; size.0.into()],
            size: (size.0.into(), size.1.into())
        };

        // TEMP
        //x.buf[10][10].set('#', color::LightGreen);


        return x;
    }

    pub fn draw(&self) {

        // First, compose a string containing the full contents of the framebuffer
        // 1. Join the pixels in each row
        // 2. Join the rendered rows together to form the final image
        let rendered = self.buf.iter()
            .map(|row| row.iter().fold(String::new(), |acc, x| acc + &x.val))
            .fold(String::new(), |acc, row| acc + "\r\n" + &row);
        
        print!("{}{}", termion::cursor::Goto(1,1), rendered);
    }

    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }
}


