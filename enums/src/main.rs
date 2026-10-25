#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(localhost);
    route(loopback);

    // Pass another value to `describe` to see how its match arms unpack each variant.
    describe(IpAddrKind::V4(192, 168, 0, 1));
    describe(IpAddrKind::V6(String::from("2001:db8::1")));
}

fn route(ip: IpAddrKind) {
    println!("Routing request to {:?}", ip);
}

fn describe(ip: IpAddrKind) {
    // Matching lets us pull the data back out of whichever variant we received.
    match ip {
        IpAddrKind::V4(first, second, third, fourth) => {
            println!("IPv4 octets: {first}.{second}.{third}.{fourth}");
        }
        IpAddrKind::V6(address) => println!("IPv6 address: {address}"),
    }
}
