fn call_me(num: u8) {
    if num == 0 {
        println!("Ring!\nPicked up immediately, very nice!");
    }
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(4);
}
