fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    let buffer_overflow = fruit[4]; // в Rust вместо того, чтобы переменной было присвоено неправильное место в памяти, произойдет сбой компиляции

    assert_eq!(buffer_overflow, '🍉'); // assert_eq! -  проверяет равенство аргументов
}
