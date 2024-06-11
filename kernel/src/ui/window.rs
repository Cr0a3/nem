use crate::{colors, println, GraphicsDriver};

use super::rel_pos;

pub struct Window {
    size_x: i64,
    size_y: i64,

    x: i64,
    y: i64,
}

impl Window {
    pub fn new(size: (i64, i64), pos: (i64, i64)) -> Self {
        Self {
            size_x: size.0,
            size_y: size.1,

            x: pos.0,
            y: pos.1,
        }
    }

    pub fn render(&self, graphics: &GraphicsDriver) {
        let (x, y) = rel_pos::middle_pos_to_left_top_pos(self.x, self.y, graphics.framebuffer.width() as i64, graphics.framebuffer.height() as i64);

        let x = x - self.size_x / 2;
        let y = y - self.size_y / 2;

        graphics.rect_rounded(x, y, self.size_x, self.size_y, 20, colors::BRIGHT_BLACK);
    }

    pub fn render_center_text(&self, msg: &str, size: i64, rgb: u32, graphics: &GraphicsDriver) {
        let msg_len = msg.chars().count() as i64;
        let msg_len_in_pixels =  msg_len * 15 * size + (10 * size);
        let msg_len_in_pixels = msg_len_in_pixels / 8;


        let pos: (i64, i64) = rel_pos::middle_pos_to_left_top_pos(msg_len_in_pixels + self.x, (self.y + self.size_y - size * 17) / 2, graphics.framebuffer.width() as i64, graphics.framebuffer.height() as i64);

        let mut x = pos.0 - self.size_x / 2;
        let mut y = pos.1 - self.size_y / 2;

        self.render(&graphics);

        graphics.print(msg, &mut x, &mut y, size, rgb);
    }
}