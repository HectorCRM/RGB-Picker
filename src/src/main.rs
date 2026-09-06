use Slider::Slider;
use macroquad::{self, color::{BLUE, Color, GRAY, GREEN, RED, WHITE}, shapes::draw_circle, window::{clear_background, next_frame, screen_width}};

#[macroquad::main("RGB Picker")]
async fn  main() {
    
    //Slider color rojo
    let mut red: f32 = 0.5;
    let mut slider_red: Slider = Slider::nuevo_slider("RED", 50.0, 400.0, 20.0, red, WHITE, RED);
    //Slider color verde
    let mut green: f32 = 0.5;
    let mut slider_green: Slider = Slider::nuevo_slider("GREEN", 150.0, 400.0, 20.0, green, WHITE, GREEN);
    //Slider color azul
    let mut blue: f32 = 0.5;
    let mut slider_blue: Slider = Slider::nuevo_slider("BLUE", 250.0, 400.0, 20.0, blue, WHITE, BLUE);

    loop{
        clear_background(GRAY);

        slider_red.pintar_slider(red);
        red = slider_red.mover_slider(red);

        slider_green.pintar_slider(green);
        green = slider_green.mover_slider(green);

        slider_blue.pintar_slider(blue);
        blue = slider_blue.mover_slider(blue);

        let mut color = Color::new(red, green, blue, 1.0);

        draw_circle(screen_width() / 2.0, 500.0, 80.0, color);



        next_frame().await;

    }
}
