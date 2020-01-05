// cargo run --example spinning_square
//
// The turorial is based entirely on the spinning square tutorial
// from the Piston Game Engine
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
extern crate graphics;
extern crate piston;

extern crate glutin_window;
extern crate opengl_graphics;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use caprice::{Caprice, CapriceCommand};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    pub bg_color: [f32; 4],
    pub fg_color: [f32; 4],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let fg_color = self.fg_color;
        let bg_color = self.bg_color;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(fg_color, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // create a caprice instance
    let mut caprice = Caprice::new().disable_ctrl_c().init();

    // set keywords
    caprice.set_keywords(&[
        "exit".to_owned(),
        "red_square".to_owned(),
        "green_square".to_owned(),
        "red_background".to_owned(),
        "green_background".to_owned(),
        "blue_square".to_owned(),
        "blue_background".to_owned(),
    ]);

    // get the caprice channels
    let (tx, rx, handle) = caprice.run().unwrap();

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .automatic_close(false)
        .build()
        .unwrap();
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        bg_color: [0.0, 1.0, 0.0, 1.0],
        fg_color: [1.0, 0.0, 0.0, 1.0],
    };
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            // check if we received a token from caprice
            if let Ok(color) = rx.try_recv() {
                // and react accordingly
                match color.as_str() {
                    "red_background" => app.bg_color = [1.0, 0.0, 0.0, 1.0],
                    "green_background" => app.bg_color = [0.0, 1.0, 0.0, 1.0],
                    "blue_background" => app.bg_color = [0.0, 0.0, 1.0, 1.0],
                    "red_square" => app.fg_color = [1.0, 0.0, 0.0, 1.0],
                    "green_square" => app.fg_color = [0.0, 1.0, 0.0, 1.0],
                    "blue_square" => app.fg_color = [0.0, 0.0, 1.0, 1.0],
                    "exit" => {
                        tx.send(CapriceCommand::Exit).unwrap();
                        // wait for caprice to exit, otherwise the terminal
                        // might be left in raw mode on exit
                        handle.join().unwrap().unwrap();
                        // exit the main application
                        break;
                    }
                    _ => {}
                }
            };
            app.update(&u);
        }
    }
}
