use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};

use crate::prelude::ColorMode;

#[derive(PartialEq)]
pub struct ColorAttribute {
    pub foreground: Color,
    pub background: Color,
}

pub fn print_image(
    ascii_matrix: &Vec<Vec<char>>,
    brightness_matrix: &Vec<Vec<u8>>,
    pixel_matrix: &Vec<Vec<[u8; 3]>>,
    color_mode: &ColorMode,
) -> Result<(), Error> {
    ascii_matrix
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            let pixel_row = &pixel_matrix[row_index];
            let brightness_row = &brightness_matrix[row_index];

            // a bit of optimization wouldn't bother
            let mut last: Option<ColorAttribute> = None;

            row.iter().enumerate().for_each(|(pixel_index, pixel)| {
                let mut color_attr: Option<ColorAttribute> = None;

                match color_mode {
                    ColorMode::Green => {
                        let pixel_brightness = brightness_row[pixel_index];
                        let foreground_color = Color::Rgb {
                            r: 0,
                            g: pixel_brightness,
                            b: 0,
                        };
                        let background_color = Color::Black;

                        color_attr = Some(ColorAttribute {
                            foreground: foreground_color,
                            background: background_color,
                        });
                    }
                    ColorMode::TrueColor => {
                        let pixel_color = pixel_row[pixel_index];
                        let foreground_color = Color::Rgb {
                            r: pixel_color[0],
                            g: pixel_color[1],
                            b: pixel_color[2],
                        };
                        let background_color = Color::Black;

                        color_attr = Some(ColorAttribute {
                            foreground: foreground_color,
                            background: background_color,
                        });
                    }
                    _ => (),
                }

                if let Some(ca) = color_attr {
                    // we only want to set the attribute if the current pixel's color is
                    // different from the last one
                    if last.is_none() || last.as_ref().is_some_and(|lca| *lca != ca) {
                        // set the attribute and ignore the result
                        let _ = set_color_attribute(&ca);
                        last = Some(ca);
                    }
                }

                // // Printing 3 times to stretch image back
                let _ = print(&format!("{pixel}"));
                let _ = print(&format!("{pixel}"));
                let _ = print(&format!("{pixel}"));
            });
            print("\r\n").unwrap();
        });

    reset_color()?;

    execute()
}

pub fn print(string: &str) -> Result<(), Error> {
    queue_command(Print(string))?;
    Ok(())
}

pub fn set_color_attribute(attribute: &ColorAttribute) -> Result<(), Error> {
    queue_command(SetForegroundColor(attribute.foreground))?;

    queue_command(SetBackgroundColor(attribute.background))?;
    Ok(())
}

pub fn reset_color() -> Result<(), Error> {
    queue_command(ResetColor)?;
    Ok(())
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()?;
    Ok(())
}

fn queue_command<T: Command>(command: T) -> Result<(), Error> {
    queue!(stdout(), command)?;
    Ok(())
}
