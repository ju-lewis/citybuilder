
use std::io::{Read, stdout};
use std::process::exit;

use termion::async_stdin;
use termion::raw::IntoRawMode;

use crate::world::map::Map;
use crate::rendering::framebuffer::Framebuffer;


pub struct Game {
    //paused: bool,
    map: Map,
    framebuffer: Framebuffer
}


impl Game {

    pub fn new() -> Self {

        let buf = Framebuffer::new();

        Game {
            //paused: false,
            map: Map::new(buf.get_size()),
            framebuffer: buf
        }
    }


    /// Renders the entire map into the framebuffer (even if there are no changes)
    /// Very inefficient but simple for testing.
    pub fn debug_render_all(&mut self) {

        for row in 0..self.map.size.0 {
            for col in 0..self.map.size.1 {
                let pixel = &mut self.framebuffer.buf[row][col];
                let map_cell = &self.map.cells[row][col];

                pixel.set(map_cell.render());
            }
        }
    }


    pub fn play(&mut self) {

        // Force terminal into raw mode
        let stdout = stdout().into_raw_mode().unwrap();
        let mut stdin = async_stdin();
        let mut key_buf = [0u8; 1];


        // Game loop
        loop {

            //thread::sleep(Duration::from_millis(100));

            
            if stdin.read(&mut key_buf).is_ok() && key_buf[0] == b'q' {
                print!("{}{}{}", termion::style::Reset, termion::clear::All, termion::cursor::Restore);
                let _ = stdout.suspend_raw_mode();
                exit(0);
            }

            self.map.update_entities();

            self.debug_render_all();
            self.framebuffer.draw();
        }
    }

}




