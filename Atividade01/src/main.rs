mod front_of_house;
mod back_of_house;
mod constants;

fn main() {
    front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::seat_at_table();
    back_of_house::take_care_trash();
    println!("Value of PI: {}", constants::PI);
}
