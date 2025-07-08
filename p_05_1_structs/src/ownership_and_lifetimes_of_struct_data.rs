use crate::structs;

fn structs_own_its_data() {
    let email = String::from("emaill@gmail.com");
    let username = String::from("user1234");

    // In the User struct we used the owned String type rather than the &str string slice
    let u = structs::build_user(email, username);

    // This is because we want each instance of this struct to own all of its data
    // and for the data to be valid for as long as the entire struct is valid
    // That is why the code below will fail ownership rules
    // FAIL: let u2 = structs::build_user(email, username);
}

// Structs can have non-owning reference data but there is a caveat
// To store references to data owned by something else requires the use of lifetimes
fn struct_with_referenced_data() {
    // Let's say for example we could set username and email to a string slice
    /*
    struct User {
        username: &str,
        email: &str,
    }

    let user1 = User {
        username: "someusername123",
        email: "someone@example.com",
    };
    */

    // The code as written above would not compile.

    // The struct needs to know how long a reference lasts for
    // This means lifetimes must be specified for those arguments defined in struct User
    // One possible way of doing this would be this
    /*
    struct User<'a> {
        username: &'a str,
        email: &'a str
    }
     */
}
