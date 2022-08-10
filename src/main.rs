fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(7, 'm');
}

fn another_function(x: i32) {
    println!("Heyooooo! this is in another_function()...the value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
