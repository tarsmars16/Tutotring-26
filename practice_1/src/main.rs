use std::io;
fn main() {
    
    println!("---Welcome to Tare's Student Information Collector");

    loop {

        println!("\nWould you like to Input a Student's Information? (y/n)");
        let mut input1 = String::new();
        io::stdin().read_line(& mut input1).expect("Failed to Read String");
        let choice = input1.trim().to_lowercase();
       

       
        if choice == "y" {
            println!("\nInput the Student Information Below");
            vectors();
        }
        else if choice == "n" {

            println!("\nThank you!");
            
            break;
        }
        else { println!("\nPlease enter either y to Continue or n to Stop");
                break; }
        

   } 
}

fn vectors() {
       let mut mentees_name: Vec<String> = Vec::new();
       let mut mentees_age: Vec<u8> = Vec::new();

 loop {

    
    println!("\nWhat is the Name of The Student");
    let mut input1 = String::new();
    io::stdin().read_line(& mut input1).expect("Failed to Read String");
    let name = input1.trim();
    mentees_name.push(name.to_string());


    println!("\nWhat is the Age of the Student?");
    let mut input2 = String::new();
    io::stdin().read_line(& mut input2).expect("Failed to Read String");
    let age = input2.trim().parse().expect("Invalid Input");
    mentees_age.push(age);

    println!("\nWould you like to add another Student's Information? (y/n) ");
    let mut input3 = String::new();
    io::stdin().read_line(& mut input3).expect("Failed to Read String");
    let choice = input3.trim().to_lowercase();

     if choice == "y" {
        println!("\nInput the Information of the Next Student");
        continue;
    }
        else if choice == "n" {
           
            println!("---These are the Information of the Students you Inputed---\n");

            for i in 0..mentees_name.len() {
                println!("Student {} Name: {}", i+1, mentees_name[i]);
                println!("Student {} Age : {}\n", i+1, mentees_age[i]);
            } break;
          
          
        }

        else { println!("Please enter either y to Continue or n to Stop");
        continue; }



 
 }

}
