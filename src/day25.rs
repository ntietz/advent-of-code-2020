pub fn run() {
    let (card_pubkey, door_pubkey) = input(false);

    let card_loops = crack_loop_size(7, card_pubkey);
    let encrption_key = transform(door_pubkey, card_loops);

    println!("day25.part1.solution = {}", encrption_key);
}

fn input(example: bool) -> (u64, u64) {
    if example {
        (5764801, 17807724)
    } else {
        (13233401, 6552760)
    }
}

fn transform(input: u64, loops: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loops {
        value = (value * input) % 20201227;
    }
    value
}

fn crack_loop_size(input: u64, output: u64) -> u64 {
    let mut num_loops = 0;
    let mut value = 1;

    while value != output {
        value = (value * input) % 20201227;
        num_loops += 1;
    }

    num_loops
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cracks_keys() {
        assert_eq!(crack_loop_size(7, 5764801), 8);
    }
}
