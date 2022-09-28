fn main() {
    println!("Hello, world!");
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // v3 - catchall and do nothing
        // returns unit value
        _ => (),
        // v2 - catchall and do something
        // _ => reroll(),

        // v1 - unused variable
        // other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
