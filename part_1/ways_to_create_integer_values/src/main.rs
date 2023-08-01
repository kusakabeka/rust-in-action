use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // целое число в стеке
    let b = Box::new(20); // целое число в куче, также именуемое "упакованное целое число"
    let c = Rc::new(Box::new(30)); // упакованное целое число, завернутое в счетчик ссылок
    let d = Arc::new(Mutex::new(40)); // Целое число, завернутое в атомарный счетчик ссылок и защищенное блокировкой взаимного исключения

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}