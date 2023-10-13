fn reduce_ten_times (mut balance: i32) {
 //TODO: Create a for loop that runs 10 times, Each iteration should:
 // TODO: Reduce the balance by 1.0
 // TODO: Use an If Else to
 // Print the balance to terminal if balance is less or equal to 3,
 // Print "Balance still above 3" if the balance is more than 3
 // Break out of the loop if balance is less than 0
 for _ in 1..=10 {
    
    balance -= 1;
    if balance < 0 {
        break;
    } else if balance <=3 {
        println!("{}", balance)
    } else if balance >3 {
        println!("Balance still above 3");
    } 
 }
}
fn main() {
 // TODO: Create a variable representing the users starting balance, give it any value
 // TODO: Call the function you created and pass in the users starting balance
 let balance = 6;
 // scalar values will be passed by value by default to functions: i.e. making another copy rather than a value
 reduce_ten_times(balance);
 // printing original balance will still result in 8
 println!("User balance is still {}", balance); 
 let mut mut_balance = 6;
 reduce_ten_times_borrowed(&mut mut_balance);
}

// how to modify the actual value
// what is &mut, how would you recommend clearing up this confusion?

fn reduce_ten_times_borrowed ( balance: &mut i32) {
 for _ in 1..=10 {
    
    *balance = *balance - 1;
    if *balance < 0 {
        break;
    } else if *balance <= 3 {

        println!("{}", *balance)
    } else if *balance > 3 {
        println!("Balance still above 3");
    } 
 }
}