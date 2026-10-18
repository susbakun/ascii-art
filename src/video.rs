use std::{
    io::Read,
    process::{Command, Stdio},
};

use crate::{brightness::BrightnessMethod, matrix, prelude::ColorMode, terminal};

pub fn render_video(
    path: &str,
    w: usize,
    h: usize,
    brightness_method: BrightnessMethod,
    invert: bool,
    color_mode: &ColorMode,
) {
    let mut child = Command::new("ffmpeg")
        .args([
            "-loglevel",
            "error",
            "-re", // read at real-time speed, so ffmpeg does the pacing
            "-i",
            path,
            "-vf",
            &format!("fps=15,scale={w}:{h}"),
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't start ffmpeg");

    let mut src = child.stdout.take().expect("Failed to generate frames");
    let mut buffer = vec![0u8; w * h * 3];

    let _ = terminal::print("\x1b[2J"); // clear once

    while src.read_exact(&mut buffer).is_ok() {
        let pixel_matrix = matrix::pixel_matrix_from_raw(&buffer, w);
        let brightness_matrix = matrix::get_brightness_matrix(&pixel_matrix, &brightness_method);
        let ascii_matrix = matrix::get_ascii_matrix(&brightness_matrix, invert);

        let _ = terminal::print("\x1b[H"); // cursor back to top-left, overwrite the last frame

        terminal::print_image(
            &ascii_matrix,
            &brightness_matrix,
            &pixel_matrix,
            &color_mode,
        )
        .expect("Couldn't print the image");
    }
}
