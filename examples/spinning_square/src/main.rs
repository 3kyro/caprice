extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use caprice::Caprice;
use std::thread::spawn;
use std::sync::mpsc;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
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
        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            let transform = c.transform.trans(x, y)
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
    let mut caprice = Caprice::new();
    caprice.set_keywords(&vec!["green".to_owned(), "blue".to_owned()]);
    caprice.set_prompt("!:");
    caprice.init(true).unwrap();

    let (tx, rx) = mpsc::channel();
    let (exit_tx, exit_rx) = mpsc::channel();
    
    let thr = spawn(move || {

        loop {
            if let Ok(true) = exit_rx.try_recv() {
                // drop(caprice);
                dbg!("exit");
                break;
            }
            
            if let Ok(option) = caprice.parse() {
                if let Some(token) = option {
                    dbg!("token");
                    tx.send(token).unwrap();
                }

            } else {
                break;
            }
            
        }
    });

    let opengl = OpenGL::V3_2;
        let mut window: Window = WindowSettings::new(
                "spinning-square",
                [200, 200]
            )
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
                if let Ok(color) = rx.try_recv() {
                    match color.as_ref() {
                        "green" => app.bg_color = [0.0, 1.0, 0.0, 1.0],
                        "blue" => app.bg_color = [0.0, 0.0, 1.0, 1.0],
                        _ => {}
                    }
                };
                app.update(&u);
            }

            if let Some(c) = e.close_args() {
                exit_tx.send(true).unwrap();
                // thr.join().unwrap();
                break;
                
            }
        }


}