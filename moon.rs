use std::{thread, time::Duration};
use std::io::{self, Write};

fn main() {
    // Clear screen first
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    // Animation frames for "Hello World!"
    let text = "Hello World!";
    let frames = vec![
        "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
    ];

    // Animation loop
    for _ in 0..3 { // Loop 3 times
        for (i, char) in text.chars().enumerate() {
            for frame in &frames {
                print!("\r{}", " ".repeat(text.len())); // Clear line
                print!("\r");
                
                // Print completed characters
                for j in 0..i {
                    print!("{}", text.chars().nth(j).unwrap());
                }
                
                // Print current animated character
                print!("{} {}", frame, char);
                io::stdout().flush().unwrap();
                
                thread::sleep(Duration::from_millis(50));
            }
            
            print!("\r{}", " ".repeat(text.len())); // Clear line
            print!("\r");
            
            // Print all completed characters including current
            for j in 0..=i {
                print!("{}", text.chars().nth(j).unwrap());
            }
            io::stdout().flush().unwrap();
        }
        
        thread::sleep(Duration::from_millis(500));
        
        // Optional: Clear the text with a fade effect
        for char in text.chars().rev() {
            print!("\r{}", " ".repeat(text.len()));
            print!("\r");
            print!("{}", &text[..text.find(char).unwrap()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    // Final display
    println!("\r{}", text);
}