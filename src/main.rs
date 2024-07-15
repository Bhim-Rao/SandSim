#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use std::time::Instant;

const STEP: f32 = 2.0;

#[macroquad::main(window_conf)]
async fn main() {
    let w_i = (screen_width()/STEP) as i32;
    let h_i = (screen_height()/STEP) as i32;
    let dirs: Vec<i32> = vec![1, -1];
    let mut g = grid();
    let mut mouse_x;
    let mut mouse_y;
    let mut m_x;
    let mut m_y;
    let mut y_u;
    let mut x_u;
    let mut state;
    let mut dir;
    let mut below;
    let mut b_d;
    let mut a_d;
    loop {
        let now = Instant::now();
        let mut ng = grid();
        let background = Color::from_hex(0x0f0f0f);
        clear_background(background);
        if is_mouse_button_down(MouseButton::Left) {
            (mouse_x,mouse_y) = mouse_position();
            if mouse_x < screen_width()-(STEP*40.0) && mouse_y < screen_height()-(STEP*40.0) && mouse_x > 0.0 && mouse_y > 0.0 {
                m_x = ((mouse_x/STEP)) as usize;
                m_y = ((mouse_y/STEP)) as usize;
                for i in 0..40 {
                    for j in 0..40 {
                        g[(m_y + i) as usize][(m_x + j) as usize] = 1;
                    }
                }
            }
        }
        for x in 0..w_i {
            for y in 0..h_i {
                y_u = y as usize;
                x_u = x as usize;
                state = g[y_u][x_u];
                if state == 1 {
                    draw_rectangle((x as f32)*STEP, (y as f32)*STEP, STEP, STEP, GREEN);
                    if y == h_i-1 || x == w_i-1 || y == 0 || x == 0 {
                        ng[y_u][x_u] = 1;
                    } else {
                        dir = *(dirs.choose().unwrap_or(&(-1 as i32))) as i32;
                        a_d = (x+dir) as usize;
                        b_d = (x-dir) as usize;
                        below = g[y_u+1][x_u];
                        if below == 0 {
                            ng[y_u+1][x_u] = 1;
                        } else {
                            let belowa = g[y_u+1][a_d];
                            let belowb = g[y_u+1][b_d];
                            if belowa == 0 {
                                ng[y_u+1][a_d] = 1;
                            } else if belowb == 0 {
                                ng[y_u+1][b_d] = 1;
                            } else {
                                ng[y_u][x_u] = 1;
                            }
                        }
                    }
                }
            }
        }
        g = ng;
        let fps = (1.0/((now.elapsed().as_millis() as f32)/1000.0)) as i32;
        draw_text(&(fps.to_string() + "fps"), 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}

fn grid() -> Vec<Vec<i32>> {
    let x_values = vec![0i32; (screen_width()/STEP) as usize];
    let y_values = vec![x_values.clone(); (screen_height()/STEP) as usize];
    return y_values;
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_title: "Sand Simulation".to_string(),
        ..Default::default()
    }
}
                        // if current == 1 && y < (((screen_height()/step)-1.0) as i32) && x < (((screen_width()/step)-1.0) as i32) {
                        //     draw_rectangle((x as f32)*step, (y as f32)*step, step as f32, step as f32, BLACK);
                        // }
                        // if y_values[y_u+1][x_u] == 0 {
                        //     y_values[y_u][x_u] = 0;
                        //     y_values[y_u+1][x_u] = 1;
                        // } else if y_values[y_u+1][x_u] == 1 {
                        //     if y_values[y_u+1][x_u+1] == 0 {
                        //         y_values[y_u][x_u] = 0;
                        //         y_values[y_u+1][x_u+1] = 1;
                        //     } else if y_values[y_u+1][x_u-1] == 0 {
                        //         y_values[y_u][x_u] = 0;
                        //         y_values[y_u+1][x_u-1] = 1;
                        //     }
                        // }