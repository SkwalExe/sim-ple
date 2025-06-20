use eframe::run_native;
use mathsim::Mathsim;

fn main() {
    env_logger::init();
    // Use the default window options
    let win_options = eframe::NativeOptions::default();

    // Run our app
    let _ = run_native(
        "Mathsim",
        win_options,
        Box::new(|cc| Ok(Box::new(Mathsim::new(cc)))),
    )
    .inspect_err(|err| {
        println!("Could not start app: {err}");
    });
}
