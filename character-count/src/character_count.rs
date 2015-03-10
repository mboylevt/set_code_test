fn character_count(input: &str) {
    println!("Input: {}", input);
    let mut count: i32 = 0;
    let mut last: u8 = 0; 
    
    for c in input.bytes() {
        if last == c {
            count += 1;    
        }
        else {
            count = 0;
        }
        println!("Count: {}", count);
        last = c;
    }
}

fn main() {
    character_count("testt");
}
