use std::io::{Error, Write, stdout};
use crossterm::style::{
    Print, ResetColor, 
    SetBackgroundColor, SetForegroundColor,
    Color
};
use crossterm::{queue, Command};

pub struct Attribute {
    pub foreground: Color,
    pub background: Color,
}

pub fn print_image(
    ascii_matrix: &Vec<Vec<char>>, 
    brightness_matrix: &Vec<Vec<u8>>,
    in_green: bool
    ) -> Result<(), Error>
{
    ascii_matrix.iter().enumerate().for_each(|(row_index, row)| {
        let brightness_row = &brightness_matrix[row_index];

        row.iter().enumerate().for_each(|(pixel_index, pixel)| {
            if in_green {
                let pixel_brightness = brightness_row[pixel_index];
                annotate_text(pixel_brightness)
                    .expect("Failed to annotate");
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

pub fn set_attribute(attribute: &Attribute) -> Result<(), Error> {
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

fn annotate_text(pixel_brightness: u8) -> Result<(), Error> {

    let foreground = Color::Rgb { r: 0, g: pixel_brightness, b: 0 };
    let attribute = Attribute {
        foreground,
        background: Color::Black
    };

    set_attribute(&attribute)
}