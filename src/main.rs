mod my_module {
    fn private_fn(){
        println!("This is a private function");
    }

    pub fn public_fn(){
        println!("This is a public function");
    }


    pub fn indirect_access(){
        println!("This is an indirect access");
        private_fn();
    }

    pub mod nested {
        pub fn function(){
            println!("This is a nested function");
        }
        // This function is private
        pub fn private_fn(){
            println!("This is a private nested function");
        }

        pub(in crate::my_module) fn public_fn_in_my_module(){
            println!("This is a public nested function called from my_module my_mod::nested::public_function_in_super_mod}} ")
        }

        pub(crate) fn public_fn_in_nested(){
            println!("This is a public nested function called from my_module my_mod::nested::public_function_in_nested}} ")
        }

        pub(super) fn public_fn_in_super_mod(){
            println!("This is a public nested function called from my_module my_mod::nested::public_function_in_super_mod}} ")
        }

    }
    
    pub fn call_public_fn_in_my_module(){
        println!("This is a public function called from my_module");
        nested::public_fn_in_my_module();
        nested::public_fn_in_nested();
        nested::public_fn_in_super_mod();
    }


    pub(crate) fn public_fn_in_crate(){
        println!("This is a public function called from my_module");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function(){
            println!("This is a private nested function");
        }

        #[allow(dead_code)]
        pub (crate) fn restricted_function(){
            println!("This is a restricted function");
        }
    }


}
fn function(){
    println!("This is a function");
}

pub fn public_function(){
    println!("This is a public function");
}
fn main() {

   function();
    public_function();
    my_module::public_fn();
    my_module::indirect_access();

    my_module::nested::function();
    my_module::call_public_fn_in_my_module();
    my_module::public_fn_in_crate();
    //my_module::nested::public_fn_in_my_module();

    // my_module::nested::public_fn_in_nested();

    // my_module::nested::public_fn_in_super_mod();

    // my_module::nested::private_fn(); 
   
}
