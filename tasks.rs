


//For tasks 1 - 3, you will review a block of code and then enter expected output or
//behavior where prompted. If you are unsure of the answer, simply write 'unsure'.

/* TASK 1: MOVE ASSIGNMENT*/

//Review the code:
fn main() {
    let x = String:from("hello");
    let mut y = String:from("test");
    y = x;
}

//For the same code block, fill in the blanks in the comments. If you
// believe the output would be a panic or error, in your own words 
// describe what type of error you expect (doesn't have to be exact).
fn main() {
    let x = String:from("hello");
    println!("{}", x);
    //Enter expected output A: hello
    //Enter expected output B: hello

    let mut y = String:from("test");
    println!("{}", y); 
    //Enter expected output A: test
    //Enter expected output B: test

    y = x;
    println!("x:{}, y:{}", x, y); 
    //Enter expected output A: x:hello, y:hello
    //Enter expected output B: either print x: "panic" (x doesn't have ownership), y:hello; or error since x does not have ownership

    println!("y:{}", y); 
    //Enter expected output A: y:hello
    //Enter expected output B: y:hello
}

//do git command in terminal: git commit -m "task 1 answers"

//Now that you're done, let's take a look at the RustViz diagram.
// http://jp8.ddns.net:8000/move_assignment.html

//After reviewing the diagram, make any changes to your previous 
// answers you think are needed in the expected output B comment lines.

//do git command in terminal: git commit -m "task 1 diagram answers"









/* TASK 2: HATRA 1*/

//Review the code:
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let mut x = 5;
    let y = x;
    x = 6;
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}


//For the same code block, fill in the blanks in the comments. If you
// believe the output would be a panic or error, in your own words 
// describe what type of error you expect (doesn't have to be exact).
fn main() {
    let s = String::from("hello");
    println!("{}", s); 
    //Enter expected output A: hello
    //Enter expected output B: hello

    takes_ownership(s);
    let mut x = 5; 
    println!("{}", x); 
    //Enter expected output A: hello\ 5
    //Enter expected output B: hello\ 5

    let y = x;
    x = 6;
    println!("{}", s); 
    //Enter expected output A: panic
    //Enter expected output B: panic

    println!("{}", x + y); 
    //Enter expected output A: 11
    //Enter expected output B: 11
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

//do git command in terminal: git commit -m "task 2 answers"

//Now that you're done, let's take a look at the RustViz diagram.
// http://jp8.ddns.net:8000/hatra1.html

//After reviewing the diagram, make any changes to your previous 
// answers you think are needed in the expected output B comment lines.

//do git command in terminal: git commit -m "task 2 diagram answers"





/*TASK 3: Aligning RustViz annotations*/

//This code block has placeholders // !{ REPLACE ME WITH ANNOTATION } 
// that need to be replaced with the correct RustViz annotation.
// Review the code block below, then copy and past the annotations
// in the comments below the code into the appropriate placeholders
// in the code.

//Take a look at the diagram: http://jp8.ddns.net:8000/multiple_immutable_borrow.html

//Now see if you can annotate the code to generate the diagram:

/* --- BEGIN Variable Definitions ---
Owner x;
StaticRef y;
StaticRef z;
StaticRef s1;
StaticRef s2;
Function String::from();
Function f();
Function println!()
--- END Variable Definitions --- */
fn main() {
    let x = String::from("hello"); // !{ Move(String::from()->x) }
    let y = &x; // !{ StaticBorrow(x->y) }
    let z = &x; // !{ StaticBorrow(x->z) }
    f(y, z); /* !{ PassByStaticReference(y->f()),
        PassByStaticReference(z->f()),
        StaticDie(y->x), StaticDie(z->x) } */
} // !{ GoOutOfScope(x), GoOutOfScope(y), GoOutOfScope(z) }

fn f(s1 : &String, s2 : &String) { // !{ InitRefParam(s1), InitRefParam(s2) }
    println!("{} and {}", s1, s2); // !{ PassByStaticReference(s1->println!()), PassByStaticReference(s2->println!()) }
} // !{ GoOutOfScope(s1), GoOutOfScope(s2) }


/* Copy the annotations from this comment block and paste them where they
belong in the code above. NOTE: They are not listed in order of appearance
in the code.

-------------------------------------------------------------------------

-------------------------------------------------------------------------

-------------------------------------------------------------------------

-------------------------------------------------------------------------

-------------------------------------------------------------------------

-------------------------------------------------------------------------

-------------------------------------------------------------------------

-------------------------------------------------------------------------        

-------------------------------------------------------------------------
*/



//do git command in terminal: git commit -m "task 4 answers"


//All done! Now we have a few questions for you.
