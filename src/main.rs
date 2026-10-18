use macroquad::prelude::*;

const FRAME_W: u16 = 64;
const FRAME_H: u16 = 64;
const SCALE: f32 = 3.0;
const SPEED: f32 = 230.0;
const JUMP: f32 = 470.0;
const GRAVITY: f32 = 1250.0;
const FRAME_TIME: f32 = 0.09;

fn window_conf() -> Conf {
    Conf {
        window_title: "rlsporeman - playable asset test".into(),
        window_width: 1152,
        window_height: 704,
        window_resizable: true,
        ..Default::default()
    }
}

fn px(img: &mut Image, x: i32, y: i32, c: Color) {
    if x >= 0 && y >= 0 && x < FRAME_W as i32 && y < FRAME_H as i32 {
        img.set_pixel(x as u32, y as u32, c);
    }
}

fn rect(img: &mut Image, x: i32, y: i32, w: i32, h: i32, c: Color) {
    for yy in y..y+h { for xx in x..x+w { px(img, xx, yy, c); } }
}

fn ellipse(img: &mut Image, cx: i32, cy: i32, rx: i32, ry: i32, c: Color) {
    let rx2 = (rx * rx).max(1) as f32;
    let ry2 = (ry * ry).max(1) as f32;
    for y in cy-ry..=cy+ry {
        for x in cx-rx..=cx+rx {
            let dx = (x-cx) as f32;
            let dy = (y-cy) as f32;
            if dx*dx/rx2 + dy*dy/ry2 <= 1.0 { px(img, x, y, c); }
        }
    }
}

fn line(img: &mut Image, mut x0: i32, mut y0: i32, x1: i32, y1: i32, c: Color) {
    let dx = (x1-x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1-y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        px(img, x0, y0, c);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2*err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

fn make_frame(frame: Option<usize>) -> Image {
    let mut img = Image::gen_image_color(FRAME_W, FRAME_H, Color::new(0.,0.,0.,0.));
    let outline = Color::from_rgba(28,35,42,255);
    let teal = Color::from_rgba(59,177,157,255);
    let teal2 = Color::from_rgba(39,135,126,255);
    let cream = Color::from_rgba(235,216,169,255);
    let purple = Color::from_rgba(105,71,151,255);
    let white = Color::from_rgba(246,243,234,255);
    let pupil = Color::from_rgba(31,24,38,255);
    let orange = Color::from_rgba(246,170,54,255);
    let claw = Color::from_rgba(229,220,194,255);

    let (ldx, rdx, bob, arm) = match frame {
        None => (0,0,0,0),
        Some(0) => (-2,2,0,2),
        Some(1) => (-1,1,-1,1),
        Some(2) => (0,0,0,0),
        Some(3) => (2,-2,1,-2),
        Some(4) => (1,-1,0,-1),
        _ => (0,0,-1,0),
    };

    // Cola y placas dorsales: mismas posiciones relativas en todos los cuadros.
    ellipse(&mut img, 14, 42+bob, 6, 4, outline);
    ellipse(&mut img, 14, 41+bob, 5, 3, teal2);
    for &(x,y,r) in &[(18,28,4),(15,35,4),(17,42,3)] {
        ellipse(&mut img, x, y+bob, r+1, r+1, outline);
        ellipse(&mut img, x, y+bob, r, r, purple);
    }

    // Piernas: la planta siempre termina en y=57.
    for (x,dx) in [(26,ldx),(40,rdx)] {
        ellipse(&mut img, x+dx, 48+bob, 5, 7, outline);
        ellipse(&mut img, x+dx, 48+bob, 4, 6, teal2);
        ellipse(&mut img, x+dx, 55, 8, 3, outline);
        ellipse(&mut img, x+dx, 54, 7, 2, teal);
    }
    for x in [21+ldx,26+ldx,35+rdx,40+rdx,45+rdx] { rect(&mut img, x,55,3,2,claw); }

    // Cuerpo principal: exactamente la misma silueta por frame.
    ellipse(&mut img, 33,36+bob,18,17,outline);
    ellipse(&mut img, 33,35+bob,17,16,teal);
    ellipse(&mut img, 35,41+bob,10,10,cream);

    // Brazos: solo cambia el swing.
    ellipse(&mut img,18,37+bob+arm,6,5,outline);
    ellipse(&mut img,18,37+bob+arm,5,4,teal);
    ellipse(&mut img,49,37+bob-arm,6,5,outline);
    ellipse(&mut img,49,37+bob-arm,5,4,teal);
    rect(&mut img,13,39+bob+arm,3,2,claw);
    rect(&mut img,52,39+bob-arm,3,2,claw);

    // Antenas ancladas a la cabeza.
    line(&mut img,27,22+bob,23,10+bob,outline);
    line(&mut img,28,22+bob,24,10+bob,teal);
    line(&mut img,39,22+bob,43,10+bob,outline);
    line(&mut img,38,22+bob,42,10+bob,teal);
    ellipse(&mut img,23,9+bob,4,4,outline);
    ellipse(&mut img,23,9+bob,3,3,orange);
    ellipse(&mut img,43,9+bob,4,4,outline);
    ellipse(&mut img,43,9+bob,3,3,orange);

    // Cara.
    for ex in [27,40] {
        ellipse(&mut img,ex,28+bob,6,7,outline);
        ellipse(&mut img,ex,28+bob,5,6,white);
        ellipse(&mut img,ex+1,29+bob,2,3,pupil);
        px(&mut img,ex,27+bob,WHITE);
    }
    rect(&mut img,31,36+bob,6,1,outline);
    rect(&mut img,33,37+bob,2,1,outline);

    // Marcas fijas para detectar cualquier inconsistencia entre cuadros.
    ellipse(&mut img,21,31+bob,2,2,purple);
    ellipse(&mut img,47,32+bob,2,2,purple);
    ellipse(&mut img,23,42+bob,2,2,teal2);

    img
}

fn player_hitbox(pos: Vec2) -> Rect {
    Rect::new(pos.x+54.0, pos.y+39.0, 84.0, 132.0)
}

fn overlaps(a: Rect, b: Rect) -> bool {
    a.x < b.x+b.w && a.x+a.w > b.x && a.y < b.y+b.h && a.y+a.h > b.y
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut frames = Vec::<Texture2D>::new();
    let idle = Texture2D::from_image(&make_frame(None));
    idle.set_filter(FilterMode::Nearest);
    frames.push(idle);
    for i in 0..6 {
        let t = Texture2D::from_image(&make_frame(Some(i)));
        t.set_filter(FilterMode::Nearest);
        frames.push(t);
    }

    let world_w = 2600.0;
    let platforms = [
        Rect::new(0.0,560.0,world_w,180.0),
        Rect::new(430.0,455.0,250.0,24.0),
        Rect::new(790.0,385.0,190.0,24.0),
        Rect::new(1120.0,485.0,300.0,24.0),
        Rect::new(1560.0,420.0,220.0,24.0),
        Rect::new(1940.0,350.0,260.0,24.0),
    ];

    let mut pos = vec2(120.0,350.0);
    let mut vel = Vec2::ZERO;
    let mut facing = 1.0_f32;
    let mut grounded = false;
    let mut walk_clock = 0.0;
    let mut walk_frame = 0usize;
    let mut camera_x = 0.0;

    loop {
        let dt = get_frame_time().min(0.033);
        let mut axis = 0.0_f32;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { axis -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { axis += 1.0; }
        vel.x = axis*SPEED;
        if axis != 0.0 { facing = axis.signum(); }

        if grounded && (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up)) {
            vel.y = -JUMP;
            grounded = false;
        }
        vel.y += GRAVITY*dt;

        // Movimiento horizontal + colisiones laterales.
        pos.x += vel.x*dt;
        pos.x = pos.x.clamp(-54.0, world_w-138.0);
        let mut hb = player_hitbox(pos);
        for p in platforms {
            if overlaps(hb,p) {
                if vel.x > 0.0 { pos.x = p.x-138.0; }
                else if vel.x < 0.0 { pos.x = p.x+p.w-54.0; }
                vel.x = 0.0;
                hb = player_hitbox(pos);
            }
        }

        // Movimiento vertical + suelo/plataformas.
        let prev_bottom = hb.y+hb.h;
        pos.y += vel.y*dt;
        grounded = false;
        hb = player_hitbox(pos);
        for p in platforms {
            if overlaps(hb,p) {
                if vel.y >= 0.0 && prev_bottom <= p.y+8.0 {
                    pos.y = p.y-171.0;
                    vel.y = 0.0;
                    grounded = true;
                } else if vel.y < 0.0 {
                    pos.y = p.y+p.h-39.0;
                    vel.y = 0.0;
                }
                hb = player_hitbox(pos);
            }
        }

        if pos.y > 760.0 { pos = vec2(120.0,350.0); vel = Vec2::ZERO; }

        if grounded && axis != 0.0 {
            walk_clock += dt;
            if walk_clock >= FRAME_TIME {
                walk_clock -= FRAME_TIME;
                walk_frame = (walk_frame+1)%6;
            }
        } else {
            walk_clock = 0.0;
            walk_frame = 0;
        }

        let target_camera = (pos.x - screen_width()*0.42).clamp(0.0,(world_w-screen_width()).max(0.0));
        camera_x += (target_camera-camera_x)*(1.0-(-10.0*dt).exp());

        clear_background(Color::from_rgba(135,183,202,255));

        // Fondo simple con parallax.
        for i in 0..18 {
            let x = i as f32*190.0 - (camera_x*0.25)%190.0;
            draw_rectangle(x,430.0-(i%3) as f32*28.0,100.0,130.0,Color::from_rgba(101,145,157,255));
        }

        for p in platforms {
            draw_rectangle(p.x-camera_x,p.y,p.w,p.h,Color::from_rgba(83,103,61,255));
            draw_rectangle(p.x-camera_x,p.y,p.w,6.0,Color::from_rgba(117,143,82,255));
        }

        let texture_index = if grounded && axis != 0.0 { walk_frame+1 } else { 0 };
        let draw_pos = vec2(pos.x-camera_x,pos.y);
        draw_texture_ex(
            &frames[texture_index],
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(FRAME_W as f32*SCALE,FRAME_H as f32*SCALE)),
                flip_x: facing < 0.0,
                ..Default::default()
            },
        );

        draw_rectangle(12.0,12.0,500.0,72.0,Color::new(0.0,0.0,0.0,0.55));
        draw_text("A/D o flechas: mover   ESPACIO/W/arriba: saltar",24.0,40.0,23.0,WHITE);
        draw_text(
            if grounded && axis != 0.0 { &format!("estado: WALK  frame {}/6",walk_frame+1) } else if grounded { "estado: IDLE" } else { "estado: AIR" },
            24.0,66.0,23.0,WHITE
        );

        next_frame().await;
    }
}
