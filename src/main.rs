use rand::{thread_rng, Rng};

fn main() {
    let s = "Hello, world!".to_string();
    let l = get_length(&s);
    // println!("String has length {}.", l);
    println!("String '{}' has length {}.", s, l);

    let mut v = generate_random_array(10, -1000, 1000);
    println!("Vector = {:?}", v);

    add_seven_at_end(&mut v);
    println!("Vector after = {:?}", v);
}

fn add_seven_at_end(v : &mut Vec<i32>) {
    v.push(7);
}

fn get_length(s: &String) -> usize {
    return s.len();
}

fn generate_random_array(len: i32, min: i32, max:i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut v = Vec::new();
    for _ in 0..len {
        v.push(rng.gen_range(min, max));
    }
    return v;
}