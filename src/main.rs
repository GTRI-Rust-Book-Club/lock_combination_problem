struct Lock {
    state_vector : Vec<u32>,
}

fn lock_generator(args: &[u32]) -> Lock {
    let positions = args.to_vec();    
    Lock { state_vector: positions }
}

fn calculate_rotations(initial_position: i32, final_position: i32) -> u32 {
    // This is is not correct. Need to fix. (7 -> 2 should be 5, function returns 0)
    ((initial_position - final_position).abs() % 5) as u32
}

fn main() {
    println!("Hello, world!");
}
