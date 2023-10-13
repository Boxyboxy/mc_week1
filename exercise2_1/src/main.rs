fn main() {
    let user_vec: Vec<String> = vec![
        "john".to_owned(),
        "mary".to_owned(),
        "simon".to_owned(),
        "john".to_owned(),
        "kelly".to_owned(),
        "harry".to_owned(),
        ];
    let mut counter = 0;
    // TODO: for loop to count number of john values
    for i in &user_vec {
        if i.to_owned() == "john".to_owned() {
            counter+=1;
        }
    }
    // :? : usually used to print out a vector
    // This is a formatter
    println!("{} johns in {:?}", counter, user_vec);



    println!("++++++++++++++++++++++++++++");
    let v = vec![1, 2, 3, 4, 5];

    println!("4th Element is {:?}", v.get(4)); // prints the type of data, :? another example of a formatter
    println!("4th Element is {}", v.get(4).unwrap_or(&0)); // prints 4





}



