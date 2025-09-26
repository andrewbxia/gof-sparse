use crate::types::{
    WHITE, GREEN, BLUE, RED, 
    BLACK, P16


};

const FONT_SLASH: [u8; 7] = [0b00001,0b00010,0b00100,0b01000,0b10000,0b00000,0b00000];

const FONT: [[u8; 7]; 11] = [
    [0b01110,0b10001,0b10011,0b10101,0b11001,0b10001,0b01110], // 0
    [0b00100,0b01100,0b00100,0b00100,0b00100,0b00100,0b01110], // 1
    [0b01110,0b10001,0b00001,0b00010,0b00100,0b01000,0b11111], // 2
    [0b01110,0b10001,0b00001,0b00110,0b00001,0b10001,0b01110], // 3
    [0b00010,0b00110,0b01010,0b10010,0b11111,0b00010,0b00010], // 4
    [0b11111,0b10000,0b11110,0b00001,0b00001,0b10001,0b01110], // 5
    [0b00110,0b01000,0b10000,0b11110,0b10001,0b10001,0b01110], // 6
    [0b11111,0b00001,0b00010,0b00100,0b01000,0b01000,0b01000], // 7
    [0b01110,0b10001,0b10001,0b01110,0b10001,0b10001,0b01110], // 8
    [0b01110,0b10001,0b10001,0b01111,0b00001,0b00010,0b01100], // 9
    FONT_SLASH // /
];


//thanks chatgpt
pub(crate) fn draw_fps(frame: &mut [u8], fps: usize, resolution: &P16) {
    let digits = if fps < 10 {
        vec![fps as u8]
    } else if fps < 100 {
        vec![(fps / 10) as u8, (fps % 10) as u8]
    } else {
        vec![(fps / 100) as u8, ((fps / 10) % 10) as u8, (fps % 10) as u8]
    };

    let color = WHITE;
    let x_offset = 2;
    let y_offset = 2;
    let digit_spacing = 2;
    let digit_width = 5;
    let _digit_height = 7;

    for (i, &d) in digits.iter().enumerate() {
        let font = FONT[d as usize];
        for (row, bits) in font.iter().enumerate() {
            for col in 0..digit_width {
                if (bits >> (digit_width - 1 - col)) & 1 != 0 {
                    let x = x_offset + i * (digit_width + digit_spacing) + col;
                    let y = y_offset + row;
                    if x < resolution.0 as usize && y < resolution.1 as usize {
                        let idx = (y * resolution.0 as usize + x) * 4;
                        frame[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
}


pub(crate) fn draw_actives_len(frame: &mut [u8], actives: usize, total: usize, resolution: &P16){
    let activesdigits = {
        let mut digitvec = Vec::new();

        let mut total = total; // total before actives before reversing
        if total == 0 {
            digitvec.push(0);
        }
        while total >= 10 {
            digitvec.push((total % 10) as u8);
            total /= 10;
        }
        digitvec.push(total as u8); // Ensure last digit is pushed

        digitvec.push(10); // slash

        let mut actives = actives;
        if actives == 0 {
            digitvec.push(0);
        }
        while actives >= 10 {
            digitvec.push((actives % 10) as u8);
            actives /= 10;
        }
        digitvec.push(actives as u8); // Ensure last digit is pushed
        digitvec.reverse();
        digitvec
    };

    let color = GREEN;
    let x_offset = 2;
    let y_offset = 12; // below FPS (FPS is at y=2, height=7, spacing=3)
    let digit_spacing = 2;
    let digit_width = 5;
    let digit_height = 7;

    for (i, &d) in activesdigits.iter().enumerate() {
        let font = FONT[d as usize];
        for (row, bits) in font.iter().enumerate() {
            for col in 0..digit_width {
                if (bits >> (digit_width - 1 - col)) & 1 != 0 {
                    let x = x_offset + i * (digit_width + digit_spacing) + col;
                    let y = y_offset + row;
                    if x < resolution.0 as usize && y < resolution.1 as usize {
                        let idx = (y * resolution.0 as usize + x) * 4;
                        frame[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
}
pub(crate) fn draw_activeness(frame: &mut [u8], activeness: usize, resolution: &P16){
    let activenessdigits = {
        let mut digitvec = Vec::new();

        let mut activeness = activeness;
        if activeness == 0 {
            digitvec.push(0);
        }
        while activeness >= 10 {
            digitvec.push((activeness % 10) as u8);
            activeness /= 10;
        }
        digitvec.push(activeness as u8); // Ensure last digit is pushed
        digitvec.reverse();
        digitvec
    };

    let color = BLUE;
    let x_offset = 2;
    let y_offset = 22; // below FPS (FPS is at y=2, height=7, spacing=3)
    let digit_spacing = 2;
    let digit_width = 5;
    let digit_height = 7;

    for (i, &d) in activenessdigits.iter().enumerate() {
        let font = FONT[d as usize];
        for (row, bits) in font.iter().enumerate() {
            for col in 0..digit_width {
                if (bits >> (digit_width - 1 - col)) & 1 != 0 {
                    let x = x_offset + i * (digit_width + digit_spacing) + col;
                    let y = y_offset + row;
                    if x < resolution.0 as usize && y < resolution.1 as usize {
                        let idx = (y * resolution.0 as usize + x) * 4;
                        frame[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
}

use log::{debug, error};
use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, ElementState, MouseButton},
    event_loop::EventLoop,
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{WindowAttributes, Window},
};

use std::sync::Arc;

use std::time::Instant;
use crate::types::{Pair, PPair, ToPack, Unpack, Stamp, Pack};
use crate::game::{Game};

pub(crate) fn gentlemen_synchronize_your_death_watches(bounds: (Pair, Pair), displayscale: f64, 
    zoomspeed: i32, resolution: P16) -> Result<(), Error>{
    
    let event_loop = EventLoop::new().unwrap();
    
    let window = Arc::new({
        let size = LogicalSize::new(resolution.0 as f64, resolution.1 as f64);
        let scaled_size = LogicalSize::new(
            resolution.0 as f64 * displayscale,
            resolution.1 as f64 * displayscale,
        );
        let attr = Window::default_attributes()
            .with_title("hey stop peeking everywhere")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .with_resizable(false);
        event_loop.create_window(attr).unwrap()
    });

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &*window);
        Pixels::new(resolution.0 as u32, resolution.1 as u32, surface_texture)?
    };
    let window_clone = Arc::clone(&window);

    let mut game = Game {
        bounds: bounds,
        ..Default::default()
    };
    game.insertglider();

    let mut paused = false;
    let mut draw_state: Option<bool> = None;

    // FPS calculation variables
    let mut last_fps_update = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0;

    let mut lastcursorpos: Pair = (0, 0);
    let mut targetbounds: (Pair, Pair) = game.bounds.clone();

    let mut fades: Vec<Vec<i8>> = Vec::new();
    fades.reserve(resolution.1 as usize);
    for y in 0..resolution.1 {
        fades.push(vec![0; resolution.0 as usize]);
    }
    let mut actives = game.active.len();
    let mut total = game.cells.len();
    let mut activeness = ((actives as f32 / total.max(1) as f32).powi(2) * 100.0) as usize;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        if let Key::Named(NamedKey::Space) = event.logical_key {
                             paused = !paused;
                        }
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => draw_state = Some(true),
                        (ElementState::Pressed, MouseButton::Right) => {
                            // game.cells.clear();
                            // game.active.clear();
                            // game.nmap.clear();
                            draw_state = Some(false);
                        },
                        (ElementState::Released, _) => draw_state = None,
                        // Mouse wheel scrolling to zoom in/out
                        // Only works if you add WindowEvent::MouseWheel to the match above
                        _ => (),
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let (cx, cy) = lastcursorpos;
                    let (min_x, min_y) = game.bounds.0;
                    let (max_x, max_y) = game.bounds.1;

                    let zoom = match delta{
                        winit::event::MouseScrollDelta::LineDelta(_, y) => y as i32 * zoomspeed,
                        winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as i32,
                    };

                    let zoom_factor = {
                        100 - zoom
                    }.max(10);


                    let new_min_x = cx + ((min_x - cx) * zoom_factor) / 100;
                    let new_max_x = cx + ((max_x - cx) * zoom_factor) / 100;
                    let new_min_y = cy + ((min_y - cy) * zoom_factor) / 100;
                    let new_max_y = cy + ((max_y - cy) * zoom_factor) / 100;


                    targetbounds = ((new_min_x, new_min_y), (new_max_x.max(new_min_x + 20), new_max_y.max(new_min_y + 11)));
                    // game.bounds
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if let Ok(pos) = pixels.window_pos_to_pixel(position.into()) {
                        let coord = PPair::topack(&game.mapsb((pos.0 as u16, pos.1 as u16), &resolution));
                        lastcursorpos = coord.unpack();

                        if let Some(is_drawing) = draw_state {
                            let coord = PPair::topack(&game.mapsb((pos.0 as u16, pos.1 as u16), &resolution));
                           if is_drawing {
                            for dx in -15..=15 {
                                for dy in -15..=15 {
                                    if rand::random::<u8>() % 5 != 0{
                                        continue;
                                    }
                                    let new_coord = PPair::pack(lastcursorpos.0 + dx, lastcursorpos.1 + dy);
                                    game.addcell(new_coord);
                                }
                            }
                               lastcursorpos = coord.unpack();

                               game.addcell(coord);
                           }
                           else {
                               game.removecell(&coord);
                           }
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    game.ts.bump();
                    if !paused {
                        game.processactives();
                    }
                    if(rand::random::<u8>() % 60 == 0) {
                        game.ts.stamp("processactives".to_string());
                        println!("Active cells: {}", game.active.len());
                        println!("cells: {}", game.cells.len());

                    }

                    game.bounds.0.0 += (targetbounds.0.0 - game.bounds.0.0) / 10;
                    game.bounds.0.1 += (targetbounds.0.1 - game.bounds.0.1) / 10;
                    game.bounds.1.0 += (targetbounds.1.0 - game.bounds.1.0) / 10;
                    game.bounds.1.1 += (targetbounds.1.1 - game.bounds.1.1) / 10;

                    frame_count += 1;
                    let now = Instant::now();
                    if now.duration_since(last_fps_update).as_secs_f32() >= 1.0 {
                        fps = frame_count;
                        frame_count = 0;
                        last_fps_update = now;
                    }
                    game.ts.bump();
                    // game.draw(pixels.frame_mut(), paused, &mut fades);
                    game.draw_optimized(pixels.frame_mut(), paused, &mut fades);
                    if rand::random::<u8>() % 60 == 0 {
                        game.ts.stamp("draw".to_string());
                    }
                    
                    if(game.lifetime % 20 == 1){
                        actives = game.active.len();
                        total = game.cells.len();
                        activeness = ((actives as f32 / total.max(1) as f32).powi(2) * 100.0) as usize;
                    }

                    draw_fps(pixels.frame_mut(), fps, &resolution);
                    draw_actives_len(pixels.frame_mut(), actives, total, &resolution);
                    draw_activeness(pixels.frame_mut(), activeness, &resolution);

                    if let Err(err) = pixels.render() {
                        error!("pixels.render() failed: {err}");
                        elwt.exit();
                        return;
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                window_clone.request_redraw();
            }
            _ => (),
        }
    }).unwrap();

    Ok(())
}