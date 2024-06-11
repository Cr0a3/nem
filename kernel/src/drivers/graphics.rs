use limine::{framebuffer::Framebuffer, request::FramebufferRequest};

use crate::drivers::print::print_chr;


#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub mod colors {
    pub static RED: u32 = 0xcd3131;
    pub static GREEN: u32 = 0x0dbc79;
    pub static YELLOW: u32 = 0xe5e510;
    pub static BLUE: u32 = 0x2472c8;
    pub static MAGENTA: u32 = 0xbc3fbc;
    pub static CYAN	: u32 = 0x11a8cd;
    pub static WHITE: u32 = 0xe5e5e5;
    pub static BRIGHT_BLACK: u32 = 0x666666;
    pub static BRIGHT_RED: u32 = 0xf14c4c;
    pub static BRIGHT_GREEN: u32 = 0x23d18b;
    pub static BRIGHT_YELLOW: u32 = 0xf5f543;
    pub static BRIGHT_BLUE: u32 = 0x3b8eea;
    pub static BRIGHT_MAGENTA: u32 = 0xd670d6;
    pub static BRIGHT_CYAN: u32 = 0x29b8db;
    pub static BRIGHT_WHITE: u32 = 0xe5e5e5;
}
pub struct GraphicsDriver<'a>{
    pub framebuffer: Framebuffer<'a>,
}

impl<'a> GraphicsDriver<'a> {
    pub unsafe fn new() -> Option<Self> {
        if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
            if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
                Some(Self {
                    framebuffer: framebuffer,
                })
            } else { None }
        } else { None }
    }

    pub fn set_pixel(&self, x: i64, y: i64, rgb: u32) {
        
        let pixel_offset = x * self.framebuffer.pitch() as i64 + y * 4;
        let pixel_offset = pixel_offset as u64;

        unsafe {
            *(self.framebuffer.addr().add(pixel_offset as usize) as *mut u32) = rgb;
        }
    }

    pub fn rect(&self, x: i64, y: i64, size_x: i64, size_y: i64, rgb: u32) {
        for rel_x in 0..size_x {
            for rel_y in 0..size_y {
                self.set_pixel(x + rel_x, y + rel_y, rgb);
            }
        }
    }

    pub fn circle(&self, _x: i64, _y: i64, rad: i64, rgb: u32) {
        let mut x = rad;
        let mut y = 0;
        let mut p = 1 - rad;
    
        while x >= y {
            self.line_horiz(_x - x, _x + x, _y + y, rgb);
            self.line_horiz(_x - x, _x + x, _y - y, rgb);
            self.line_horiz(_x - y, _x + y, _y + x, rgb);
            self.line_horiz(_x - y, _x + y, _y - x, rgb);
            y += 1;
    
            if p <= 0 {
                p += 2 * y + 1;
            } else {
                x -= 1;
                p += 2 * y - 2 * x + 1;
            }
    
            self.line_horiz(_x - x, _x + x, _y + y, rgb);
            self.line_horiz(_x - x, _x + x, _y - y, rgb);
            self.line_horiz(_x - y, _x + y, _y + x, rgb);
            self.line_horiz(_x - y, _x + y, _y - x, rgb);
        }
    }
    
    pub fn line_horiz(&self, x_start: i64, x_end: i64, y: i64, rgb: u32) {
        for x in x_start..=x_end {
            if y >= 0  && x >= 0  {
                self.set_pixel(x, y, rgb);
            }
        }
    }

    pub fn rect_rounded(&self, _x: i64, _y: i64, _w: i64, _h: i64, r: i64, rgb: u32) {
        let w = _h; let x = _y;
        let h = _w; let y = _y;
        for i in x + r..x + w - r {
            for j in y..y + r {
                self.set_pixel(i, j, rgb); // Top
                self.set_pixel(i, y + h - 1 - (j - y), rgb); // Bottom
            }
        }

        for j in y + r..y + h - r {
            for i in x..x + r {
                self.set_pixel(i, j, rgb); // Left
                self.set_pixel(x + w - 1 - (i - x), j, rgb); // Right
            }
        }

        // Draw the four corners
        for i in 0..r {
            for j in 0..r {
                if i * i + j * j <= r * r {
                    self.set_pixel(x + r - i, y + r - j, rgb); // Top-left
                    self.set_pixel(x + w - r + i - 1, y + r - j, rgb); // Top-right
                    self.set_pixel(x + r - i, y + h - r + j - 1, rgb); // Bottom-left
                    self.set_pixel(x + w - r + i - 1, y + h - r + j - 1, rgb); // Bottom-right
                }
            }
        }

        // Fill the center rectangle
        for i in x + r..x + w - r {
            for j in y + r..y + h - r {
                self.set_pixel(i, j, rgb);
            }
        }
    }

    pub fn print(&self, msg: &str, x: &mut i64, y: &mut i64, scale: i64, rgb: u32) {
        for chr in msg.chars() {
            unsafe { print_chr(chr, *x, *y, rgb, scale, &self) };

            *x += 10 * scale;

            if chr == '\n' { *x = 0; *y += 17 * scale; };
            if *x >= ( self.framebuffer.width() as i64 - (10 * scale / 2) ) as i64 { *x = 0; * y += 17 * scale;};

        }
    }
}