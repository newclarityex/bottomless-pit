use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::text::TextMaterial;
use bottomless_pit::vectors::Vec2;
use bottomless_pit::Game;
use bottomless_pit::vec2;

fn main() {
    let mut engine = EngineBuilder::new()
        .set_clear_colour(Colour::BLACK)
        .build()
        .unwrap();

    let text_mat = TextMaterial::new("this is a test", Colour::RED, 20.0, 20.0 * 1.3, &mut engine);

    let text_example = TextExample { text_mat, text: String::new()};

    engine.run(text_example);
}

struct TextExample {
    text_mat: TextMaterial,
    text: String,
}

impl Game for TextExample {
    fn render<'pass, 'others>(
        &'others mut self,
        mut render_handle: RenderInformation<'pass, 'others>,
    ) where
        'others: 'pass,
    {
        self.text_mat
            .add_instance(vec2! { 0.0 }, Colour::WHITE, &render_handle);

        self.text_mat.draw(&mut render_handle);
    }

    fn update(&mut self, engine_handle: &mut Engine) {
        let text = engine_handle.get_current_text();
        if let Some(s) = text {
            self.text.push_str(s);

            self.text_mat.set_text(&self.text, Colour::RED, engine_handle)
        }

        self.text_mat.prepare(engine_handle);
    }
}