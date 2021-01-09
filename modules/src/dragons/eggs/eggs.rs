pub fn say_hi()
{
    println!("Hello, I am egg");
}

pub struct ThisEgg
{
    pub egg_color: String,
    pub egg_size: u8,
    egg_period: u16,
}

pub enum Colors
{
    Yellow,
    Green,
    Red,
    Blue,
}

pub fn egg_make(color: String, size: u8) -> ThisEgg
{
    let this = ThisEgg
    {
        egg_color: color,
        egg_size: size,
        egg_period: 100,
    };

    println!("made an egg with incubation period of {}", this.egg_period);
    return this;
}

pub fn expose_period(egg: ThisEgg) -> u16
{
    return egg.egg_period;
}