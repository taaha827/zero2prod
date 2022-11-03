// Importing the Tcp Listnerer Structure
use std::net::TcpListener;

use zero2prod::run;



// Using Declarative macro to technically create a main function that will run asynchrnously, as by default you can not have the main function
// Of a rust service set as async
#[actix_web::main]
/* 
Returning A Result Data type, remember that by default RUST treat lines of code as expression (i.e: Lines that perform an action and return a value, and the last line of a function is always implied as return statement)
*/ 
// Result<()> is used so that it shows that it will return OK signal without any value
// Or error if needed
async fn main() -> std::io::Result<()> {
    // Bind returns a RESULT, so if we want to cater that we use expect, which will cause the code to exit if the bind function returns an error
    // Expect is not like catch just a way to catch an error and modify the error message, it does not allow to write a function per se.
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    // ? allows to proceed only if there is no error, other wise throw the error back to who ever has called the function
    run(listener)?.await
}


#[cfg(test)]
mod tests{
    // Here we would write internal tests for private sub routines 
}