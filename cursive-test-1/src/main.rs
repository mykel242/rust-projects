use cursive::views::TextView;

fn main() {
    // println!("Hello, world!");
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
    siv.run();
}
