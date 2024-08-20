struct Cardinal;

struct BlueJay;

struct Turkey;

trait Red {
    
}
trait Blue{}

impl Red for Cardinal {
    
}

impl Blue for BlueJay{

}

fn red <T: Red>(_: &T) -> &'static str{"red"}
fn blue<T: Blue>(_: & T) -> &'static str{"blue"}

fn main(){
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;


    // red wont on blue_jay because it is bound to blue

    println!("Cardinal is {}", red(&cardinal));
    println!("A blue jay is  {}", blue(&blue_jay));
}
