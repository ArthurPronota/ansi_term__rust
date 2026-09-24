use ansi_term::Color ;
use ansi_term::Style ;

fn main() {
    // Вывод цветного текста
    println!("Red: {}, Blue: {}, Green: {}", 
        Color::Red.paint("red"),
        Color::Blue.paint("blue"),
        Color::Green.paint("green"),
    );

    // Bold color
    println!("Bold text: {}", Style::new().bold().paint("bold color")) ;

    // Color + bold
    println!("{}", Color::Yellow.bold().paint("bold yellow")) ;
}
