use autoclicker::Autoclicker;

fn main() {
    let mut autoclicker = Autoclicker::new(std::time::Duration::from_millis(1));
    println!("Starting clicking after 2s");
    std::thread::sleep(std::time::Duration::from_millis(2000));
    autoclicker.start().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    autoclicker.stop().unwrap();
}
