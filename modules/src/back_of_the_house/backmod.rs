pub struct Breakfast
{
    pub main: String,
    seasoning: String,
}

impl Breakfast
{
    pub fn order(main: String) -> Breakfast
    {
        return Breakfast
        {
            main: String::from(main),
            seasoning: String::from("Peaches"),
        };
    }
}

pub enum Fruits
{
    Apple,
    Bead,
    Salts,
    Snakes,
    Frog
}