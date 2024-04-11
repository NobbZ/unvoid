use unvoid::rune::prepare_context;

const VERSION: &str = "0.13.0";

fn main() {
    rune::cli::Entry::new()
        .about(format_args!("My Rune Project {VERSION}"))
        .context(&mut |_opts| Ok(prepare_context().unwrap()))
        .run();
}
