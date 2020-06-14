use std::env;

fn main() {
  let mut build = cc::Build::new();

  let target = env::var("TARGET").unwrap();

  build
    .include("./boxer/include")
    .flag_if_supported("-std=c11")
    .flag_if_supported("-w");

  if target.contains("linux") || target.contains("bsd") {
    let gtk3 = pkg_config::Config::new()
      .atleast_version("2.8")
      .probe("gtk+-3.0")
      .unwrap();
    build.file("boxer/boxer_linux.c");

    for path in gtk3.include_paths {
      build.include(path);
    }
  }

  build.compile("boxer");
}
