use spec_patcher::error::Reporter;
use spec_patcher::run;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => e.report("ERROR: Failed to execute operation"),
    }
}
