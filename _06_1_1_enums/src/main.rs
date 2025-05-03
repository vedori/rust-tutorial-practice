//enums give you a way of saying a value is one of a possible set of values.

// IP addresses can be IPv4 or IPv6
// enum IpAddrKind {
//     V4,
//     V6,
// }

// both of the type IpAddrKind
// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

// We only know what kind of data it is
// We dont have a way to <<store the actual data>>
// There is a way to bundle data with enums

// Naive approach
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// There is a better approach where you can put data directly into each enum variant.
// If we wanted to store V4 addresses as four u8 values but still
// have V6 addresses as one String value, we wouldn’t be able to with a struct.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// The std lib has an IP definitions
// It is similar but it first defines an
// Ipv4 struct and an Ipv6 struct
// It embeds the enums variations with its respective struct
/*
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

    You can embedd any data type into enums even structs and other enums
*/

fn main() {
    // IpAddr::V4() returns an instance of the IpAddr type
    let home = IpAddr::V4(127, 0, 0, 1);

    // IpAddr::V6() also returns an instance of the IpAddr type
    let loopback = IpAddr::V6(String::from("::1"));
}
