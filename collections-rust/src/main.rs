use std::collections::HashMap;

fn main()
{
    // arrays
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // a dictionary or hashmap
    let mut dict: HashMap<&str, u8> = HashMap::new();
    for i in 0..12
    {
        dict.insert(months[i], days[i]);
    }

    println!("{:?}", dict);

    // vectors
    let mut names: Vec<&str> = vec!["Hangul", "Hanoi", "Dragora", "Mithyl", "Spectre", "Thaag", "Jagir"];
    names.push("Constra");
    println!("{:?}", names);

    let last_added = names.pop();
    println!("Original: {:?}, Last Added: {:?}", names, last_added);

    let more_names: Vec<&str> = vec!["Conky", "Fabraia"];
    println!("Adding: {:?}", more_names);

    names.extend_from_slice(&more_names);
    println!("{:?}", names);
    println!("The original vector is preserved on using .extend_from_slice {:?}", more_names);
}
