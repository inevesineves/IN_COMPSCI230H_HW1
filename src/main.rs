fn main() {
    println!("Server, start!");
    
    let mut u_input = String::from("Up Down");
    // creates a String, stores it in the heap and assigns the pointer to u_input
    // unlike a string literal, we can change this one!
    
    u_input.push_str(" left Right");
    // since u_input is stored in the heap, not the stack
    // we can change its size, therefore changing the content.
    
    user_movement(u_input);
    
    // println!("This user wants to move {u_input}");
    // Uh oh! Can't use this line now. 
    // We "moved" the reference to another variable. This variable is now invalid.
    // After all, this variable was cleared after we left the method's scope.
    // Comment it out to compile the code properly.
    
    let u2_input = String::from("Up Up Down Down");
    // string comes into the scope
    
    let u3_input = special_move(u2_input);
    // unlike the above method, this method returns the string value.
    // this prevents drop from being called on it, and ownership is given to u3_input.
    
    println!("user wishes to move: {u3_input}");
    
    // println!("user wishes to move: {u2_input}");
    // u2_input is still invalid, however.
    // comment the above line of code out to compile properly.
    
        
    let status = 404;
    status_code(status);
    // we only "copied" the variable to the method,
    // so the original reference was not changed even
    // after passing status in as a variable.
    
    println!("Server status: still {status} over here!");
    // We can still use status just fine.
}

fn status_code (code: i32) {
    println!("Server status: {code}");
} 
// code goes out of scope. Nothing special happens.
// Copy type variables are stored in the stack, so they can be easily "copied"
// Other variables that are NOT stored in the stack, like String, must be handled differently.

fn user_movement (user_input: String) {
    println!("User wishes to move {user_input}");
} 
// User_input goes out of scope. 
// drop is called on all variables in the scope that are stored in heap.
// The space in the heap is freed, so user_input is no longer valid.

fn special_move(user_input: String) -> String {
    user_input 
    // since it doesn't end in a ; it will return the expression.
    // this returns ownership to the scope that *called* the method. 
    // therefore drop is not called on this String.
}
