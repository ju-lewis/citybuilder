
use crate::world::map::Map;
use crate::rendering::framebuffer::Framebuffer;


pub struct Game {
    paused: bool,
    map: Map,
    framebuffer: Framebuffer
}


impl Game {

    pub fn new() -> Self {

        let buf = Framebuffer::new();

        Game {
            paused: false,
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

        // Game loop
        loop {

            self.debug_render_all();
            
            
            self.framebuffer.draw();
        }
    }

}




