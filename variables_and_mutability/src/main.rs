fn main() {
    // let x = 5; //if x is not set a mut (mutable) the n there will be a compilation error becase later we try to midify it
    let mut x = 5
    println!("x = {x}");
    x = 10;
    println!("x = {x}");

    let y = 6;
    let y = y+1;
    {
        let y = y*2;
        prinln!("y= {y}")
    }
    println!("y={y}")
}
