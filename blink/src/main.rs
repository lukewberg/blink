use lib_blink::types::VarInt;

fn main() {
    println!("Hello, world!");
    let test = VarInt::from_net();
    print!("{test}");
}
