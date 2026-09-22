fn main() {
    println!("Server, start!");
    let u_input = String::from("Up Down Left Right");
    user_movement(u_input);
    // println!("This user wants to move {u_input}");
    // Uh oh! Can't use this line now. 
    // Comment it out to compile the code properly.
    
    let status = 404;
    status_code(status);
    println!("Server status: still {status} over here!");
}

fn user_movement (user_input: String) {
    println!("User wishes to move {user_input}");
} 
// user_input goes out of scope and drop is called.
// space in heap is freed, making the original variable invalid

fn status_code (code: i32) {
    println!("Server status: {code}");
} 
// code goes out of scope. Nothing special happens.
