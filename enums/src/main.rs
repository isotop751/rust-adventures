


fn main() {

    #[derive(Debug)]
    enum IPAddress
    {
        IPv4,
        IPv6,
    }

    struct IPAddr
    {
        kind: IPAddress,
        value: String,
    }

    let home = IPAddr
    {
        kind: IPAddress::IPv4,
        value: String::from("127.0.0.1"),
    };

    let loopback = IPAddr
    {
        kind: IPAddress::IPv6,
        value: String::from("::1"),
    };

    println!("{:?}: {}", home.kind, home.value);
    println!("{:?}: {}", loopback.kind, loopback.value);


    enum Coin
    {
        Value(u8),
    }

    let coin = Coin::Value(30);

    let quarter = match coin
    {
        Coin::Value(25) => true,
        _ => false,
    };

    if quarter == true
    {
        println!("Yes it is a quarter");
    }
    else
    {
        println!("No it is not a quarter");
    }
}
