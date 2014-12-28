use std::collections::HashMap;
use std::os;

fn main() {
    let args = os::args();
    match args.as_slice() {
        [_, ref input] => {
            match input.as_slice().parse() {
                Some(value) => find_collab_seq(value),
                None => println!("Can't convert {} to integer!", input)
            }
        }
        [ref name] => println!("Usage: {} [number to find collab seq]", name),
        _ => println!("Usage: collatz [number to find collab seq]"),
    }
}

fn find_collab_seq(collab_value : int) {
    let mut i : int = 1;
    let mut memo_table = HashMap::new();
    loop {
        let result = find_collatz(i, &memo_table);
        if !memo_table.contains_key(&i) {
            memo_table.insert(i, result);
        }

        if result == collab_value {
            println!("The number {} has a collab value of {}", i, collab_value);
            break;
        }

        i = i + 1;        
    }
}

fn find_collatz(n: int, memo_table: &HashMap<int, int>) -> int {
    if n == 1 {
        return 0;
    }
    else {
        match memo_table.get(&n) {
            Some(val) => { return *val; }
            _ => {
                match n % 2 {
                    0 => { 1 + find_collatz(n / 2, memo_table) }
                    _ => { 1 + find_collatz(n * 3 + 1, memo_table)}
                }
            }
        }
    }
}
