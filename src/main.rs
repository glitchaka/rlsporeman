use macroquad::prelude::*;

const CELL: f32 = 32.0;
const SCALE: f32 = 3.0;
const SPEED: f32 = 180.0;
const FRAME_TIME: f32 = 0.12;
const WORLD: Vec2 = Vec2::new(2200.0, 1500.0);

fn window_conf() -> Conf {
    Conf {
        window_title: "rlsporeman - creature stage asset test".into(),
        window_width: 1152,
        window_height: 704,
        window_resizable: true,
        ..Default::default()
    }
}

fn b64_value(b: u8) -> Option<u8> {
    match b {
        b'A'..=b'Z' => Some(b - b'A'),
        b'a'..=b'z' => Some(b - b'a' + 26),
        b'0'..=b'9' => Some(b - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn decode_base64(src: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() * 3 / 4);
    let mut acc = 0u32;
    let mut bits = 0u8;
    for b in src.bytes() {
        if b == b'=' { break; }
        let Some(v) = b64_value(b) else { continue; };
        acc = (acc << 6) | v as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }
    out
}

fn creature_texture() -> Texture2D {
    let palette = [
        (0,0,0,0),(222,188,106,255),(194,146,63,255),(72,158,140,255),
        (110,131,96,255),(34,150,144,255),(30,139,136,255),(27,129,128,255),
        (159,109,39,255),(106,102,63,255),(44,116,111,255),(51,97,94,255),
        (21,118,117,255),(20,109,108,255),(18,101,100,255),(16,91,91,255),
        (112,70,53,255),(84,62,80,255),(69,65,61,255),(66,42,89,255),
        (43,70,70,255),(42,47,51,255),(43,35,68,255),(39,26,60,255),
        (14,81,81,255),(14,73,73,255),(12,62,62,255),(12,51,53,255),
        (12,44,45,255),(12,36,36,255),(12,24,30,255),(0,0,0,255),
    ];
    let packed = decode_base64(include_str!("creature.rle.b64"));
    let mut img = Image::gen_image_color(128, 256, Color::new(0.,0.,0.,0.));
    let mut p = 0usize;
    for pair in packed.chunks_exact(2) {
        let count = pair[0] as usize;
        let idx = pair[1] as usize;
        let (r,g,b,a) = palette[idx];
        let c = Color::from_rgba(r,g,b,a);
        for _ in 0..count {
            if p >= 128*256 { break; }
            img.set_pixel((p % 128) as u32, (p / 128) as u32, c);
            p += 1;
        }
    }
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    tex
}

fn dir_row(v: Vec2, last: usize) -> usize {
    if v.length_squared() < 0.001 { return last; }
    let a = v.y.atan2(v.x);
    // Atlas rows: S, SE, E, NE, N, NW, W, SW.
    let oct = ((a / std::f32::consts::FRAC_PI_4).round() as i32 + 8) % 8;
    match oct {
        0 => 2, // E
        1 => 1, // SE
        2 => 0, // S
        3 => 7, // SW
        4 => 6, // W
        5 => 5, // NW
        6 => 4, // N
        _ => 3, // NE
    }
}

fn blocked(next: Vec2, rocks: &[Rect]) -> bool {
    let r = Rect::new(next.x - 23.0, next.y - 18.0, 46.0, 36.0);
    rocks.iter().any(|x| r.overlaps(x))
}

#[macroquad::main(window_conf)]
async fn main() {
    let creature = creature_texture();
    let mut pos = vec2(500.0, 520.0);
    let mut last_dir = 0usize;
    let mut frame = 0usize;
    let mut anim_t = 0.0f32;

    let rocks = [
        Rect::new(760.0, 430.0, 120.0, 85.0),
        Rect::new(1010.0, 690.0, 150.0, 105.0),
        Rect::new(1450.0, 380.0, 105.0, 130.0),
        Rect::new(1690.0, 880.0, 180.0, 100.0),
    ];

    loop {
        let dt = get_frame_time().min(0.033);
        let mut input = Vec2::ZERO;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { input.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { input.x += 1.0; }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { input.y -= 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { input.y += 1.0; }
        if input.length_squared() > 0.0 { input = input.normalize(); }

        last_dir = dir_row(input, last_dir);
        let moving = input.length_squared() > 0.0;
        if moving {
            let dx = vec2(input.x * SPEED * dt, 0.0);
            let dy = vec2(0.0, input.y * SPEED * dt);
            let nx = vec2((pos.x + dx.x).clamp(40.0, WORLD.x-40.0), pos.y);
            if !blocked(nx, &rocks) { pos = nx; }
            let ny = vec2(pos.x, (pos.y + dy.y).clamp(40.0, WORLD.y-40.0));
            if !blocked(ny, &rocks) { pos = ny; }

            anim_t += dt;
            if anim_t >= FRAME_TIME {
                anim_t -= FRAME_TIME;
                frame = (frame + 1) % 4;
            }
        } else {
            frame = 0;
            anim_t = 0.0;
        }

        let cam = vec2(
            (pos.x - screen_width()/2.0).clamp(0.0, (WORLD.x-screen_width()).max(0.0)),
            (pos.y - screen_height()/2.0).clamp(0.0, (WORLD.y-screen_height()).max(0.0)),
        );

        clear_background(Color::from_rgba(121, 164, 95, 255));

        // Alien ground: broad patches, paths, plants and stones to give scale/depth.
        for y in (0..1500).step_by(96) {
            for x in (0..2200).step_by(96) {
                let odd = ((x/96 + y/96) & 1) == 0;
                let c = if odd { Color::from_rgba(117,158,89,255) } else { Color::from_rgba(125,169,96,255) };
                draw_rectangle(x as f32-cam.x, y as f32-cam.y, 96.0, 96.0, c);
            }
        }
        draw_circle(1120.0-cam.x, 520.0-cam.y, 280.0, Color::from_rgba(156,154,99,255));
        draw_circle(1780.0-cam.x, 350.0-cam.y, 190.0, Color::from_rgba(106,151,102,255));

        for i in 0..42 {
            let wx = ((i*173 + 91) % 2150) as f32;
            let wy = ((i*271 + 133) % 1450) as f32;
            let sx = wx-cam.x; let sy = wy-cam.y;
            draw_line(sx,sy,sx-5.0,sy-13.0,3.0,Color::from_rgba(55,111,67,255));
            draw_line(sx,sy,sx+7.0,sy-10.0,3.0,Color::from_rgba(64,122,71,255));
        }

        for r in rocks {
            let x=r.x-cam.x; let y=r.y-cam.y;
            draw_ellipse(x+r.w/2.0, y+r.h/2.0+8.0, r.w*0.48, r.h*0.36, 0.0, Color::new(0.,0.,0.,0.18));
            draw_ellipse(x+r.w/2.0, y+r.h/2.0, r.w*0.5, r.h*0.5, 0.0, Color::from_rgba(92,93,78,255));
            draw_ellipse(x+r.w*0.43, y+r.h*0.38, r.w*0.31, r.h*0.26, 0.0, Color::from_rgba(124,126,98,255));
        }

        let draw_size = CELL*SCALE;
        let foot = vec2(pos.x-cam.x, pos.y-cam.y);
        draw_ellipse(foot.x, foot.y+15.0, 31.0, 13.0, 0.0, Color::new(0.,0.,0.,0.22));
        let source = Rect::new(frame as f32*CELL, last_dir as f32*CELL, CELL, CELL);
        draw_texture_ex(
            &creature,
            foot.x-draw_size/2.0,
            foot.y-draw_size+25.0,
            WHITE,
            DrawTextureParams { dest_size: Some(vec2(draw_size,draw_size)), source: Some(source), ..Default::default() },
        );

        draw_rectangle(12.0,12.0,430.0,68.0,Color::new(0.,0.,0.,0.52));
        draw_text("WASD / flechas: mover en 8 direcciones",24.0,39.0,22.0,WHITE);
        draw_text(&format!("dir: {}   frame: {}   {}", last_dir, frame+1, if moving {"WALK"} else {"IDLE"}),24.0,64.0,20.0,WHITE);

        next_frame().await;
    }
}
