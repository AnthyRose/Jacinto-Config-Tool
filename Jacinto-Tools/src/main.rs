use std::path::Path;
use std::io;

fn Get_Inis(){
    
}

fn main() {
    if Path::new("simple.txt").exists(){
        Get_Inis();
    };
    // If program is not there.
    println!("Executable is not in the same folder as Gears of War, press enter to exit program.");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Read line failed.");
}
