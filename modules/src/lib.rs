fn some_generic_function()
{

}

mod front_of_the_house;

pub fn eat_at_diner()
{
    crate::front_of_the_house::hosting::add_to_waiting();

}

pub fn relative_path()
{
    front_of_the_house::serving::take_orders();
}

mod back_of_the_house;

fn order_breakfast()
{
    let mut meal = crate::back_of_the_house::backmod::Breakfast::order(String::from("Sandwich"));
    meal.main = String::from("Big Red Sandwich");
    // this wont compile because seasoning is not pub
    // meal.seasoning = String::from("Pineapple");

    // all items of a public enum are public
    let fruit = crate::back_of_the_house::backmod::Fruits::Bead;
}