use std::env;


struct Lock {
    state_vector : Vec<u32>,
}

fn lock_generator(args: &[u32]) -> Lock {
    let positions = args.to_vec();    
    Lock { state_vector: positions }
}

fn calculate_rotations(initial_position: i32, final_position: i32) -> u32 {
    // The thought here is that the sum of direct rotations + rollover rotations will always equal 10.
    // So, the abs_difference of positions gives us the direct number of rotations and 10 - abs_difference gives the
    // number of rollover rotations required, and we want the minimum of the two.
    use std::cmp;
    let abs_difference = (initial_position - final_position).abs();
    cmp::min(abs_difference, 10 - abs_difference) as u32
}

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let num_input_elems = args.len() - 1;
    if (num_input_elems % 2) != 0 
    {
        println!("Error: must have even input");
    }

    let mut current_lock_state: Vec<u32> = Vec::<u32>::new();
    let mut unlock_state: Vec<u32> = Vec::<u32>::new();

    let size_of_one_lock = num_input_elems / 2;
    for index in 1..(size_of_one_lock + 1)
    {
        current_lock_state.push(args[index].parse::<u32>().unwrap());
        //println!("{}", current_lock_state[index-1]);
    }

    for index in (size_of_one_lock + 1)..(num_input_elems + 1)
    {
        unlock_state.push(args[index].parse::<u32>().unwrap());
        //println!("{}", unlock_state[index - size_of_one_lock - 1]);
    }

}
