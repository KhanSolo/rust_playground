use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rerun-if-changed=appsettings.json");
    // println!("cargo:warning=Hello from build.rs");
    // println!("cargo:warning=CWD is {:?}", env::current_dir().unwrap());
    // println!("cargo:warning=OUT_DIR is {:?}", env::var("OUT_DIR").unwrap());
    // println!("cargo:warning=CARGO_MANIFEST_DIR is {:?}", env::var("CARGO_MANIFEST_DIR").unwrap());
    // println!("cargo:warning=PROFILE is {:?}", env::var("PROFILE").unwrap());

    let output_path = get_output_path();
    println!(
        "cargo:warning=Calculated build path: {}",
        output_path.to_str().unwrap()
    );

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("appsettings.json");
    let output_path = Path::new(&output_path).join("appsettings.json");
    let res = std::fs::copy(input_path, output_path);
    println!("cargo:warning={:#?}", res)
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}
