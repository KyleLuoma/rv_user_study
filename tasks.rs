


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
    //Enter expected output A: 1
    //Enter expected output B: 2

    let mut y = String:from("test");
    println!("{}", y); 
    //Enter expected output A: 2
    //Enter expected output B: 3

    y = x;
    println!("x:{}, y:{}", x, y); 
    //Enter expected output A: 3
    //Enter expected output B: 4

    println!("y:{}", y); 
    //Enter expected output A: 4
    //Enter expected output B: 5
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
    //Enter expected output A:
    //Enter expected output B:

    takes_ownership(s);
    let mut x = 5; 
    println!("{}", x); 
    //Enter expected output A:
    //Enter expected output B:

    let y = x;
    x = 6;
    println!("{}", s); 
    //Enter expected output A:
    //Enter expected output B:

    println!("{}", x + y); 
    //Enter expected output A:
    //Enter expected output B:
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






/*TASK 3: HATRA 2*/

//Review the code:
fn main(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    assert!(compare_strings(r1, r2));

    let r3 = &mut s;
    clear_string(r3);
}


//For the same code block, fill in the blanks in the expected output A comment lines. If you
// believe the output would be a panic or error, in your own words 
// describe what type of error you expect (doesn't have to be exact).
fn main(){
    let mut s = String::from("hello");
    println!("{}", s);  
    //Enter expected output A:
    //Enter expected output B:

    let r1 = &s;
    println!("{}", r1); 
    //Enter expected output A:
    //Enter expected output B:

    let r2 = &s;
    println!("{}", r2);  
    //Enter expected output A:
    //Enter expected output B:

    s = String::from("world"); 
    // Is this valid? Why/Why not? A:
    // Is this valid? Why/Why not? B:

    assert!(compare_strings(r1, r2));

    s = String::from("world");
    // Is this valid? Why/Why not? A:
    // Is this valid? Why/Why not? B:

    let r3 = &mut s;
    println!("{}", r3); 
    //Enter expected output A:
    //Enter expected output B:

    clear_string(r3);
    println!("{}", r3); 
    //Enter expected output A:
    //Enter expected output B:
}


//do git command in terminal: git commit -m "task 3 answers"

//Now that you're done, let's take a look at the RustViz diagram.
// http://jp8.ddns.net:8000/hatra2.html

//After reviewing the diagram, make any changes to your previous 
// answers you think are needed in the expected output B comment lines.

//do git command in terminal: git commit -m "task 3 diagram answers"



/*TASK 4: Aligning RustViz annotations*/

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
    let x = String::from("hello"); // !{ REPLACE ME WITH ANNOTATION }
    let y = &x; // !{ REPLACE ME WITH ANNOTATION }
    let z = &x; // !{ REPLACE ME WITH ANNOTATION }
    f(y, z); /* !{ REPLACE ME WITH ANNOTATION } */
} // !{ REPLACE ME WITH ANNOTATION }

fn f(s1 : &String, s2 : &String) { // !{ REPLACE ME WITH ANNOTATION }
    println!("{} and {}", s1, s2); // !{ REPLACE ME WITH ANNOTATION }
} // !{ REPLACE ME WITH ANNOTATION }


/* Copy the annotations from this comment block and paste them where they
belong in the code above. NOTE: They are not listed in order of appearance
in the code.

-------------------------------------------------------------------------
StaticBorrow(x->y)
-------------------------------------------------------------------------
PassByStaticReference(s1->println!()), PassByStaticReference(s2->println!())
-------------------------------------------------------------------------
Move(String::from()->x)
-------------------------------------------------------------------------
StaticBorrow(x->z)
-------------------------------------------------------------------------
InitRefParam(s1), InitRefParam(s2)
-------------------------------------------------------------------------
GoOutOfScope(s1), GoOutOfScope(s2)
-------------------------------------------------------------------------
PassByStaticReference(y->f()),
        PassByStaticReference(z->f()),
        StaticDie(y->x), StaticDie(z->x)
-------------------------------------------------------------------------        
GoOutOfScope(x), GoOutOfScope(y), GoOutOfScope(z)
-------------------------------------------------------------------------
*/



//do git command in terminal: git commit -m "task 4 answers"


//All done! Now we have a few questions for you.
