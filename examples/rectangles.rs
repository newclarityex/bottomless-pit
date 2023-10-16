use bottomless_pit::Game;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::texture::Texture;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::colour::Colour;
use bottomless_pit::vectors::Vec2;

fn main() {
    let mut engine = EngineBuilder::new()
        .set_clear_colour(Colour::BLACK)
        .build()
        .unwrap();

    let texture = Texture::from_path(&engine, Some("texture"), "examples/bplogo.png")
        .unwrap()
        .register(&mut engine);
    let texture_material = MaterialBuilder::new().add_texture(texture).build(&mut engine);
    let regular_material = MaterialBuilder::new().build(&mut engine);

    let pos = Position {
        pos: Vec2 { x: 0.0, y: 0.0},
        regular_material,
        texture_material,
    };

    engine.run(pos);
}

struct Position {
    pos: Vec2<f32>,
    regular_material: Material,
    texture_material: Material,
}

impl Game for Position {
    fn render<'pass, 'others>(&'others mut self, mut render_handle: RenderInformation<'pass, 'others>) where 'others: 'pass {
        let defualt_size = Vec2{x: 50.0, y: 50.0};
        self.regular_material.add_rectangle(Vec2{x: 0.0, y: 0.0}, defualt_size, Colour::RED, &render_handle);
        self.regular_material.add_rectangle(self.pos, Vec2{x: 100.0, y: 100.0}, Colour::RED, &render_handle);
        self.texture_material.add_rectangle(Vec2{x: 0.0, y: 50.0}, defualt_size, Colour::WHITE, &render_handle);
        self.texture_material.add_rectangle_with_uv(Vec2{x: 0.0, y: 100.0}, defualt_size, Vec2{x: 311.0, y: 311.0}, Vec2{x: 311.0, y: 311.0}, Colour::WHITE, &render_handle);
        self.regular_material.add_rectangle_with_rotation(Vec2{x: 0.0, y: 150.0}, defualt_size, Colour::GREEN, 45.0, &render_handle);

        let points = [
            Vec2{x: 0.0, y: 300.0},
            Vec2{x: 80.0, y: 290.0},
            Vec2{x: 100.0, y: 400.0},
            Vec2{x: 60.0, y: 400.0},
        ];
        let uvs = [
            Vec2{x: 0.0, y: 0.0},
            Vec2{x: 1.0, y: 0.0}, 
            Vec2{x: 1.0, y: 1.0},
            Vec2{x: 0.0, y: 1.0},
        ];

        self.regular_material.add_custom(points, uvs, 0.0 , Colour::RED, &render_handle);

        self.texture_material.draw(&mut render_handle);
        self.regular_material.draw(&mut render_handle);
    }

    fn update(&mut self, engine_handle: &mut Engine) {
        let dt = engine_handle.get_frame_delta_time();
        println!("{}", dt);
        self.pos.x += 100.0 * dt;
    }
}
