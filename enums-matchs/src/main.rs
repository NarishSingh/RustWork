enum IpAddrKind {
    v4,
    v6
}

//struct with an enum
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

//putting data directly into an enum
enum IpAddrEnum {
    //here, each enum can now act as a function for ctor'ing an instance of that enum
    V4(String),
    V6(String)
}

enum IdAddrEnumInt {
    V4(u8, u8, u8, u8)
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    let home2 = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrEnum::V6(String::from("::1"));

    let home3 = IdAddrEnumInt::V4(127, 0, 0, 1);
}
