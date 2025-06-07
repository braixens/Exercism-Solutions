   // PROBLEM | given a decimal number, count how many ones would be in the binary form

pub fn egg_count(display_value: u32) -> usize {
  
  
  
  
  
  let mut total_number: u32 = 0;

    //turns input into binary, adds each digit 
    // to a vector
    let eggs: String = format!("{:b}", display_value);  
    let coop: Vec<char> = 
        eggs
        .chars()
        .collect();

    // iterated over the coop vector and
    // adds a 1 when encounters a 1
    for nest in coop {
        if nest == '1'{
        total_number += 1
    }
 }   
 // match type 
 total_number as usize
}

// prints amount of 1s in input binary
// ex: 54 in binary is 110110 | 4 ones
fn main() {
    let num_eggs = egg_count(54);
    println!("Number of eggs is: {}", num_eggs);
}
