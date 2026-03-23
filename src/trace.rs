pub fn section(message: impl AsRef<str>) {
    println!("[packages] {}", message.as_ref());
}

pub fn detail(message: impl AsRef<str>) {
    println!("[packages][detail] {}", message.as_ref());
}

pub fn package(package: &str, message: impl AsRef<str>) {
    println!("[packages][{}] {}", package, message.as_ref());
}

pub fn package_detail(package: &str, message: impl AsRef<str>) {
    println!("[packages][{}][detail] {}", package, message.as_ref());
}

pub fn command(message: impl AsRef<str>) {
    println!("[packages][cmd] {}", message.as_ref());
}

pub fn command_detail(message: impl AsRef<str>) {
    println!("[packages][cmd][detail] {}", message.as_ref());
}

pub fn warn(message: impl AsRef<str>) {
    eprintln!("[packages][warn] {}", message.as_ref());
}
