pub fn condloops() {
    condi();
    loops();
}

//Conditonals
pub fn condi(){
    let x = 99;
    let is_even = is_even(x);
    if is_even{
        println!("{} is even",x);
    }else{
        println!("{} is odd",x);
    }
}
pub fn is_even(x:i32)->bool {
    return  x % 2 == 0;
}

//loops
pub fn loops(){
    let str= String::from("Ajay Chelikhani");
    println!("First name {}",get_first_name(str))
}
pub fn get_first_name(str: String)-> String {
    let mut first_name = String::from("");
    for c in str.chars(){
        if c ==' '{
            break;
        }
        first_name.push(c);
    }
    return  first_name;
}

