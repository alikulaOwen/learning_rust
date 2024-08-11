mod my{
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {

    let open_box = my::OpenBox { contents: "public information" };

    println!("The open box contains: {}", open_box.contents);

    let closed_box = my::ClosedBox::new("classified information");

    // Error! `contents` is private
    //println!("The closed box contains: {}", closed_box.contents);
  
}
