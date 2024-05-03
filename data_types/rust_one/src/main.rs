fn main() {

  

    //data types
 

    //floats
    //intergers
    //strings
    //characters

    // let number1: i8=10;
    // let number2: i16=11;
    // let number3: f32 = 111.11;

    // let characters : char = 't' ;

    //1 -> str
    // 2-> String

    // cont my_age :&str = "Welcome to programming";

    // const text2 : String = String::from(s: "Hello there ....");

    //text2 = String::newfrom
    
    //let numbers2 : f32 = 98.9; 

    //function called sum takes in two parameters
    //println !("Hello....{}",numbers2); 

    let age : i32 = {
        2*44
    
    };
    println!("hey age {}",age);

    let num_2 : i32 = 27;
    let num_3 : i32 = 23;
    sum(num_2, num_3);

    println!("2)Subtaction  of 1 and 2 is.... {}",sub(num_2, num_3));

}


fn sum(number_1: i32, number_2: i32) {
    let number_3  = number_1+number_2;
    println!("1)Sum of 1 and 2 is... {}", number_3);

}

fn sub(number_5: i32, number_4: i32)->i32{
    let number_6  = number_5 - number_4;
    //println!("2)Subtaction  of 1 and 2 is.... {}", number_6);
    number_6
}
