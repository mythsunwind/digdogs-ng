extern crate sdl2;

use std::env;
use std::error::Error;
use std::path::Path;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs::File;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;

fn get_pixels_from_file(file: String, initial_seek: u64, amount: i32) -> Vec<u8> {

    let path = Path::new(&file);
    let mut v: Vec<u8> = Vec::new();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}",
            path.display(),
            why.description()),
        Ok(file) => {
            println!("File opened.");
            file
        },
    };

    file.seek(SeekFrom::Start(initial_seek));

    for _i in 0..amount {
        let mut buffer = vec![0u8; 1];
        file.read(&mut buffer);
        v.push(buffer[0]);
    }
    return v;
}

fn draw_pixels(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, pixels: Vec<u8>, sprite_size: i32, canvas_width: i32, canvas_height: i32,
               palette: [(u8, u8, u8); 256]) {
    let mut i = 0;
    let pixel_size: u32 = 2;

    for blocky in 0..(canvas_height/(sprite_size * pixel_size as i32)) {
        for blockx in 0..(canvas_width/(sprite_size * pixel_size as i32)) {
            for y in 0..sprite_size {
                for x in 0..sprite_size {
                    if i < pixels.len() {
                        let pixel = pixels[i as usize];
                        let posx = x * pixel_size as i32 + (blockx*sprite_size*pixel_size as i32);
                        let posy = y * pixel_size as i32 + (blocky*sprite_size*pixel_size as i32);
                        let (r, g, b) = palette[pixel as usize];
                        if blocky == 0 && blockx == 0 {
                            println!("{} {} #{}: {} {} {}", y, x, pixel, r, g, b)
                        }
                        canvas.set_draw_color(Color::RGB(r, g, b));
                        canvas.fill_rect(Rect::new(posx, posy, pixel_size, pixel_size));
                        i += 1;
                    }
                }
            }
        }
    }

    canvas.present();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file = &args[1];
    let sprite_size: i32 = (&args[2]).parse().unwrap();
    let initial_seek: u64 = (&args[3]).parse().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("digdogs dat viewer", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().software().build().unwrap();

    let pixel_size: i32 = 2;
    let amount = (800/pixel_size) * (600/pixel_size);
    
    let pixels = get_pixels_from_file(file.to_string(), initial_seek, amount);

    let canvas_width: i32 = 800;
    let canvas_height: i32 = 600;

    let palette: [(u8, u8, u8); 256] = [
        (0, 0, 0),
        (255, 255, 3),
        (126, 0, 0),
        (3, 0, 0),
        (4, 0, 0),
        (5, 0, 0),
        (6, 0, 0),
        (255, 255, 255),
        (204, 0, 109),
        (9, 0, 0),
        (10, 0, 0),
        (11, 0, 0),
        (12, 0, 0),
        (13, 0, 0),
        (0, 203, 255),
        (0, 0, 0),
        (228, 247, 255),
        (164, 196, 216),
        (122, 167, 191),
        (76, 118, 155),
        (20, 0, 0),
        (255, 190, 0),
        (255, 126, 1),
        (255, 0, 0),
        (191, 0, 0),
        (1, 101, 0), // 25
        (3, 192, 2),
        (1, 97, 0),
        (227, 80, 1),
        (5, 0, 204),
        (0, 125, 255),
        (2, 0, 126),
        (251, 236, 220),
        (216, 187, 168),
        (179, 147, 122),
        (151, 109, 67),
        (36, 0, 0),
        (228, 247, 255),
        (188, 220, 236),
        (152, 196, 212),
        (118, 171, 191),
        (89, 147, 171),
        (63, 122, 151),
        (40, 101, 131),
        (18, 80, 109),
        (5, 57, 88),
        (1, 35, 67),
        (0, 21, 44),
        (255, 255, 255),
        (236, 236, 236),
        (220, 220, 220), // 50
        (204, 204, 204),
        (187, 187, 187),
        (171, 171, 171),
        (155, 155, 155),
        (139, 139, 139),
        (122, 122, 122),
        (105, 105, 105),
        (84, 84, 84),
        (67, 67, 67),
        (49, 49, 49),
        (61, 0, 0),
        (62, 0, 0),
        (0, 0, 0),
        (255, 236, 236),
        (255, 204, 204),
        (255, 172, 172),
        (255, 143, 143),
        (255, 114, 114),
        (255, 80, 80),
        (255, 1, 1),
        (71, 0, 0),
        (212, 0, 0),
        (191, 0, 0),
        (167, 0, 0),
        (147, 0, 0), // 75
        (76, 0, 0),
        (101, 0, 0),
        (78, 0, 0),
        (57, 0, 0),
        (80, 0, 0),
        (216, 247, 152),
        (195, 235, 117),
        (179, 224, 89),
        (163, 212, 58),
        (147, 200, 31),
        (135, 188, 6),
        (122, 180, 1),
        (109, 164, 1),
        (101, 147, 1),
        (88, 131, 1),
        (76, 114, 1),
        (67, 97, 0),
        (53, 80, 0),
        (40, 63, 0),
        (31, 49, 0),
        (255, 251, 216),
        (255, 243, 180),
        (255, 232, 147),
        (255, 216, 114),
        (255, 195, 81), // 100
        (255, 176, 67),
        (255, 156, 53),
        (255, 135, 40),
        (255, 110, 31),
        (105, 0, 0),
        (106, 0, 0),
        (107, 0, 0),
        (0, 227, 0),
        (0, 178, 0),
        (0, 150, 0),
        (1, 126, 1),
        (236, 251, 255),
        (208, 239, 255),
        (180, 224, 255),
        (156, 204, 255),
        (127, 179, 255),
        (98, 147, 255),
        (73, 109, 255),
        (46, 66, 255),
        (22, 19, 255),
        (13, 14, 228),
        (8, 9, 200),
        (3, 6, 171),
        (3, 3, 147),
        (125, 0, 0), // 125
        (126, 0, 0),
        (0, 0, 63),
        (251, 251, 216),
        (239, 239, 190),
        (232, 228, 172),
        (223, 215, 150),
        (215, 203, 134),
        (203, 186, 113),
        (196, 176, 97),
        (186, 158, 81),
        (178, 146, 69),
        (162, 121, 52),
        (146, 97, 36),
        (130, 77, 24),
        (113, 52, 16),
        (97, 31, 2),
        (80, 12, 0),
        (67, 2, 0),
        (144, 0, 0),
        (145, 0, 0),
        (146, 0, 0),
        (147, 0, 0),
        (148, 0, 0),
        (149, 0, 0),
        (150, 0, 0), // 150
        (151, 0, 0),
        (152, 0, 0),
        (153, 0, 0),
        (154, 0, 0),
        (155, 0, 0),
        (156, 0, 0),
        (157, 0, 0),
        (158, 0, 0),
        (159, 0, 0),
        (160, 0, 0),
        (192, 239, 239),
        (172, 228, 228),
        (148, 216, 216),
        (126, 204, 204),
        (106, 191, 191),
        (166, 0, 0),
        (167, 0, 0),
        (54, 139, 147),
        (36, 114, 131),
        (22, 93, 114),
        (13, 71, 97),
        (172, 0, 0),
        (173, 0, 0),
        (174, 0, 0),
        (175, 0, 0), // 175
        (176, 0, 0),
        (177, 0, 0),
        (178, 0, 0),
        (179, 0, 0),
        (180, 0, 0),
        (181, 0, 0),
        (182, 0, 0),
        (183, 0, 0),
        (184, 0, 0),
        (185, 0, 0),
        (186, 0, 0),
        (187, 0, 0),
        (188, 0, 0),
        (189, 0, 0),
        (190, 0, 0),
        (191, 0, 0),
        (255, 255, 255),
        (193, 0, 0),
        (220, 220, 220),
        (195, 0, 0),
        (172, 171, 187),
        (155, 155, 171),
        (139, 139, 155),
        (199, 0, 0),
        (122, 122, 122), // 200
        (101, 101, 101),
        (202, 0, 0),
        (203, 0, 0),
        (204, 0, 0),
        (205, 0, 0),
        (206, 0, 0),
        (0, 0, 0),
        (159, 159, 175),
        (147, 147, 163),
        (135, 135, 151),
        (122, 122, 139),
        (109, 109, 126),
        (220, 220, 236),
        (204, 204, 220),
        (187, 187, 204), // 215
        (109, 114, 131),
        (101, 101, 118),
        (218, 0, 0),
        (219, 0, 0),
        (220, 0, 0),
        (221, 0, 0),
        (222, 0, 0),
        (223, 0, 0),
        (224, 0, 0),
        (225, 0, 0),
        (226, 0, 0),
        (227, 0, 0),
        (228, 0, 0),
        (229, 0, 0),
        (230, 0, 0),
        (231, 0, 0),
        (232, 0, 0),
        (233, 0, 0),
        (234, 0, 0),
        (235, 0, 0),
        (236, 0, 0),
        (237, 0, 0),
        (238, 0, 0),
        (239, 0, 0),
        (240, 0, 0),
        (241, 0, 0),
        (242, 0, 0),
        (243, 0, 0),
        (244, 0, 0),
        (245, 0, 0),
        (246, 0, 0),
        (247, 0, 0),
        (248, 0, 0),
        (249, 0, 0),
        (250, 0, 0), // 250
        (251, 0, 0),
        (252, 0, 0),
        (253, 0, 0),
        (254, 0, 0),
        (255, 0, 0),
    ];

    draw_pixels(&mut canvas, pixels, sprite_size, canvas_width, canvas_height, palette);

    let mut pos = 0;
    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                /*
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    let pixels = get_pixels_from_file(file.to_string(), initial_seek + (pos * 600), amount);
                    draw_pixels(&mut canvas, pixels, sprite_size, canvas_width, canvas_height);
                    pos+=1;
                },
                */
                _ => {}
            }
        }
    }
}
