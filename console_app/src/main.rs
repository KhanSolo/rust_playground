fn main(){
    /*
    linux
    macos
    ios
    freebsd
    dragonfly
    netbsd
    openbsd
    solaris
    android
    windows
    */
    let os = std::env::consts::OS;

    match os {
        "windows" => {},
        "linux" => {},
        _ => {},
    }

    println!("{}", os);
}