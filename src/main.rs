mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

use deeeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public information",
    };

    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified information");

    // Error! `contents` is private
    //println!("The closed box contains: {}", closed_box.contents);

    other_function();

    println!("Entering block");
    {
        use crate::deeeply::nested::function;
        function();

        println!("Leaving block");
    }
}
