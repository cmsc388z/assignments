use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = (x as f64) * 25.0;
    let gui_y = (y as f64) * 25.0;

    rectangle(color, [gui_x, gui_y, 25.0, 25.0], con.transform, g);
}

pub fn draw_rectange(
    color: Color,
    start_x: i32,
    start_y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_start_x = (start_x as f64) * 25.0;
    let gui_start_y = (start_y as f64) * 25.0;

    rectangle(
        color,
        [
            gui_start_x,
            gui_start_y,
            25.0 * (width as f64),
            25.0 * (height as f64),
        ],
        con.transform,
        g,
    );
}
