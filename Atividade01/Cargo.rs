extern crate restaurant;

use restaurant::front_of_house::hosting;
use restaurant::back_of_house;
use restaurant::PI;

fn main() {
    hosting::add_to_wait_list();
    hosting::seat_at_table();
    back_of_house::take_care_trash();
    println!("O valor de PI é: {}", PI);
}