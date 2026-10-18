use macroquad::prelude::*;

const CELL: u16 = 48;
const SCALE: f32 = 3.0;
const SPEED: f32 = 210.0;
const FRAME_TIME: f32 = 0.12;

const BODIES: &[&str] = &[
    include_str!("../assets/creature_parts/bodies/body_round.part"),
    include_str!("../assets/creature_parts/bodies/body_long.part"),
    include_str!("../assets/creature_parts/bodies/body_heavy.part"),
    include_str!("../assets/creature_parts/bodies/body_thin.part"),
];
const EYES: &[&str] = &[
    include_str!("../assets/creature_parts/eyes/eyes_pair.part"),
    include_str!("../assets/creature_parts/eyes/eyes_cyclops.part"),
    include_str!("../assets/creature_parts/eyes/eyes_stalks.part"),
    include_str!("../assets/creature_parts/eyes/eyes_triple.part"),
    include_str!("../assets/creature_parts/eyes/eyes_wide.part"),
];
const MOUTHS: &[&str] = &[
    include_str!("../assets/creature_parts/mouths/mouth_beak.part"),
    include_str!("../assets/creature_parts/mouths/mouth_fangs.part"),
    include_str!("../assets/creature_parts/mouths/mouth_smile.part"),
    include_str!("../assets/creature_parts/mouths/mouth_snout.part"),
    include_str!("../assets/creature_parts/mouths/mouth_tusks.part"),
];
const LEGS: &[&str] = &[
    include_str!("../assets/creature_parts/legs/legs_short.part"),
    include_str!("../assets/creature_parts/legs/legs_long.part"),
    include_str!("../assets/creature_parts/legs/legs_insect.part"),
    include_str!("../assets/creature_parts/legs/legs_hoof.part"),
];
const ARMS: &[&str] = &[
    include_str!("../assets/creature_parts/arms/arms_short.part"),
    include_str!("../assets/creature_parts/arms/arms_long.part"),
    include_str!("../assets/creature_parts/arms/arms_claws.part"),
];
const TAILS: &[&str] = &[
    include_str!("../assets/creature_parts/tails/tail_short.part"),
    include_str!("../assets/creature_parts/tails/tail_fin.part"),
    include_str!("../assets/creature_parts/tails/tail_spike.part"),
];
const ANTENNAE: &[&str] = &[
    include_str!("../assets/creature_parts/antennae/antennae_straight.part"),
    include_str!("../assets/creature_parts/antennae/antennae_leaf.part"),
    include_str!("../assets/creature_parts/antennae/antennae_curled.part"),
];
const BACKS: &[&str] = &[
    include_str!("../assets/creature_parts/backs/back_plates.part"),
    include_str!("../assets/creature_parts/backs/back_spikes.part"),
    include_str!("../assets/creature_parts/backs/back_shell.part"),
];
const PATTERNS: &[&str] = &[
    include_str!("../assets/creature_parts/patterns/pattern_spots.part"),
    include_str!("../assets/creature_parts/patterns/pattern_stripes.part"),
    include_str!("../assets/creature_parts/patterns/pattern_belly.part"),
];

#[derive(Clone)]
struct PartSpec {
    name: String,
    kind: String,
    variant: String,
    primary: Color,
    secondary: Color,
    a: i32,
    b: i32,
}

struct PartTex {
    spec: PartSpec,
    tex: Texture2D,
}

struct Wardrobe {
    bodies: Vec<PartTex>,
    eyes: Vec<PartTex>,
    mouths: Vec<PartTex>,
    legs: Vec<PartTex>,
    arms: Vec<PartTex>,
    tails: Vec<PartTex>,
    antennae: Vec<PartTex>,
    backs: Vec<PartTex>,
    patterns: Vec<PartTex>,
}

#[derive(Default)]
struct Pick {
    body: usize,
    eyes: usize,
    mouth: usize,
    legs: usize,
    arms: usize,
    tail: usize,
    antennae: usize,
    back: usize,
    pattern: usize,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "rlsporeman - modular creature builder".into(),
        window_width: 1280,
        window_height: 760,
        window_resizable: true,
        ..Default::default()
    }
}

fn hex(s: &str, fallback: Color) -> Color {
    if s.len() != 6 {
        return fallback;
    }
    let Ok(v) = u32::from_str_radix(s, 16) else {
        return fallback;
    };
    Color::from_rgba(
        ((v >> 16) & 255) as u8,
        ((v >> 8) & 255) as u8,
        (v & 255) as u8,
        255,
    )
}

fn parse_spec(src: &str) -> PartSpec {
    let mut p = PartSpec {
        name: "unnamed".into(),
        kind: "body".into(),
        variant: "round".into(),
        primary: GREEN,
        secondary: WHITE,
        a: 8,
        b: 8,
    };
    for line in src.lines() {
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        match k.trim() {
            "name" => p.name = v.trim().into(),
            "kind" => p.kind = v.trim().into(),
            "variant" => p.variant = v.trim().into(),
            "primary" => p.primary = hex(v.trim(), p.primary),
            "secondary" => p.secondary = hex(v.trim(), p.secondary),
            "a" => p.a = v.trim().parse().unwrap_or(p.a),
            "b" => p.b = v.trim().parse().unwrap_or(p.b),
            _ => {}
        }
    }
    p
}

fn px(img: &mut Image, x: i32, y: i32, c: Color) {
    if x >= 0 && y >= 0 && x < CELL as i32 && y < CELL as i32 {
        img.set_pixel(x as u32, y as u32, c);
    }
}

fn rect(img: &mut Image, x: i32, y: i32, w: i32, h: i32, c: Color) {
    for yy in y..y + h {
        for xx in x..x + w {
            px(img, xx, yy, c);
        }
    }
}

fn ellipse(img: &mut Image, cx: i32, cy: i32, rx: i32, ry: i32, c: Color) {
    let rx2 = (rx * rx).max(1) as f32;
    let ry2 = (ry * ry).max(1) as f32;
    for y in cy - ry..=cy + ry {
        for x in cx - rx..=cx + rx {
            let dx = (x - cx) as f32;
            let dy = (y - cy) as f32;
            if dx * dx / rx2 + dy * dy / ry2 <= 1.0 {
                px(img, x, y, c);
            }
        }
    }
}

fn line(img: &mut Image, mut x0: i32, mut y0: i32, x1: i32, y1: i32, c: Color, w: i32) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        for oy in -(w / 2)..=(w / 2) {
            for ox in -(w / 2)..=(w / 2) {
                px(img, x0 + ox, y0 + oy, c);
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn tri(img: &mut Image, a: (i32, i32), b: (i32, i32), c: (i32, i32), col: Color) {
    fn edge(a: (i32, i32), b: (i32, i32), p: (i32, i32)) -> i32 {
        (p.0 - a.0) * (b.1 - a.1) - (p.1 - a.1) * (b.0 - a.0)
    }
    let min_x = a.0.min(b.0).min(c.0);
    let max_x = a.0.max(b.0).max(c.0);
    let min_y = a.1.min(b.1).min(c.1);
    let max_y = a.1.max(b.1).max(c.1);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x, y);
            let e1 = edge(a, b, p);
            let e2 = edge(b, c, p);
            let e3 = edge(c, a, p);
            if (e1 >= 0 && e2 >= 0 && e3 >= 0) || (e1 <= 0 && e2 <= 0 && e3 <= 0) {
                px(img, x, y, col);
            }
        }
    }
}

fn paint(s: &PartSpec, img: &mut Image, dir: usize, frame: usize) {
    let bob = [0, -1, 0, 1][frame];
    let gait = [(-2, 2), (-1, 1), (2, -2), (1, -1)][frame];
    let side = [0, 2, 4, 2, 0, -2, -4, -2][dir];
    let outline = Color::from_rgba(24, 29, 34, 255);

    match s.kind.as_str() {
        "body" => {
            let rx = (s.a / 2).max(5);
            let ry = (s.b / 2).max(6);
            ellipse(img, 24 + side, 24 + bob, rx, ry, outline);
            ellipse(img, 24 + side, 24 + bob, rx - 1, ry - 1, s.primary);
            if matches!(dir, 0 | 1 | 2 | 6 | 7) {
                ellipse(img, 25 + side, 30 + bob, (rx / 2).max(3), (ry / 3).max(4), s.secondary);
            }
        }
        "eyes" => {
            if s.variant == "stalks" {
                for (bx, tx) in [(19, 16), (29, 32)] {
                    line(img, bx + side, 18 + bob, tx + side, 5 + bob, outline, 3);
                    line(img, bx + side, 18 + bob, tx + side, 5 + bob, s.secondary, 1);
                    ellipse(img, tx + side, 4 + bob, 4, 4, outline);
                    ellipse(img, tx + side, 4 + bob, 3, 3, s.primary);
                }
            } else if s.variant == "cyclops" {
                ellipse(img, 24 + side, 15 + bob, 6, 7, outline);
                ellipse(img, 24 + side, 15 + bob, 5, 6, s.primary);
                ellipse(img, 25 + side, 16 + bob, 2, 3, s.secondary);
            } else if s.variant == "triple" {
                for (cx, cy) in [(18, 16), (30, 16), (24, 10)] {
                    ellipse(img, cx + side, cy + bob, 3, 3, outline);
                    ellipse(img, cx + side, cy + bob, 2, 2, s.primary);
                    px(img, cx + side, cy + bob, s.secondary);
                }
            } else {
                let spread = (s.b / 2).max(5);
                for cx in [24 - spread, 24 + spread] {
                    let radius = (s.a / 2).max(3);
                    ellipse(img, cx + side, 15 + bob, radius, radius, outline);
                    ellipse(img, cx + side, 15 + bob, (radius - 1).max(2), (radius - 1).max(2), s.primary);
                    ellipse(img, cx + side, 16 + bob, 2, 2, s.secondary);
                }
            }
        }
        "mouth" => {
            if s.variant == "beak" {
                tri(img, (18 + side, 23 + bob), (34 + side, 26 + bob), (18 + side, 29 + bob), s.primary);
            } else if s.variant == "fangs" {
                rect(img, 17 + side, 23 + bob, 14, 5, s.secondary);
                tri(img, (19 + side, 27 + bob), (22 + side, 27 + bob), (20 + side, 33 + bob), s.primary);
                tri(img, (27 + side, 27 + bob), (30 + side, 27 + bob), (29 + side, 33 + bob), s.primary);
            } else if s.variant == "snout" {
                ellipse(img, 24 + side, 25 + bob, 9, 5, outline);
                ellipse(img, 24 + side, 25 + bob, 8, 4, s.primary);
                ellipse(img, 20 + side, 25 + bob, 1, 1, s.secondary);
                ellipse(img, 28 + side, 25 + bob, 1, 1, s.secondary);
            } else if s.variant == "tusks" {
                line(img, 17 + side, 24 + bob, 31 + side, 24 + bob, s.secondary, 2);
                tri(img, (18 + side, 24 + bob), (13 + side, 32 + bob), (22 + side, 25 + bob), s.primary);
                tri(img, (30 + side, 24 + bob), (35 + side, 32 + bob), (26 + side, 25 + bob), s.primary);
            } else {
                line(img, 17 + side, 24 + bob, 24 + side, 27 + bob, s.primary, 1);
                line(img, 24 + side, 27 + bob, 31 + side, 24 + bob, s.primary, 1);
            }
        }
        "legs" => {
            if s.variant == "insect" {
                let sw = [-3, -1, 3, 1][frame];
                for yy in [29, 33, 37] {
                    line(img, 20 + side, yy, 7 + side + sw, yy + 7, s.primary, 2);
                    line(img, 28 + side, yy, 41 + side - sw, yy + 7, s.primary, 2);
                }
            } else {
                for (cx, off) in [(18, gait.0), (30, gait.1)] {
                    let h = s.b.max(10);
                    line(img, cx + side, 29, cx + side + off, 29 + h, s.primary, if s.variant == "long" { 4 } else { 5 });
                    rect(img, cx - 5 + side + off, 27 + h, 10, 4, s.secondary);
                }
            }
        }
        "arms" => {
            let sw = [-2, -1, 2, 1][frame];
            let len = s.a.max(8);
            line(img, 15 + side, 22, 15 - len / 2 + side, 25 + len / 2 + sw, s.primary, if s.variant == "long" { 4 } else { 5 });
            line(img, 33 + side, 22, 33 + len / 2 + side, 25 + len / 2 - sw, s.primary, if s.variant == "long" { 4 } else { 5 });
            if s.variant == "claws" {
                for ox in [7, 10, 13] {
                    tri(img, (ox + side, 31 + sw), (ox + 2 + side, 37 + sw), (ox + 3 + side, 31 + sw), s.secondary);
                }
                for ox in [35, 38, 41] {
                    tri(img, (ox + side, 31 - sw), (ox + 2 + side, 37 - sw), (ox + 3 + side, 31 - sw), s.secondary);
                }
            }
        }
        "tail" => {
            let len = s.a.max(10);
            if matches!(dir, 2 | 3) {
                tri(img, (18 + side, 30 + bob), (18 - len + side, 27 + bob), (17 + side, 36 + bob), s.primary);
            } else if matches!(dir, 5 | 6) {
                tri(img, (30 + side, 30 + bob), (30 + len + side, 27 + bob), (31 + side, 36 + bob), s.primary);
            } else {
                tri(img, (24 + side, 31 + bob), (24 - len / 2 + side, 44 + bob), (24 + len / 2 + side, 40 + bob), s.primary);
            }
            if s.variant == "fin" {
                tri(img, (24 + side, 37 + bob), (16 + side, 46 + bob), (32 + side, 45 + bob), s.secondary);
            } else if s.variant == "spike" {
                tri(img, (24 + side, 39 + bob), (20 + side, 47 + bob), (28 + side, 43 + bob), s.secondary);
            }
        }
        "antennae" => {
            for (bx, tx) in [(19, 17), (29, 31)] {
                line(img, bx + side, 15 + bob, tx + side, 4 + bob, s.secondary, 2);
                if s.variant == "leaf" {
                    ellipse(img, tx + side, 4 + bob, 5, 3, s.primary);
                } else if s.variant == "curled" {
                    ellipse(img, tx + side, 5 + bob, 3, 3, s.primary);
                } else {
                    ellipse(img, tx + side, 4 + bob, 3, 3, s.primary);
                }
            }
        }
        "back" => {
            if s.variant == "shell" {
                ellipse(img, 21 + side, 25 + bob, 12, 11, outline);
                ellipse(img, 21 + side, 25 + bob, 11, 10, s.primary);
                for yy in [20, 25, 30] {
                    line(img, 12 + side, yy + bob, 29 + side, yy + bob, s.secondary, 1);
                }
            } else {
                for (yy, sz) in [(15, 4), (21, 5), (28, 4), (34, 3)] {
                    if s.variant == "spikes" {
                        tri(img, (20 + side, yy + bob), (12 + side, yy + 4 + bob), (22 + side, yy + 5 + bob), s.primary);
                    } else {
                        ellipse(img, 19 + side, yy + bob, sz, sz, s.primary);
                    }
                }
            }
        }
        "pattern" => {
            if s.variant == "stripes" {
                for yy in [18, 23, 28, 33] {
                    line(img, 15 + side, yy + bob, 33 + side, yy + bob, s.primary, 2);
                }
            } else if s.variant == "belly" {
                for yy in [27, 32, 37] {
                    line(img, 21 + side, yy + bob, 28 + side, yy + bob, s.primary, 1);
                }
            } else {
                for (cx, cy) in [(17, 21), (30, 24), (22, 31), (32, 17)] {
                    ellipse(img, cx + side, cy + bob, 2, 2, s.primary);
                }
            }
        }
        _ => {}
    }
}

fn make_atlas(src: &str) -> PartTex {
    let spec = parse_spec(src);
    let mut atlas = Image::gen_image_color(CELL * 4, CELL * 8, Color::new(0., 0., 0., 0.));
    for d in 0..8 {
        for f in 0..4 {
            let mut cell = Image::gen_image_color(CELL, CELL, Color::new(0., 0., 0., 0.));
            paint(&spec, &mut cell, d, f);
            for y in 0..CELL as u32 {
                for x in 0..CELL as u32 {
                    let color = cell.get_pixel(x, y);
                    atlas.set_pixel(x + f as u32 * CELL as u32, y + d as u32 * CELL as u32, color);
                }
            }
        }
    }
    let tex = Texture2D::from_image(&atlas);
    tex.set_filter(FilterMode::Nearest);
    PartTex { spec, tex }
}

fn list(srcs: &[&str]) -> Vec<PartTex> {
    srcs.iter().map(|src| make_atlas(src)).collect()
}

fn wardrobe() -> Wardrobe {
    Wardrobe {
        bodies: list(BODIES),
        eyes: list(EYES),
        mouths: list(MOUTHS),
        legs: list(LEGS),
        arms: list(ARMS),
        tails: list(TAILS),
        antennae: list(ANTENNAE),
        backs: list(BACKS),
        patterns: list(PATTERNS),
    }
}

fn direction(v: Vec2, last: usize) -> usize {
    if v.length_squared() < 0.001 {
        return last;
    }
    let oct = ((v.y.atan2(v.x) / std::f32::consts::FRAC_PI_4).round() as i32 + 8) % 8;
    match oct {
        0 => 2,
        1 => 1,
        2 => 0,
        3 => 7,
        4 => 6,
        5 => 5,
        6 => 4,
        _ => 3,
    }
}

fn cycle(index: &mut usize, len: usize, delta: i32) {
    *index = ((*index as i32 + delta).rem_euclid(len as i32)) as usize;
}

fn draw_part(part: &PartTex, pos: Vec2, dir: usize, frame: usize) {
    let size = CELL as f32 * SCALE;
    draw_texture_ex(
        &part.tex,
        pos.x - size / 2.0,
        pos.y - size + 28.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            source: Some(Rect::new(
                frame as f32 * CELL as f32,
                dir as f32 * CELL as f32,
                CELL as f32,
                CELL as f32,
            )),
            ..Default::default()
        },
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = wardrobe();
    let mut pick = Pick::default();
    let mut active = 0usize;
    let mut pos = vec2(620.0, 500.0);
    let mut facing = 0usize;
    let mut frame = 0usize;
    let mut clock = 0.0f32;

    loop {
        let dt = get_frame_time().min(0.033);
        let mut movement = Vec2::ZERO;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { movement.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { movement.x += 1.0; }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { movement.y -= 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { movement.y += 1.0; }

        if movement.length_squared() > 0.0 {
            movement = movement.normalize();
            facing = direction(movement, facing);
            clock += dt;
            if clock >= FRAME_TIME {
                clock -= FRAME_TIME;
                frame = (frame + 1) % 4;
            }
        } else {
            clock = 0.0;
            frame = 0;
        }

        pos += movement * SPEED * dt;
        pos.x = pos.x.clamp(360.0, screen_width() - 70.0);
        pos.y = pos.y.clamp(100.0, screen_height() - 70.0);

        if is_key_pressed(KeyCode::Tab) {
            active = (active + 1) % 9;
        }
        let delta = if is_key_pressed(KeyCode::Q) { -1 } else if is_key_pressed(KeyCode::E) { 1 } else { 0 };
        if delta != 0 {
            match active {
                0 => cycle(&mut pick.body, w.bodies.len(), delta),
                1 => cycle(&mut pick.eyes, w.eyes.len(), delta),
                2 => cycle(&mut pick.mouth, w.mouths.len(), delta),
                3 => cycle(&mut pick.legs, w.legs.len(), delta),
                4 => cycle(&mut pick.arms, w.arms.len(), delta),
                5 => cycle(&mut pick.tail, w.tails.len(), delta),
                6 => cycle(&mut pick.antennae, w.antennae.len(), delta),
                7 => cycle(&mut pick.back, w.backs.len(), delta),
                _ => cycle(&mut pick.pattern, w.patterns.len(), delta),
            }
        }

        clear_background(Color::from_rgba(104, 147, 83, 255));
        for y in (0..800).step_by(64) {
            for x in (330..1400).step_by(64) {
                let c = if ((x / 64 + y / 64) & 1) == 0 {
                    Color::from_rgba(111, 154, 88, 255)
                } else {
                    Color::from_rgba(119, 162, 94, 255)
                };
                draw_rectangle(x as f32, y as f32, 64.0, 64.0, c);
            }
        }

        draw_ellipse(pos.x, pos.y + 18.0, 38.0, 14.0, 0.0, Color::new(0.0, 0.0, 0.0, 0.2));
        draw_part(&w.tails[pick.tail], pos, facing, frame);
        draw_part(&w.backs[pick.back], pos, facing, frame);
        draw_part(&w.legs[pick.legs], pos, facing, frame);
        draw_part(&w.bodies[pick.body], pos, facing, frame);
        draw_part(&w.patterns[pick.pattern], pos, facing, frame);
        draw_part(&w.arms[pick.arms], pos, facing, frame);
        draw_part(&w.eyes[pick.eyes], pos, facing, frame);
        draw_part(&w.mouths[pick.mouth], pos, facing, frame);
        draw_part(&w.antennae[pick.antennae], pos, facing, frame);

        draw_rectangle(0.0, 0.0, 340.0, screen_height(), Color::from_rgba(17, 20, 23, 245));
        draw_text("CREATURE BUILDER", 22.0, 38.0, 26.0, WHITE);
        draw_text("TAB: slot    Q/E: change part", 22.0, 64.0, 17.0, LIGHTGRAY);
        draw_text("WASD / arrows: move", 22.0, 86.0, 17.0, LIGHTGRAY);

        let rows = [
            ("BODY", &w.bodies[pick.body].spec.name),
            ("EYES", &w.eyes[pick.eyes].spec.name),
            ("MOUTH", &w.mouths[pick.mouth].spec.name),
            ("LEGS", &w.legs[pick.legs].spec.name),
            ("ARMS", &w.arms[pick.arms].spec.name),
            ("TAIL", &w.tails[pick.tail].spec.name),
            ("ANT", &w.antennae[pick.antennae].spec.name),
            ("BACK", &w.backs[pick.back].spec.name),
            ("PATTERN", &w.patterns[pick.pattern].spec.name),
        ];
        for (i, (key, name)) in rows.iter().enumerate() {
            let y = 112.0 + i as f32 * 34.0;
            let bg = if i == active { Color::from_rgba(59, 105, 91, 255) } else { Color::from_rgba(28, 33, 37, 255) };
            draw_rectangle(18.0, y, 304.0, 30.0, bg);
            draw_text(&format!("{:<8} {}", key, name), 28.0, y + 21.0, 17.0, WHITE);
        }

        next_frame().await;
    }
}
