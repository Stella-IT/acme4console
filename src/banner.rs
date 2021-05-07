use std::io::{stdout, Write };
use crossterm::{
    style::{self, Colorize}, queue, Result
};
use crate::VERSION;

pub fn print_banner() -> Result<()> {
    let mut stdout = stdout();

    println!("ACME4Console - ver. {}", VERSION);
    print!("Copyright ");
    queue!(
        stdout,
        style::PrintStyledContent("Stella".cyan()),
        style::PrintStyledContent(" IT".blue()),
        style::PrintStyledContent(" Inc.".magenta())
    )?;
    println!(" - Distributed under MIT License");
    println!();

    stdout.flush();

    Ok(())
}