use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Font;

use crate::traffic::Stats;

pub fn create_layout(canvas: &mut WindowCanvas, texture: &Texture) {
    board(canvas, texture, 0, 0);
    board(canvas, texture, 544, 0);
    board(canvas, texture, 0, 544);
    board_lake(canvas, texture, 544, 544);

    let mut r1: Rect;
    let mut r2: Rect;

    //corner top left

    r1 = Rect::new(48, 704, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(256, 16 * i, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    r1 = Rect::new(80, 576, 16, 16);
    r2 = Rect::new(256, 16 * 16, 16, 16);
    canvas.copy(texture, r1, r2).unwrap();

    r1 = Rect::new(80, 672, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(0 + 16 * i, 16 * 16, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    //corner bot left

    r1 = Rect::new(48, 704, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(256, 544 + 16 * i, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    r1 = Rect::new(144, 576, 16, 16);
    r2 = Rect::new(256, 528, 16, 16);
    canvas.copy(texture, r1, r2).unwrap();

    r1 = Rect::new(112, 704, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(0 + 16 * i, 528, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    //corner top right
    r1 = Rect::new(80, 704, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(528, 16 * i, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    r1 = Rect::new(144, 608, 16, 16);
    r2 = Rect::new(528, 16 * 16, 16, 16);
    canvas.copy(texture, r1, r2).unwrap();

    r1 = Rect::new(80, 672, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(544 + 16 * i, 16 * 16, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    //corner bot right

    r1 = Rect::new(80, 704, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(528, 544 + 16 * i, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    r1 = Rect::new(112, 576, 16, 16);
    r2 = Rect::new(528, 528, 16, 16);
    canvas.copy(texture, r1, r2).unwrap();

    r1 = Rect::new(112, 704, 16, 16);
    for i in 0..16 {
        r2 = Rect::new(544 + 16 * i, 528, 16, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }

    r1 = Rect::new(272, 32, 80, 96);
    r2 = Rect::new(50, 35, 160, 192);
    canvas.copy(texture, r1, r2).unwrap();

    r1 = Rect::new(112, 278, 47, 42);
    r2 = Rect::new(592, 576, 94, 84);
    canvas.copy(texture, r1, r2).unwrap();
}

pub fn update_layout(canvas: &mut WindowCanvas, texture: &Texture) {
    let mut r1: Rect;
    let mut r2: Rect;

    r1 = Rect::new(48, 608, 16, 16);
    for j in 0..16 {
        for i in 0..50 {
            r2 = Rect::new(272 + 16 * j, 16 * i, 16, 16);
            canvas.copy(texture, r1, r2).unwrap();
        }
    }

    r1 = Rect::new(48, 608, 16, 16);
    for j in 0..16 {
        for i in 0..50 {
            r2 = Rect::new(16 * i, 272 + 16 * j, 16, 16);
            canvas.copy(texture, r1, r2).unwrap();
        }
    }

    crosswalk(canvas, texture);
    yellow_line(canvas, texture);
    broken_line(canvas, texture);
}

fn board(canvas: &mut WindowCanvas, texture: &Texture, abs_x: i32, abs_y: i32) {
    // top left
    let mut src = Rect::new(208, 608, 16, 16);
    let mut dst = Rect::new(abs_x, abs_y, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    // top side
    src = Rect::new(272, 608, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x + 16 + i * 16, abs_y, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // top right
    src = Rect::new(304, 608, 16, 16);
    dst = Rect::new(abs_x + 240, abs_y, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    // left side
    src = Rect::new(208, 640, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x, abs_y + 16 + i * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // right side
    src = Rect::new(304, 640, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x + 240, abs_y + 16 + i * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // bottom right
    src = Rect::new(304, 704, 16, 16);
    dst = Rect::new(abs_x + 240, abs_y + 240, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    // bottom side
    src = Rect::new(272, 704, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x + 16 + i * 16, abs_y + 240, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // bottom left
    src = Rect::new(208, 704, 16, 16);
    dst = Rect::new(abs_x, abs_y + 240, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    //inside
    src = Rect::new(336, 640, 16, 16);
    for i in 0..14 {
        for j in 0..14 {
            dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + i * 16, 16, 16);
            canvas.copy(texture, src, dst).unwrap();
        }
    }
}

fn board_lake(canvas: &mut WindowCanvas, texture: &Texture, abs_x: i32, abs_y: i32) {
    // top left
    let mut src = Rect::new(208, 608, 16, 16);
    let mut dst = Rect::new(abs_x, abs_y, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    // top side
    src = Rect::new(272, 608, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x + 16 + i * 16, abs_y, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // top right
    src = Rect::new(304, 608, 16, 16);
    dst = Rect::new(abs_x + 240, abs_y, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    // left side
    src = Rect::new(208, 640, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x, abs_y + 16 + i * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // right side
    src = Rect::new(304, 640, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x + 240, abs_y + 16 + i * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // bottom right
    src = Rect::new(304, 704, 16, 16);
    dst = Rect::new(abs_x + 240, abs_y + 240, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    // bottom side
    src = Rect::new(272, 704, 16, 16);
    for i in 0..14 {
        dst = Rect::new(abs_x + 16 + i * 16, abs_y + 240, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    // bottom left
    src = Rect::new(208, 704, 16, 16);
    dst = Rect::new(abs_x, abs_y + 240, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    //inside
    src = Rect::new(80, 816, 16, 16);
    for i in 0..3 {
        for j in 0..14 {
            dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + i * 16, 16, 16);
            canvas.copy(texture, src, dst).unwrap();
        }
    }

    for i in 3..7 {
        for j in 0..9 {
            dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + i * 16, 16, 16);
            canvas.copy(texture, src, dst).unwrap();
        }
    }

    for j in 9..14 {
        dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + 6 * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    for i in 3..7 {
        for j in 12..14 {
            dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + i * 16, 16, 16);
            canvas.copy(texture, src, dst).unwrap();
        }
    }

    for i in 7..14 {
        dst = Rect::new(abs_x + 16, abs_y + 16 + i * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }
    for i in 7..14 {
        dst = Rect::new(abs_x + 224, abs_y + 16 + i * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }
    for j in 0..14 {
        dst = Rect::new(abs_x + 16 + j * 16, abs_y + 224, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    //lake

    //corner top left
    src = Rect::new(16, 768, 16, 16);
    dst = Rect::new(abs_x + 16 + 1 * 16, abs_y + 16 + 7 * 16, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    //corner bottom left
    src = Rect::new(16, 896, 16, 16);
    dst = Rect::new(abs_x + 16 + 1 * 16, abs_y + 16 + 12 * 16, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    //corner top right
    src = Rect::new(144, 768, 16, 16);
    dst = Rect::new(abs_x + 16 + 12 * 16, abs_y + 16 + 7 * 16, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    //corner bottom right
    src = Rect::new(144, 896, 16, 16);
    dst = Rect::new(abs_x + 16 + 12 * 16, abs_y + 16 + 12 * 16, 16, 16);
    canvas.copy(texture, src, dst).unwrap();

    //line top
    let mut rng = rand::thread_rng();
    let mut random_number: u32;

    for j in 2..12 {
        random_number = rng.gen_range(1..=3);
        match random_number {
            1 => src = Rect::new(48, 768, 16, 16),
            2 => src = Rect::new(80, 768, 16, 16),
            _ => src = Rect::new(112, 768, 16, 16),
        }
        dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + 7 * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    //line bot

    for j in 2..12 {
        random_number = rng.gen_range(1..=3);
        match random_number {
            1 => src = Rect::new(48, 896, 16, 16),
            2 => src = Rect::new(80, 896, 16, 16),
            _ => src = Rect::new(112, 896, 16, 16),
        }
        dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + 12 * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    //line left
    for j in 8..12 {
        random_number = rng.gen_range(1..=3);
        match random_number {
            1 => src = Rect::new(16, 800, 16, 16),
            2 => src = Rect::new(16, 832, 16, 16),
            _ => src = Rect::new(16, 864, 16, 16),
        }
        dst = Rect::new(abs_x + 16 + 1 * 16, abs_y + 16 + j * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    //line right
    for j in 8..12 {
        random_number = rng.gen_range(1..=3);
        match random_number {
            1 => src = Rect::new(144, 800, 16, 16),
            2 => src = Rect::new(144, 832, 16, 16),
            _ => src = Rect::new(144, 864, 16, 16),
        }
        dst = Rect::new(abs_x + 16 + 12 * 16, abs_y + 16 + j * 16, 16, 16);
        canvas.copy(texture, src, dst).unwrap();
    }

    //inside

    src = Rect::new(48, 816, 16, 16);
    for i in 8..12 {
        for j in 2..12 {
            dst = Rect::new(abs_x + 16 + j * 16, abs_y + 16 + i * 16, 16, 16);
            canvas.copy(texture, src, dst).unwrap();
        }
    }

    //terre
    for i in 9..12 {
        for j in 3..6 {
            src = Rect::new(224 + (32 * (i % 9)), 768 + (32 * (j % 3)), 16, 16);
            dst = Rect::new(abs_x + 16 + i * 16, abs_y + 16 + j * 16, 16, 16);
            canvas.copy(texture, src, dst).unwrap();
        }
    }
}

pub fn stats_layout(canvas: &mut WindowCanvas, stats: Stats, font: &Font, texture: &Texture) {
    canvas.clear();
    board(canvas, texture, 260, 260);
    let mut surface = font
        .render("statistics")
        .blended(sdl2::pixels::Color::BLACK)
        .unwrap();
    let mut size = surface.size();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let mut rect = Rect::new(280, 280, size.0 / 8, size.1 / 8);
    canvas.copy(&texture, None, rect).unwrap();
    for i in 0..7 {
        let mut text = format!("min time: {} s", stats.min_time);
        match i {
            0 => text = format!("avg velocity: {:.2} pxs/s", stats.average_velocity),
            1 => text = format!("total cars: {}", stats.total_cars),
            2 => text = format!("nbr of passed cars: {}", stats.nbr_passed),
            3 => text = format!("max velocity: {} pxs/s", stats.max_velocity),
            4 => text = format!("min velocity: {} psx/s", stats.min_velocity),
            5 => text = format!("max time: {} s", stats.max_time),
            _ => {}
        }
        surface = font
            .render(&text)
            .blended(sdl2::pixels::Color::BLACK)
            .unwrap();
        size = surface.size();
        rect = Rect::new(280, 320 + 25 * i, size.0 / 8, size.1 / 8);
        texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        canvas.copy(&texture, None, rect).unwrap();
    }
}

fn yellow_line(canvas: &mut WindowCanvas, texture: &Texture) {
    let mut r1: Rect;
    let mut r2: Rect;

    for i in 0..6 {
        r1 = Rect::new(22, 640, 6, 15);
        r2 = Rect::new(397, 0 + 40 * i, 10, 40);
        canvas.copy(texture, r1, r2).unwrap();
    }

    for i in 14..20 {
        r1 = Rect::new(22, 640, 6, 15);
        r2 = Rect::new(397, 0 + 40 * i, 10, 40);
        canvas.copy(texture, r1, r2).unwrap();
    }

    for i in 0..6 {
        r1 = Rect::new(48, 646, 15, 6);
        r2 = Rect::new(0 + 40 * i, 397, 40, 10);
        canvas.copy(texture, r1, r2).unwrap();
    }

    for i in 14..20 {
        r1 = Rect::new(48, 646, 15, 6);
        r2 = Rect::new(0 + 40 * i, 397, 40, 10);
        canvas.copy(texture, r1, r2).unwrap();
    }
}

fn crosswalk(canvas: &mut WindowCanvas, texture: &Texture) {
    let mut r1: Rect;
    let mut r2: Rect;

    for i in 0..14 {
        r1 = Rect::new(48, 672, 16, 16);
        r2 = Rect::new(280 + 16 * i, 10 + 40 * 6, 16, 26);
        canvas.copy(texture, r1, r2).unwrap();
    }
    r1 = Rect::new(48, 672, 12, 16);
    r2 = Rect::new(280 + 16 * 14, 10 + 40 * 6, 12, 26);
    canvas.copy(texture, r1, r2).unwrap();

    for i in 0..14 {
        r1 = Rect::new(48, 672, 16, 16);
        r2 = Rect::new(280 + 16 * i, 10 + 40 * 13, 16, 26);
        canvas.copy(texture, r1, r2).unwrap();
    }
    r1 = Rect::new(48, 672, 12, 16);
    r2 = Rect::new(280 + 16 * 14, 10 + 40 * 13, 12, 26);
    canvas.copy(texture, r1, r2).unwrap();

    for i in 0..14 {
        r1 = Rect::new(112, 672, 16, 16);
        r2 = Rect::new(10 + 40 * 6, 280 + 16 * i, 26, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }
    r1 = Rect::new(112, 672, 16, 12);
    r2 = Rect::new(10 + 40 * 6, 280 + 16 * 14, 26, 12);
    canvas.copy(texture, r1, r2).unwrap();

    for i in 0..14 {
        r1 = Rect::new(112, 672, 16, 16);
        r2 = Rect::new(10 + 40 * 13, 280 + 16 * i, 26, 16);
        canvas.copy(texture, r1, r2).unwrap();
    }
    r1 = Rect::new(112, 672, 16, 12);
    r2 = Rect::new(10 + 40 * 13, 280 + 16 * 14, 26, 12);
    canvas.copy(texture, r1, r2).unwrap();
}

fn broken_line(canvas: &mut WindowCanvas, texture: &Texture) {
    let mut r1: Rect;
    let mut r2: Rect;

    for j in 0..6 {
        if j != 0 && j != 3 {
            for i in 0..6 {
                r1 = Rect::new(118, 608, 4, 16);
                r2 = Rect::new(276 + 40 * j, 0 + 40 * i, 8, 30);
                canvas.copy(texture, r1, r2).unwrap();
            }
        }
    }

    for j in 0..6 {
        if j != 0 && j != 3 {
            for i in 14..20 {
                r1 = Rect::new(118, 608, 4, 16);
                r2 = Rect::new(276 + 40 * j, 10 + 40 * i, 8, 30);
                canvas.copy(texture, r1, r2).unwrap();
            }
        }
    }

    for j in 0..6 {
        if j != 0 && j != 3 {
            for i in 0..6 {
                r1 = Rect::new(80, 614, 16, 4);
                r2 = Rect::new(0 + 40 * i, 276 + 40 * j, 30, 8);
                canvas.copy(texture, r1, r2).unwrap();
            }
        }
    }

    for j in 0..6 {
        if j != 0 && j != 3 {
            for i in 14..20 {
                r1 = Rect::new(80, 614, 16, 4);
                r2 = Rect::new(10 + 40 * i, 276 + 40 * j, 30, 8);
                canvas.copy(texture, r1, r2).unwrap();
            }
        }
    }
}
