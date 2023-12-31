struct User {
 // TODO: create 1 field called name (String)
 // TODO: create 1 field called balance (Tuple) that contains the balance (f32) and currency (String)
 name: String, 
 balance: (f32, String)
}
impl User {
 // TODO: Create 1 method for User struct called print_name, that prints the name

 fn shout(phrase: String){ // associated function (in C speak static function)
    println!("{}", phrase);
 }

 fn print_name(&self){ // Associated function - method: a subset of associated function
    println!("{}",self.name);
 }
 
}
fn main() {
 // TODO: create a variable user of type User and provide values into its fields.
 // TODO: call print_name on variable user. Then, print the individual values within the user.balance tuple.
    let user = User {
        name: "Alex".to_owned(),
        balance: (10000.01, "SGD".to_owned())
    };
    user.print_name(); // the same as User::print_name(&user_record);
    println!("balance of {} {}", user.balance.0, user.balance.1);
    User::shout("whatever!".to_owned());
}