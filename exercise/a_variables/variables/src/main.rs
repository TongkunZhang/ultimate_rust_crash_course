
const STARTING_MISSILES: i32 = 8;
const READY_MISSILES: i32 = 2;

fn main() {
    let unused_variable = 3;
    let (missiles, ready) = (STARTING_MISSILES, READY_MISSILES);
    println!("Firing {} of my {} missiles...", ready, missiles);

    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);

}
