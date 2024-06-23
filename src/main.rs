extern crate piston;
extern crate piston_window;
extern crate find_folder;

use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::{RenderEvent, UpdateEvent};
use piston_window::{PistonWindow, WindowSettings, Texture, TextureSettings, clear, G2dTexture, Image, Flip, Transformed};
use find_folder::Search;
use std::collections::HashMap;

mod map;
use map::Map;

pub struct App {
    map: Map,
    textures: HashMap<String, G2dTexture>,
}

impl App {
    fn new(window: &mut PistonWindow) -> Self {
        let mut textures = HashMap::new();

        let assets = Search::ParentsThenKids(3, 3).for_folder("assets/images").unwrap();
        let texture_settings = TextureSettings::new();
        println!("App instance created and textures loaded.");

        textures.insert(
            "empty".to_string(),
            Texture::from_path(
                &mut window.create_texture_context(),
                assets.join("empty.png"),
                Flip::None,
                &texture_settings
            ).unwrap()
        );

        textures.insert(
            "obstacle".to_string(),
            Texture::from_path(
                &mut window.create_texture_context(),
                assets.join("obstacle.png"),
                Flip::None,
                &texture_settings
            ).unwrap()
        );

        textures.insert(
            "energy".to_string(),
            Texture::from_path(
                &mut window.create_texture_context(),
                assets.join("energy.png"),
                Flip::None,
                &texture_settings
            ).unwrap()
        );

        textures.insert(
            "mineral".to_string(),
            Texture::from_path(
                &mut window.create_texture_context(),
                assets.join("mineral.png"),
                Flip::None,
                &texture_settings
            ).unwrap()
        );

        textures.insert(
            "scientific_interest".to_string(),
            Texture::from_path(
                &mut window.create_texture_context(),
                assets.join("scientific_interest.png"),
                Flip::None,
                &texture_settings
            ).unwrap()
        );

          textures.insert(
               "robot".to_string(),
               Texture::from_path(
                    &mut window.create_texture_context(),
                    assets.join("robot.png"),
                    Flip::None,
                    &texture_settings
               ).unwrap()
          );

          textures.insert(
          "station".to_string(),
          Texture::from_path(
               &mut window.create_texture_context(),
               assets.join("station.png"),
               Flip::None,
               &texture_settings
          ).unwrap()
          );

        let map = Map::new(32, 24, 42);

        App {
            map,
            textures,
        }
    }

 fn render(&mut self, window: &mut PistonWindow, event: &piston_window::Event) {
    let tile_size = 32.0;

    window.draw_2d(event, |c, g, device| {
        clear([1.0; 4], g); // Clear the screen with white color

        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let texture = match tile {
                    map::Tile::Empty => &self.textures["empty"],
                    map::Tile::Obstacle => &self.textures["obstacle"],
                    map::Tile::Energy => &self.textures["energy"],
                    map::Tile::Mineral => &self.textures["mineral"],
                    map::Tile::ScientificInterest => &self.textures["scientific_interest"],
                    map::Tile::Robot => {
                        // Use texture for robot (assuming it's loaded)
                        &self.textures["robot"]
                    },
                    map::Tile::Station => {
                        // Use texture for station (assuming it's loaded)
                        &self.textures["station"]
                    },
                };
                println!("Rendering tile at position ({}, {})", x, y);

                let transform = c.transform.trans((x as f64) * tile_size, (y as f64) * tile_size);
                Image::new().draw(texture, &c.draw_state, transform, g);
            }
        }
    });
}


    fn update(&mut self) {
        println!("App state updated.");
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Ereea Project", [1024, 768])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(&mut window);

    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            app.render(&mut window, &e);
        }

        if let Some(_) = e.update_args() {
            app.update();
        }
    }
}
