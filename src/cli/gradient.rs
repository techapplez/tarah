use random_number::random;
use crate::debug_exec;
use crate::helpers;



pub fn gout(text: &str, debug: bool) {
    let aa = random!(50..255);
    let ab = random!(50..255);
    let ac = random!(50..255);
    let ba = random!(50..255);
    let bb = random!(50..255);
    let bc = random!(50..255);
    
    let start_color = (aa, ab, ac);
    let end_color = (ba, bb, bc);

    for line in text.lines() {
        for (i, c) in line.chars().enumerate() {
            let progress = i as f32 / (line.len().max(1) - 1) as f32;

            let r = lerp(start_color.0, end_color.0, progress);
            let g = lerp(start_color.1, end_color.1, progress);
            let b = lerp(start_color.2, end_color.2, progress);

            print!("\x1b[38;2;{r};{g};{b}m{c}\x1b[0m");
        }
        println!();
    }
}

pub fn lerp(start: u8, end: u8, t: f32) -> u8 {
    ((start as f32) * (1.0 - t) + (end as f32) * t) as u8
}