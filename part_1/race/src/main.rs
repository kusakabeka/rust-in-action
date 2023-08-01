use std::thread; // сведение многопоточности к локальной области видимости

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; }); // thread::spawn() принимает в качестве аргумента замыкание
    thread::spawn(|| {data = 500; }); // thread::spawn() принимает в качестве аргумента замыкание

    println!("{}", data);
}