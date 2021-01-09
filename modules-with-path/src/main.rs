#[path = "utils/printer.rs"] mod printer;
#[path = "modules/scanner/scanner.rs"] mod scam;

fn main()
{
    printer::print_hello();
    scam::scan();
}
