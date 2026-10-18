use macroquad::prelude::*;

const FRAME_W: f32 = 160.0;
const FRAME_H: f32 = 180.0;
const FRAME_COUNT: usize = 6;
const FRAME_TIME: f32 = 0.11;
const SPEED: f32 = 110.0;
const DRAW_SCALE: f32 = 1.35;

fn window_conf() -> Conf {
    Conf {
        window_title: "rlsporeman - walking asset demo".to_owned(),
        window_width: 1100,
        window_height: 650,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture = Texture2D::from_file_with_format(
        include_bytes!("../assets/blippik_walk_spritesheet.png"),
        Some(ImageFormat::Png),
    );
    texture.set_filter(FilterMode::Nearest);

    let mut x = 70.0;
    let mut direction = 1.0f32;
    let mut frame = 0usize;
    let mut frame_acc = 0.0f32;
    let mut paused = false;

    loop {
        let dt = get_frame_time().min(0.05);

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        let mut input = 0.0f32;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            input -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            input += 1.0;
        }

        if !paused {
            if input != 0.0 {
                direction = input.signum();
                x += input * SPEED * 1.6 * dt;
            } else {
                x += direction * SPEED * dt;
            }

            frame_acc += dt;
            if frame_acc >= FRAME_TIME {
                frame_acc -= FRAME_TIME;
                frame = (frame + 1) % FRAME_COUNT;
            }
        }

        let sw = screen_width();
        let sh = screen_height();
        let draw_w = FRAME_W * DRAW_SCALE;
        let draw_h = FRAME_H * DRAW_SCALE;

        if x < 0.0 {
            x = 0.0;
            direction = 1.0;
        }
        if x > sw - draw_w {
            x = (sw - draw_w).max(0.0);
            direction = -1.0;
        }

        clear_background(Color::from_rgba(142, 184, 199, 255));
        let ground_y = sh * 0.73;

        for i in 0..12 {
            let bx = ((i * 190) as f32 % (sw + 200.0)) - 100.0;
            let extra = ((i % 3) * 12) as f32;
            draw_rectangle(
                bx,
                ground_y - 45.0 - extra,
                80.0,
                45.0 + extra,
                Color::from_rgba(109, 150, 162, 255),
            );
        }

        draw_rectangle(
            0.0,
            ground_y,
            sw,
            sh - ground_y,
            Color::from_rgba(111, 132, 80, 255),
        );
        draw_rectangle(
            0.0,
            ground_y,
            sw,
            8.0,
            Color::from_rgba(82, 102, 61, 255),
        );

        for i in 0..18 {
            let sx = ((i * 113 + 40) as f32) % sw.max(1.0);
            let sy = ground_y + 28.0 + ((i % 4) * 18) as f32;
            draw_rectangle(
                sx,
                sy,
                18.0 + ((i % 3) * 7) as f32,
                5.0,
                Color::from_rgba(124, 115, 94, 255),
            );
        }

        let y = ground_y - draw_h + 32.0;
        let source = Rect::new(frame as f32 * FRAME_W, 0.0, FRAME_W, FRAME_H);

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                source: Some(source),
                flip_x: direction < 0.0,
                ..Default::default()
            },
        );

        draw_rectangle(12.0, 12.0, 300.0, 78.0, Color::new(0.0, 0.0, 0.0, 0.58));
        draw_text(
            &format!("frame: {} / {}", frame + 1, FRAME_COUNT),
            24.0,
            38.0,
            22.0,
            WHITE,
        );
        draw_text(
            if paused { "estado: PAUSA" } else { "estado: CAMINANDO" },
            24.0,
            64.0,
            22.0,
            WHITE,
        );
        draw_text(
            "A/D o flechas: mover | Espacio: pausa",
            24.0,
            sh - 20.0,
            22.0,
            WHITE,
        );

        next_frame().await;
    }
}
