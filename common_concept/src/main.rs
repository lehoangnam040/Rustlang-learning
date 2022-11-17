const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn less3_1() {
    let mut x = 5;
    println!("the value of x {x}");
    x = 6;
    println!("value changed to {x}");

    println!("const is {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;

    {
        let y = y *2;
        println!("value inner scope {y}");
    }

    println!("value of y {y}");
}

fn main() {
    less3_1();
}
