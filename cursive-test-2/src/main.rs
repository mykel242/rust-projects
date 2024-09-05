use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    println!("Hello, world!");
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("Hello\nPress <NEXT> to continue.")
            .title("Very Important")
            .button("NEXT", show_next),
    );

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("You did it.")
            .title("Q1")
            .button("Yes", |s| show_answer(s, "You picked Yes."))
            .button("No", |s| show_answer(s, "You picked No?!"))
            .button("IDK", |s| {
                s.add_layer(Dialog::info("🙈 I don't know either."))
            }),
    );
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Your Results.")
            .button("fin.", |s| s.quit()),
    );
}
