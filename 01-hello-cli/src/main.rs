use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // arg[0] is the path
    let path = &args[0];
    
    println!("Starting: {}", path);
    if args.len() > 1 {
        let name = &args[1];
        println!("Jello, {}!", name);
    } else {
        println!("Hello, world!");
    }
}
