const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (READY_AMOUNT + 6, READY_AMOUNT);
    // let mut missiles = READY_AMOUNT + 6;
    // READY_AMOUNT = 8;


    println!("firing {} our of {} missiles", ready, missiles);


    // missiles = missiles - ready;
    println!("remaining missiles: {}", missiles - ready);
}
