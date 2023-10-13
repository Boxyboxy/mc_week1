// 1. Create a User struct with fields balance(f32), currency(String)
// 2. In main, create a User instance with balance of 100.00, currency of "SGD"
// 3. Using expression and match, assign balance_is_100 to true if balance equals to 100.00. Or else, return false.
// 4. Print the value of balance_is_100


struct User{
    balance: f32, 
    currency: String
}

fn main() {
    let user = User{
        balance: 100.00, 
        currency: "SGD".to_owned()
    };

    let balance_is_100 = match user.balance {
        100.0 => true,
        _ => false
    };

    println!("{}",balance_is_100);


    
    


}