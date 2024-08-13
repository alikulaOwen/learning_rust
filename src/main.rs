struct A;
struct S(A);

struct SGenerics<T>(T);

fn reg_fn(_s: S){}

fn gen_spec_t(_s: SGenerics<A>){}
 

 fn gen_spec_i32(_s: SGenerics<i32>){}


 fn generic<T>(_s: SGenerics<T>){}

 struct Val {
        val: f64
 }

 impl Val {
     fn value(&self) -> &f64 {
         &self.val
     }
     fn sum (&self, other: &Val) -> f64 {
         self.val + other.val
     }

     fn multiply(&self, other: &Val) -> f64 {
         self.val * other.val
     }
 }


 fn main(){
     reg_fn(S(A));
     gen_spec_t(SGenerics(A));
     gen_spec_i32(SGenerics(6));
     generic(SGenerics(A));
     generic(SGenerics(6));

     generic::<char>(SGenerics('a'));

        let a = Val {val: 3.0};
        let b = Val {val: 4.0};

        let sum_2 = a.sum(&b);

        let mul_2 = a.multiply(&b);

        println!("Sum: {}", sum_2);
        println!("Multiply: {}", mul_2);

        println!("Value: {}", a.value());
        print!("Value: {}", b.value());
 }

