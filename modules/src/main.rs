mod dragons;

use dragons::eggs;

fn main() {
    println!("Hello, world!");
    dragons::dragon::say_hello();
    dragons::eggs::eggs::say_hi();

    // struct demos
    let an_egg = eggs::eggs::egg_make(String::from("Yellow"), 10);
    
    // Even if the struct is public - its members are not by default
    println!("EggColor: {}", an_egg.egg_color);
    println!("EggSize: {}", an_egg.egg_size);
    // this wont compile
    // println!("{}", an_egg.EggPeriod);
    // but this technique will help expose it to us
    println!("The incubation period is {}", eggs::eggs::expose_period(an_egg));


    // demo for enums
    let egg_colors = eggs::eggs::Colors::Blue;
    if match egg_colors
    {
        eggs::eggs::Colors::Blue => true,
        eggs::eggs::Colors::Red => false,
        eggs::eggs::Colors::Yellow => false,
        eggs::eggs::Colors::Green => false,
    }
    {
        println!("Color is blue");
    }
    else
    {
        println!("Color is something else");
    }

}
