
mod rendering;


use rendering::framebuffer::Framebuffer;



fn main() {

    // Create framebuffer
    let buf = Framebuffer::new();


    // Game loop
    loop {
        
        
        buf.draw();
    }
    

}
