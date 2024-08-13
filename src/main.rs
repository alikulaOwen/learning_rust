
struct A;

struct Single(A);

struct SingleGen<T>(T);

fn main() {
    println!("Hello, world!");


    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // uses A
    let _i32 = SingleGen(6); // uses i32
    let _char = SingleGen('a'); // uses char

}

