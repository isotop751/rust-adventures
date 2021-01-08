fn common_house_functions()
{

}

pub mod hosting
{
    pub fn add_to_waiting()
    {
        // using relative path
        super::common_house_functions();
    }

    fn seat_at_table()
    {
        // relative path to the function
        super::super::some_generic_function();
    }
}

pub mod serving
{
    pub fn take_orders()
    {

    }

    fn serve_orders()
    {
        // Absolute path to function
        crate::some_generic_function();
    }

    fn collect_bills()
    {
        // absolute path to functions
        crate::front_of_the_house::common_house_functions();
    }
}