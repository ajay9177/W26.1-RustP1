// src/file1.rs
pub fn types() {
    //Numbers
    let x:i32=1;
    println!("{}",x);

    //Booleans
    let is_male = false;
    let is_above_18 = true;
    if is_male{
        println!("You are a male");
    }else{
        println!("You are not a male");
    }
    if is_male && is_above_18{
        println!("You are a legal male");
    }

    //Strings
    let greeting = String::from("Hello World");
    println!("{}",greeting);

    //Arrays 
    let arr:[i32;5] = [1,2,3,4,5];
    println!("{}",arr.len());

    //Vectors
    let mut xs = vec![1,2,3];
    print!("{}",xs.len());
    xs.push(4);
    print!("{}",xs.len());
}
