use std::time;
mod day_1;

fn main() {
    let start = time::Instant::now();
    
    day_1::run();
    println!("\n<------------->\n Execution took {:?}", start.elapsed())
}