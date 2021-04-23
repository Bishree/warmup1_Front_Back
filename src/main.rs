fn main() {
    //creat a function for the front back
    //creat new string
    let word= String::from("ahmed");
    //pars the leters
    let word_capacity = word.capacity();
    if word_capacity > 1{
        let first_car= word.chars().next().unwrap();
        let last_car= word.chars().last().unwrap();
        let word2= rem_first_and_last(&word);
        println!("{}{}{}", last_car,word2, first_car);
    }else {
        println!("{}", word);
    }


    fn rem_first_and_last(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.as_str()
    }
    // for c in word.chars() {
    //
    //     println!("{}",c);
    // }
    //find the first one
    // let s1=&word.chars()[1];
    // println!("{}", s1);
    // //find the last one
    //replace first with last

}
