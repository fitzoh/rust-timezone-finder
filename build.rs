use std::process::{Command, ExitStatus};
use std::io;


fn check_command(status: io::Result<ExitStatus>, message: &'static str) {
    assert!(status.expect(message.clone()).success(), message)
}

//Download and unzip the latest (at time of writing) timezone-boundary-builder shapefile release
fn main() {
    let url = "https://github.com/evansiroky/timezone-boundary-builder/releases/download/2020d/timezones.shapefile.zip";
    check_command(Command::new("wget").current_dir("tzdata").arg(url).arg("-O").arg("timezones.zip").status(), "failed to download timezone file");
    check_command(Command::new("unzip").current_dir("tzdata").arg("-o").arg("timezones.zip").status(), "failed to unzip");
    println!("cargo:rerun-if-changed=build.rs");
}