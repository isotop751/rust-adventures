fn main() {

    #[derive(Debug)]
    struct Rectangle
    {
        width: u64,
        height: u64,
    }

    impl Rectangle
    {
        // Method
        fn area(&self) -> u64
        {
            return self.width * self.height;
        }

        // Method
        fn perimeter(&self) -> u64
        {
            return (self.width + self.height) * 2
        }

        // static method
        fn new_square(width: u64) -> Rectangle
        {
            let square = Rectangle
            {
                width: width,
                height: width,
            };

            return square;
        }
    }


    // usage
    let yuyu: Rectangle = Rectangle
    {
        width: 30,
        height: 40,
    };

    let yoyo = Rectangle::new_square(40);

    println!("Rectangle yuyu: {:?}", yuyu);
    println!("Rectangle yoyo: {:?}", yoyo);
    println!("Area of yuyu: {}", yuyu.area());
    println!("Area of yoyo: {}", yoyo.area());
    println!("Perimeter of yuyu:{}", yuyu.perimeter());
    println!("Perimeter of yoyo: {}", yoyo.perimeter());

    // Named Tuple Structs
    struct Color(u8, u8, u8);
    
    let red: Color = Color(255, 0, 0);
    let blue: Color = Color(0, 255, 0);
    let green: Color = Color(0, 0, 255);

    impl Color
    {
        fn to_hex(&self) -> String
        {
            let hex_form = format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2);
            return hex_form;
        }

    }

    println!("Color: Red, hex: {}", red.to_hex());
    println!("Color: Blue, hex: {}", blue.to_hex());
    println!("Color: Green, hex: {}", green.to_hex());
}
