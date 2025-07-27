// There are situations where something should never produce an error
// even if the compiler is unsure
// This is often the case with hard-coded values

use std::net::IpAddr;

// A hard coded valid IP address shouldn't fail
// but if it was user supplied then it is possible it could fail
// The compiler isn't this robust so for either cases you still perform error handling
fn hard_coded_value() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        // This is a good error message because if this function changes
        // to accept user data it's important to know that it originally
        // assumed a hardcoded value and to make changes accordingly
        .expect("Hardcoded IP address should be valid.");
}