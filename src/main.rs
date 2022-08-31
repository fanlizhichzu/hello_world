fn main() {
    println!("Hello, world!");
    hello();
}

fn hello() {
    let chinese = "你好，世界";
    let english = "helo,world";
    let regions = ["asia", "south", chinese, english];
    for region in regions {
        println!("{}", &region);
    }
}
