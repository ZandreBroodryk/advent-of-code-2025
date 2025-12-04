use advent_of_code_2025::{day_1, day_2, day_3, day_4, shared::InputTypes};

fn main() {
    let result = day_1::part_1(Some(InputTypes::Example));

    println!("The answer for day 1 part 1 is: {result}");

    let result = day_1::part_2(Some(InputTypes::Example));

    println!("The answer for day 1 part 2 is: {result}");

    let result = day_2::part_1(Some(InputTypes::Example));

    println!("The answer for day 2 part 1 is: {result}");

    let result = day_2::part_2(Some(InputTypes::Example));

    println!("The answer for day 2 part 2 is: {result}");

    let result = day_3::part_1(InputTypes::Example);

    println!("The answer for day 3 part 1 is: {result}");

    let result = day_3::part_2(InputTypes::Example);

    println!("The answer for day 3 part 2 is: {result}");

    let result = day_4::part_1(InputTypes::MyInput);

    println!("The answer for day 4 part 1 is: {result}");

    // let result = day_4::part_2(InputTypes::Example);

    // println!("The answer for day 4 part 2 is: {result}");
}
