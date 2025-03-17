use std::fs;
use chrono::{DateTime, Utc};


fn main() {
    gen_tag("./dist/output@0.1.css");
    gen_stamp("./dist/output@0.1.css");
    gen_stamp("./dist/hx@2.0.4.js");
    gen_stamp("./dist/carousel@8.5.2.js");
}

fn gen_stamp(path: &str) {
    println!("cargo::rerun-if-changed={path}");
    let metadata = fs::metadata(path).unwrap().modified().unwrap();
    let datetime = DateTime::<Utc>::from(metadata);
    let mut buffer = String::new();
    datetime.format("%a, %d %b %Y %H:%M:%S GMT").write_to(&mut buffer).unwrap();
    fs::write(format!("{path}.stamp"), buffer).unwrap();
}

fn gen_tag(path: &str) {
    let metadata = fs::metadata(path).unwrap().modified().unwrap();
    let datetime = DateTime::<Utc>::from(metadata);
    let mut buffer = String::new();
    datetime.format("%d%m%Y%H%M%S").write_to(&mut buffer).unwrap();
    fs::write(format!("{path}.tag"), buffer).unwrap();
}

