
fn calc_amount(number: f32, rate: f32) -> f32 {
 number * rate
}
fn main() {
 say_something();
 let amount1 = calc_amount(2.0, 5.5);
 let amount2 = calc_amount(amount1, 2.0);
//  println!("{}", amount2);
}
fn say_something() {
 println!("hello");
}