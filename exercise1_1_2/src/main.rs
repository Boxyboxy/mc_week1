
fn calc_amount(number: f32, rate: f32) -> f32 {
 number * rate
}
fn main() {
 say_something();
 let amount1 = calc_amount(2.0, 5.5);
 let _amount2 = calc_amount(amount1, 2.0);
 // variable naming, compiler needs to know that it will not be used
//  println!("{}", amount2);
}
fn say_something() {
 println!("hello");
}


// use cases of _

//1.  telling the compiler to infer the data type
let v: Vec<_> = ["string", "hello"];
//Note: do not confuse with T: generic type that is used to define blue prints

//2.  match _ : wildcard to catch all

//3. when the variable does not matter
//for _ in 1..=10 {  
//}

