use std::cmp;

pub enum Color {
    White,
    Red,
    Black,
}

struct MemoryLocation {
    byte: usize,
    bit: u8,
}

pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn get_mem_loc(&self) -> Option<MemoryLocation> {
        let byte: usize = (self.y as usize * super::WIDTH) / 8 + (self.x as usize / 8);
        if byte > super::FRAME_SIZE {
            return None;
        }
        let bit = (self.x % 8) as u8;
        Some(MemoryLocation {
            byte: byte,
            bit: bit,
        })
    }
}

pub fn flood(frame: &mut super::Frame, color: Color) {
    match color {
        Color::White => frame.clear(),
        Color::Black => {
            for byte in frame.buffer_black.iter_mut() {
                *byte = 0x00;
            }
            frame.clear_red();
        }
        Color::Red => {
            for byte in frame.buffer_red.iter_mut() {
                *byte = 0x00;
            }
            frame.clear_black();
        }
    }
}

pub fn draw_point(frame: &mut super::Frame, point: Point, color: Color) {
    if let Some(point) = point.get_mem_loc() {
        match color {
            Color::White => {
                frame.buffer_black[point.byte] |= 0b1000_0000 >> point.bit;
                frame.buffer_red[point.byte] |= 0b1000_0000 >> point.bit;
            }
            Color::Black => {
                frame.buffer_black[point.byte] &= !(0b1000_0000 >> point.bit);
                frame.buffer_red[point.byte] |= 0b1000_0000 >> point.bit;
            }
            Color::Red => {
                frame.buffer_red[point.byte] &= !(0b1000_0000 >> point.bit);
            }
        }
    }
}

fn brush(color: Color) -> Box<Fn(usize, &mut super::Frame, u8) -> ()> {
    match color {
        Color::Red => Box::new(|address: usize, frame: &mut super::Frame, mask: u8| {
            frame.buffer_red[address] &= !mask;
        }),
        Color::Black => Box::new(|address: usize, frame: &mut super::Frame, mask: u8| {
            frame.buffer_black[address] &= !mask;
            frame.buffer_red[address] |= mask;
        }),
        Color::White => Box::new(|address: usize, frame: &mut super::Frame, mask: u8| {
            frame.buffer_black[address] |= mask;
            frame.buffer_red[address] |= mask;
        }),
    }
}

pub fn draw_rect(mut frame: &mut super::Frame, point: Point, width: isize, height: isize, color: Color) {
    //temporary: as i eventually want to allow negative width and height but that requires mor logic
    let point_in_memory = point.get_mem_loc().unwrap();

    let width = cmp::min(width, super::WIDTH as isize - point.x);
    let height = cmp::min(height, super::HEIGHT as isize - point.y);
    let write = brush(color);
    let mask_start = 0xFF >> point_in_memory.bit;
    let mut mask_end = !(0xFF >> (point.x + width) % 8);
    mask_end = match mask_end { 0x0 => 0xFF, _ => mask_end };
    let height = height as usize;
    if (width + point_in_memory.bit as isize) < 9 {
        let mask_start = mask_start & mask_end;
        for i in 0..height {
            write(point_in_memory.byte + i * super::WIDTH as usize / 8, &mut frame, mask_start);
        }
    } else {
        let width = width - (8 - point_in_memory.bit as isize);
        let width_range = (width / 8) as usize + match width % 8 {0 => 0, _ => 1};
        for i in 0..height {
            let current_address = point_in_memory.byte + i * super::WIDTH as usize / 8;
            write(current_address, &mut frame, mask_start);
            for j in 1..width_range {
                write(current_address + j, &mut frame, 0xFF);
            }
            write(current_address + width_range, &mut frame, mask_end)
        }
    }
}

pub fn draw_line(mut frame: &mut super::Frame, point_a: Point, point_b: Point, color: Color) {
    let point_r;
    let point_l;
    if point_a.x > point_b.x {
        point_r = point_a;
        point_l = point_b;
    } else {
        point_r = point_b;
        point_l = point_a;
    }
    let slope = (point_l.y - point_r.y) as f32 / (point_r.x - point_l.x) as f32;
    
}