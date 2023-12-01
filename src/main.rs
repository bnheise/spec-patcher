use spec_patcher::run;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }
}
