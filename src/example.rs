/*
    An Example Program Using Structs
*/

/*
*   To understand when we might want to use structs, let's write a program that calculates
*   the area of a rectangle. We'll start by using single variables, and then refactor the
*   program until we're using structs instead.
*
*   Instead of making a new binary projects, I made this 'example.rs' file.
*
*   This rectangles pogram will take the width and height of a rectangle specified in pixels
*   and calculate the area of the rectange.
*/

use std::{char::REPLACEMENT_CHARACTER, thread::sleep};

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

/*
*   This code succeeds in figuring out the area of the rectangle by calling the 'area'
*   function with each dimension, but we can do more to make this code clear and readable.
*
*   The issue with the above code is evident in the signature of 'area'
*/

// fn area(width: u32, height: u32) -> u32 {

/*
*   The 'area' function is supposed to calculate the area of one rectangle, but the function
*   we wrote has two parameters, and it's not clear anywhere in our program that the parameters
*   are related. It would be more readable and more manageable to group width and height together.
*
*   We've already discussed one way we might do that in "The Tuple Type" section of Chapter 3.
*/

/*
    Refactoring with Tuples
*/

/*
*   The below code shows another version of our program that uses tuples.
*/

fn main1() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

/*
*   In one way, this program is better. Tuples let us add a bit of structure, and we're now
*   passing just one argument. But in another way, this version is less clear: tuples don't
*   name their elements, so we have to index into the parts of the tuple, making our calculation
*   less obvious.
*
*   Mixing up the width and heigth wouldn't matter for the area calculation, but if we want to
*   draw the rectangle on the screen, it would matter! We would have to keep in mind that
*   'width' is the tuple index '0' and height is the tuple index '1'. This would be even harder
*   for someone else to figure out and keep in mind if they were to use our code. Because we
*   haven't conveyed the meaning of our data in our code, it's now easier to introduce errors.
*/

/*
    Refactoring with Structs: Adding More Meaning
*/

/*
*   We use structs to add meaning by labeling the data. We can transform the tuple we're using into
*   a struct with a name for the whole as well as names for the parts, as shown below.
*/

struct Rectangle {
    width: u32,
    height: u32,
}

fn main2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    )
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

/*
*   Here we've defined a struct and named it 'Rectangle'. Inside the curly brackets, we
*   defined the fields as 'width' and 'height', both of which have type 'u32'. Then, in 'main',
*   we created a particular instance of 'Rectangle' that has a width of '30' and a height of '50'.
*
*   Our 'area' function is now defined with one parameter, which we've named 'rect', whose type
*   is an immutable borrow of a struct 'Rectangle' instance. As mentioned in Chapter 4, we want to
*   borrow the struct rather than take ownership of it. This way, 'main' retains ownership and can
*   continue using 'rect1', which is the reason we use the '&' in the function signature and where
*   we call the function.
*
*   The 'area' function accesses the 'width' and 'height' fields of the 'Rectangle' instance(note that -
*   - accessing fields of a borrowed instance does not move the field values, which is why you often see borrows -
*   of structs). Our function signature for 'area' now says exactly what we mean: calculate the area of
*   'Rectangle', using its 'width' and 'height' fields. This conveys that the width and height are related
*   to each other, and it gives descriptive names to the values rather than using the tuple index values
*   of '0' and '1'. This is a win for clarity.
*/

/*
    Adding Useful Functionality with Derived Traits
*/

/*
*   It'd be useful to be able to print an instance of 'Rectangle' while we're debugging our
*   program and see the values for all its fields. The below program tries using the 'println!'
*   macro as we have used in previous chapters. This won't work, however.
*/

struct Rectangle2 {
    width: u32,
    height: u32,
}

fn main3() {
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}

/* When we compile this code, we get an error with the core message: */
/* error:[E0277]: 'Rectangle2' doesn't implement 'std::fmt::Display' */

/*
*   The 'println!' macro can do many kinds of formatting, and by default, the curly brackets
*   tell 'println!' to use formatting known as 'Display': output intended for direct end user
*   consumption. The primitive types we've seen so far imlement 'Display' by default because
*   there's only one way you'd want to show a '1' or any other primitive type to a user. But with
*   structs, the way 'println!' should format the output is less clear because there are more display
*   possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all of
*   the fields be shown? Due to this ambiguity, Rust doesn't try to guess what we want, and structs
*   don't have a provided implementation of 'Display' to use with 'println!' and the '{}' placeholder.
*
*   If we continue to read the errors, there is a helpful note:
*/

/* = help: the trait 'std::format::Display' is not implemented for 'Rectangle2" */
/* = help: in format strings you may be able to ue '{:?}' (or {:#?} for pretty-print) instead */

/*
*   Let's try it! The 'println!' macro call will now look like
/* println!("rect1 is {:?}", rect1); */. Putting the specifier ':?' inside the curly brackets
*   tells 'println!' we want to use an output called 'Debug'. The 'Debug' trait enables use to print
*   our struct in a way that is useful for developers so we can see its value while we're debugging
*   our code.
*
*   You'll get an error again, this time for not using #[derive(debug)] for the 'Rectangle2' struct.
*
*   Rust does include functionality to print out debugging information, but we have to explicitly opt
*   in to make that functionality available for our struct. To do that, we add the outer attribute
*   '#[derive(Debug)]' just before the struct definition, as shown below.
*/

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

fn main4() {
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

/*
*   Now when we run the program, we don't get any errors, and we'll see the following
*   output:
*/

/* rect1 is Rectangle { width: 30, height: 50 } */

/*
*   Nice! It's not the prettiest output, but it shows the values of all the fields for this
*   instance, which would definitely help during debugging. When we have larger structs, it's
*   useful to have output that's a bit easier to read; in those cases, we can use '{:#?}'
*   instead of '{:?}' in the 'println!' string. In this example, using the '{:#?}' style will
*   output the following:

    rect1 is Rectangle {
        width: 30,
        height: 50,
    }
*
*/

/*
*   Another way to print out a value using the 'Debug' format is to use the 'dbg!' macro,
*   which takes ownership of an expression (as opposed to 'println!', which takes a reference),
*   prints the file and line number of where that 'dbg!' macro call occurs in your code along
*   with the resultant value of that expression, and returns ownership of the value.

NOTE: 'dbg!' prints to the STDERR not STDOUT!!
*/

/*
*   Here's an example where we're interested in the value that gets assigned to the 'width' field,
*   as well as the value of the whole struct in 'rect1':
*/

#[derive(Debug)]
struct Rectangle3 {
    width: u32,
    height: u32,
}

fn main5() {
    let scale = 2;
    let rect1 = Rectangle3 {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

/*
*   We can put 'dbg!' around the expression '30 * scale' and, because 'dbg!' returns ownership
*   of the expression's value, the 'width' field will get the same value as if we didn't have
*   the 'dbg!' call there. We don't want 'dbg!' to take ownership of 'rect1', so we use a
*   reference to 'rect1' in the next call. Here's what the output of this example looks like:

[example.rs:246:16] 30 * scale = 60
[example.rs:250:5] &rect1 = Rectangle3 {
    width: 60,
    height: 50,
}

*/

/*
*   We can see the first bit of output came from src/example.rs line 246 where we're debugging
*   the expression '30 * scale', and its resultant value is '60' (the 'Debug' formatting impemented -
*   - for integers is to print only their value). The 'dbg!' call on line 250 of src/example.rs
*   outputs the value of '&rect1', which is the 'Rectangle3' type. The 'dbg!' macro can be
*   really helpful when you're trying to figure out what your code is doing!
*
*   In addition to the 'Debug' trait, Rust has provided a number of traits for us to use with
*   the 'derive' attribute that can add useful behavior to our custom types. Those traits and
*   their behaviors are listed in the books Appendix[C]. We'll cover how to implement these traits
*   with custom behavior as well as how to create you own traits in Chapter 10. There are also many
*   attributes other than 'derive'; for more information see the "Attributes" section of the Rust Reference
*   book.
*
*   Our 'area' function is very specific: it only computes the area of rectangles. It would be
*   helpful to tie this behavior more closely to our 'Rectangle' struct because it won't
*   work with any other type. Let's look at how we continue to refactor this code by turning the
*   'area' function into an 'area' method defined on our 'Rectangle' type.
*/

/*
    Method Syntax
*/

/*
*   Methods are similar to functions: we declare them with the 'fn' keyword and a name,
*   they can have parameters and a return value, and they contain some code that's run when
*   the method is called from somewhere else. Unlike functions, methods are defined within
*   the context of a struct (or an enum or a trait object, which is covered in Chapter 6 & 17),
*   and their first parameter is always 'self', which represents the instance of the struct the
*   method is being called on.
*/

/*
    Defining Methods
*/

/*
*   Let's change the 'area' function that has a 'Rectangle' instance as a parameter and instead
*   make an 'area' method defined on the 'Rectangle' struct, as shown below:
*/

#[derive(Debug)]
struct Rectangle4 {
    width: u32,
    height: u32,
}

impl Rectangle4 {
    #[derive(default)]
    fn default(&self) -> Self {
        Self {
            width: 30,
            height: 50,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main6() {
    let rect1 = Rectangle4 { width, height };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

/*
*   To define the function within the context of 'Rectangle', we start an 'impl' (implementation) block
*   for 'Rectangle'. Everything within this 'impl' block will be associated with the 'Rectangle' type.
*   Then we move the 'area' function within the 'impl' curly brackets and change the first (and -
*   - in this case, only) parameter to be 'self' in the signature and everywhere within the body. In 'main'
*   where we called the 'area' function and passed 'rect1' as an argument, we can instead use method syntax
*   to call the 'area' method on our 'Rectangle' instance. The method syntax goes after an instance: we add
*   a dot followe by the method name, parenthesis, and any arguments
*
*   In the signature for 'area', we use '&self' instead of 'rect: &Rectangle4'. The '&self' is
*   actually short for 'self: &Self'. Within an 'impl' block, the type 'Self' is an alias for the type
*   that the 'impl' block is for. Methods must have a parameter named 'self' of type 'Self' for their
*   first parameter, so Rust lets you abbreviate this with only the name 'self' in the first parameter spot.
*   Note that we still need to use the '&' in front of the 'self' shorthand to indicate that this method
*   borrows the 'Self' instance, just as we did in 'rect: &Rectangle'. Methods can take ownership of 'self',
*   borrow 'self' immutably, as we've done here, or borrow 'self' mutably, just as they can any other parameter.
*
*   We chose '&self' here for the same reason we used '&Rectangle' in the function version: we don't want
*   to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to
*   change the instance that we've called the method on as part of what the method does, we'd use
*   '&mut self' as the first parameter. Having a method that takes ownership of the instance by using just 'self'
*   as the first parameter is rare; this technique is usually used when the method transforms 'self' into
*   something else and you want to prevent the caller from using the original instance after the transformation.
*
*   The main reason for using methods instead of functions, in addition to providing method syntax and not
*   having to repeat the type of 'self' in every method's signature, is for organization. We've put all the things
*   we can do with an instance of a type in one 'impl' block rather than making future users of our code search
*   for capabilities of 'Rectangle' in various places in the library we provide.
*
*   Note that we can choose to give a method the same name as one of the struct's fields. For example,
*   we can define a method on 'Rectangle' that is also named 'width':
*/
struct Rectangle5 {
    width: u32,
    height: u32,
}

impl Rectangle5 {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main7() {
    let rect1 = Rectangle5 {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

/*
*   Here, we're choosing to make the 'width' method return 'true' if the value in the instance's 'width' field
*   is greater than '0' and 'false' if the value is '0': we can use a field within a method of the same name for
*   any purpose. In 'main7', when we follow 'rect1.width' with parenteses, Rust knows we mean the
*   method 'width'. When we don't use parentheses, Rust knows we mean the field 'width'.
*
*   Often, but not always, when we give a method the same name as a field we want it to only return the value
*   in the field and do nothing else. Methods like this are called getters, and Rust does not implement them
*   automatically for struct fields as some other languages do. Getters are useful because you can make the field
*   private but the method public, and thus enable read-only access to that field as part of the type's
*   API. We will discuss what public and private are and how to designate a field or method as public or private in
*   Chapter 7.
*/

/*
    Methods with More Parameters
*/

/*
*   Let's practice using methods by implementing a second method on the 'Rectangle' struct. This time we want
*   an instance of 'Rectangle' to take another instance of 'Rectangle' and return 'true' if the second
*   'Rectangle' can fit completely within 'self' (the first 'Rectangle); otherwise, it should return 'false'.
*   That is, once we've defined the 'can_hold' method, we want to be able to write the program show below:
*/

struct Rectangle6 {
    width: u32,
    height: u32,
}

fn main8() {
    let rect1 = Rectangle6 {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle6 {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle6 {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/*
*   The expected output would look something like the following because both dimensions of 'rect2' are smaller than the
*   dimenisions of 'rect1', but 'rect3' is wider than 'rect1':

    Can rect1 hold rect2? true
    Can rect1 hold rect3? false
*/

/*
*   We know we want to define a method, so it will be within the 'impl Rectangle6' block. The method name
*   will be 'can_hold', and it will take an immutable borrow of another 'Rectangle6' as a parameter. We can
*   tell what the type of the parameter will be by looking at the code that calls the method:
*   'rect1.can_hold(&rect2)' passes in '&rect2', which is an immutable borrow to rect2, an instance of 'Rectangle6'.
*   This makes sense because we only need to read 'rect2' (rather than write, which would mean we'd need a -
*   - mutable borrow), and we want 'main' to retain ownership of 'rect2' so we can use it again after calling the
*   'can_hold' method. The return value fo 'can_hold' will be a boolean, and the implementation will check whether
*   the width and height of 'self' are greater than the width and height of the other 'Rectangle6', respectively.
*   Let's add the new 'can_hold' method to the 'impl' block shown below:
*/

impl Rectangle6 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle6) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
*   When we run this code with the 'main' function in the code above it, we'll get our desired output. Methods can take
*   multiple parameters that we add to the signature after the 'self' parameter, and those parameters work just like
*   parameters in functions.
*/

/*
    Associated Functions
*/

/*
*   All functions defined with an 'impl' block are called associated functions because they're associated with
*   the type named after the 'impl'. We can define assocaited functions as functions that don't have 'self' as their
*   first parameter (and thus are not methods) because they don't need an instance of the type to work with. We've already
*   used one function like this: the 'String::from' function that's defined on the 'String' type.
*
*   Associate functions that aren't methods are often used for constructors that will return a new instance of the struct.
*   These are often called 'new', but 'new' isn't a special name and isn't built into the language. For example,
*   we could choose to provide an associated function named 'square' that would have one dimension parameter and use that
*   as both width and height, thus making it easier to create a square 'Rectangle' rather than having to specify
*   the same value twice:
*/

impl Rectangle {
    fn sqaure(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/*
*   The 'self' keywords in the return type and in the body of the function are aliases for the type that appears after the
*   'impl' keyword, which in this case is 'Rectangle'.
*
*   To call this associated function, we use the '::' syntax with the struct name; 'let sq = Rectangle::square(3);' is an example.
*   This function is namespaced by the struct: the '::' syntax is used for both associated functions and namespaces created
*   by modules. Modules are discussed in Chapter 7.
*/

/*
    Mutliple impl Blocks
*/

/*
*   Each struct is allowed to have mutliple 'impl' blocks. For example, the previous code is equivalent to the code shown below, which
*   has each method in its own 'impl' block.
*/

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
*   There's no reason to separate these methods into multiple 'impl' blocks here, but this is valid syntax.
*   We'll see a case in which mutliple 'impl' blocks are useful in Chapter 10, where we discuss generic types and
*   traits.
*/

/*
    Method Calls are Syntactic Sugar for Function Calls
*/

/*
*   Using the concepts we've discussed so far, we can now see how method calls are syntactic sugar for function
*   calls. For example, let's say we have a rectangle struct with an 'area' method and a 'set_width' method:
*/

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

/*
*   And let's say we have a rectangle 'r'. Then the method calls 'r.area()' and 'r.set_width(2)' are equivalent
*   to this:
*/

fn example() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
}

/*
*   The method call 'r.area()' becomes 'Rectangle::area(&r)'. The function name is the associated function
*   'Rectangle::area'. The function argument is the '&self' parameter. Rust automatically inserts the borrowing operator '&'.
*
*   NOTE: If you are familiar with C or C++, you are used to two different syntaxes for method calls: 'r.area()' and
*   'r->area()'. Rust does not have an equivalent to the arrow operator '->'. Rust will automatically reference and dereference
*   the method receiver when you use the dot operator.
*
*   The method call 'r.set_width(2)' similarly becomes 'Rectangle::set_width(&mut r, 2)'. This method expects
*   '&mut self', so the first argument is a mutable borrow '&mut r'. The second argument is exactly the same, the number 2.
*
*   As described in Chapter 4.3 "Dereferencing a Pointer Accessses Its Data", Rust will insert as many references and derefences
*   as needed to make the types match up for the 'self' parameter. For example, here are two equivalent calls to 'area'
*   for a mutable reference to a boxed rectangle:
*/

fn example_again() {
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });

    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
}

/*
*   Rust will add two deferences (once for the mutable reference, once for the box) and then one immutable borrow
*   because 'area' expects '&Rectangle'. Note that this is also a situation where a mutable reference is "downgraded" into
*   a shared reference, like discussed in Chapter 4.2. Conversely, you would not be allowed to call 'set_width' on a value type
*   '&Rectangle' or '&Box<Rectangle>'.
*/

/*
    Methods and Ownership
*/

/*
*   Like discussed in Chapter 4.2 "References and Borrowing", methods must be called on structs that have the
*   necessary permissions. As a running example, we will use these three methods that take '&self', '&mut self', and
*   'self', respectively.
*/

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

/*
    Reads and Writes with &self and &mut self
*/

/*
*   If we make an owned rectangle with 'let rect = Rectangle { ... }', then 'rect' has R and O permissions.
*   With those permissions, it is permissable to call the 'area' and 'max' methods.
*/

fn another_example() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    println!("{}", rect.area()); //Rect has R (read) permissions

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);
}

/*
*   However, if we try to call 'set_width', we are missing the W permission:
*/

fn another_example2() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    rect.set_width(0); // rect has R (read), but not W (write) permissions
                       // Therefore this is rejected with:
                       // error[E0596]: cannot borrow `rect` as mutable, as it is not declared as mutable
}

/*
*   We will get a similar error if we try to call 'set_width' on an immutable reference to a 'Rectangle', even if
*   the underlying rectangle is mutable:
*/

fn another_example3() {
    // Added the mut keyword to the let-binding
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    rect.set_width(1); // Now rect. has the R and W permissions as expected.

    let rect_ref = &rect;

    rect_ref.set_width(2); // This is still not okay to perform after the immutable borrow
}

/*
    Moves with self
*/

/*
*   Calling a method that expects 'self' will move the input struct (unless the struct implements 'copy'). For example,
*   we cannot use a 'Rectangle' after passing it to 'max':
*/

fn another_example4() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);

    println!("{}", rect.area()); // Missing Read permission due to the immutable copy of self
}

/*
*   Once we call 'rect.max(..)', we move 'rect' and so lose all permissions on it. Trying to compile this program
*   would give us the following error:
*
*   error[E0382]: borrow of moved value: 'rect'
*/

/*
*   A similar situation arises if we try to call a 'self' method on a reference. For instance, say we tried to make
*   a method 'set_to_max' that assigns 'self' to the output of 'self.max(..)':
*/

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        self = self.max(other); // Missing O (own) permission
    }
}

/*
*   Then we can see that 'self' is missing O permissions in the operation 'self.max(..)'. Rust therefore rejects this program
*   with the following error:
*
*   error[E0507]: cannot move out of '*self' which is behind a mutable reference
*/

/*
*   This is the same kind of error discussed in Chapter 4.3 "Copying vs. Moving Out of a Collection".
*/

/*
    Good Moves and Bad Moves
*/

/*
*   You might wonder: why does it matter if we move out of '*self'? In fact, for the case of 'Rectangle', it
*   actually is safe to move out of '*self', even though Rust doesn't let you do it. For example, if we simulate
*   a program that calls the rejected 'set_to_max', you can see how nothing unsafe occurs:
*/

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

fn example_main() {
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectanlge {
        width: 1,
        height: 0,
    };
    rect.set_to_max(other_rect);
}

/*
*   The reason it's safe to move out of '*self' is because 'Rectangle' does not own any heap data. In fact, we can actually
*   get Rust to compile 'set_to_max" by simply adding '#[derive(Copy, Clone)] to the definition of 'Rectangle':
*/

#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

/*
*   Notice that unlike before, 'self.max(other)' no longer requires the O permission on '*self' or 'other'.
*   Remember that 'self.max(other)' desugars to 'Rectangle::max(*self, other). The dereference '*self' does not require
*   ownership over '*self' if 'Rectangle' is copyable.
*
*   You might wonder: why doesn't Rust automatically derive 'Copy' for 'Rectangle'? Rust does not auto-derive 'Copy'
*   for stability across API changes. Imagine that the author of the 'Rectangle' type decided to add a 'name: String' field.
*   Then all client code that relies on 'Rectangle' being 'Copy' would suddenly get rejected by the compiler. To avoid
*   that issue, APi authors must explicitly add '#[derive(Copy)]' to indicate that they expect their struct to always be 'Copy'.
*
*   To better understand the issue, let's run a simulation. Say we added 'name: String' to 'Rectangle'. What would
*   happen if Rust allowed 'set_to_max' to compile?
*/

struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h,
            name: String::from("max"),
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        drop(*self); // This is usually implicit,
                     // but added here for clarity.
        *self = max;
    }
}

fn main() {
    let mut r1 = Rectangle {
        width: 9,
        height: 9,
        name: String::from("r1"),
    };
    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2"),
    };
    r1.set_to_max(r2);
}

/*
*   In this program, we call 'set_to_max' with two rectangles 'r1' and 'r2'. 'self' is a mutable reference to
*   'r1' and 'other' is a move of 'r2'. After calling 'self.max(other)', the 'max' method consumes ownership of
*   both rectangles. When 'max' returns, Rust deallocates both strings "r1" and "r2" in the heap. Notice the problem:
*   at the location L2, '*self' is supposed to be readable and writable. However, '(*self).name' (actually 'r1.name')
*   has been deallocated.
*
*   Therefore when we do '*self = max', we encounter undefined behavior. When we overwrite '*self', Rust will
*   implicitly drop the data previously in '*self'. To make that behavior explicit, we have added 'drop(*self)'.
*   After calling 'drop(*self)', Rust attempts to free '(*self).name' a second time. That action is a double-free, which
*   is undefined behavior.
*
*   So remember: when you see an error like "cannot move out of '*self'", that's usually because you're trying to
*   call a 'self' method on a reference like '&self' or '&mut self'. Rust is protecting you from a double-free.
*/

/*
    Summary
*/

/*
*   Structs let you create custom types that are meaningful for your domain. By using structs, you can keep
*   associated pieces of data connected to each other and name each piece to make your code clear. In 'impl'
*   blocks, you can define functions that are associated with your type, and methods are a kind of associated
*   function that let you specify the behavior that instances of your structs have.
*
*   But structs aren't the only way you can create custom types: Next is Enums!
*/
