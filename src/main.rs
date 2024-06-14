/*
*   A struct, or structure, is a custom data type that lets you package together and name
*   multiple related values that make up a meaningful group. If you're familiar with an object-oriented language, a struct is like an object's data attributes.
*   In this chapter, we'll compare and contrast tuples with structs to build on what you already
*   know and demonstrate when structs are a better way to group data.
*
*   We'll demonstrate how to define and instantiate structs. We'll discuss how to define associated functions,
*   epsecially the kind of associated functions called methods, to specify behavior associated with a struct type.
*   Structs and enums (This topic is discussed in chapter 6) are the building blocks for creating new types in your
*   program's domain to take full advantage of Rust's compile-time type checking.
*/

/*
   Defining and instantiating Structs
*/

/*
*   Structs are similar to tuples, discussed in chapter 3.2, in that both hold mutliple related values.
*   Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you'll
*   name each piece of data so it's clear what the values mean. Adding these names means that structs are more
*   flexible than tuples: you don't have ot rely on the order of the data to specify or access the values of an
*   instance.
*
*   To define a struct, we enter the keyword 'struct' and name the entire struct. A struct's name should
*   describe hte significance of the pieces of data being grouped together. Then, inside the curly brackets,
*   we define the names and types of the pieces of data, which we call fields. For example, below we show a struct
*   that stores information about a user account.
*/
#[warn(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/*
*   To use a struct after we've defined it, we create an instance of that struct by specifying conrete values for each
*   of the fields. We create an instacne by stating the name of the struct and then add curly brackets
*   containing key: value pairs, where the ekys are the names of the fields in the same order in which we declared
*   them in the struct. In other words, the struct definition is like a general template for the type, and instances
*   fill in that template with particular data to create values of the type. For example, we can declare a particular user
*   as shown below
*/

fn _main1() {
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

/*
*   To get a specific value from a struct, we use dot notation. For example, to access this users' email
*   address, we user 'user1.email'. If the instance is mutable, we can change a value by using the dot
*   notation and assigning into a particular field. Below shows how to change the value in the 'email'
*   field of a mutable 'User' isntance.
*/

fn _main2() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

/*
*   Note that the entire isntance must be mutable; Rust doesn't allow us to mark only certain fields as
*   mutable. As with any expression, we can construct a new instance of the struct as the last expression
*   in the function body to implicitly return that new instance.
*
*   Below shows a 'build_user' function that returns a 'User' instance with the given email
*   and username. The 'active' field gets the value 'true', and the 'sign_in_count' gets a value of '1'.
*/

#[allow(clippy::redundant_field_names)]
fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/*
*   It makes sense to name the function parameters with the same name as the struct fields, but having
*   to repeat the 'email' and 'username' field names and variables is a bit tedious. If the
*   struct had more fields, repeating each name would get even more annoying. Luckily, there's a
*   convenient shorthand!
*/

/*
    Using the Field Init Shorthand
*/

/*
*   Because the parameter names and the struct field names are exactly the same in the last function,
*   we can use the field init shorthand syntax to rewrite 'build_user' so it behaves exactly the same but
*   doesn't have the repetition of 'username' and 'email', as shown below.
*/

fn _build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

/*
*   Here, we're creating a new instance of the 'User' struct, which has a field named 'email'. We want to
*   set the 'email' fields' value in the 'email' parameter of the 'build_user' function. Because
*   the 'email' field and the 'email' parameter have the same name, we only need to write 'email'
*   rather than 'email: email'.
*/

#[allow(clippy::no_effect)]
// Shutting up the linter
fn main() {
    0;
}
