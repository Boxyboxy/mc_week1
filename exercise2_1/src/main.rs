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
}

