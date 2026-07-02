fn main() {
    // let x = 5; //if x is not set a mut (mutable) the n there will be a compilation error becase later we try to midify it
    let mut x = 5;
    println!("x = {x}");
    x = 10;
    println!("x = {x}");

    let y = 5;
    let y = y + 1; //shadow the first variable, it's effectively creating a new varable => thus not throwing an immutability exception
    {
        let y = y * 2;
        println!("in inner scope is y= {y}")
    }
    println!("y= {y}");
    /*
    let mut spaces = "         ";
    spaces = spaces.len(); //this would not compile because you cannot mutate varaible type
    */
    // you should do this instead
    let spaces = "        ";
    println!("spaces  = \"{spaces}\"");
    let spaces = spaces.len();

    println!("spaces = {spaces}")
}
