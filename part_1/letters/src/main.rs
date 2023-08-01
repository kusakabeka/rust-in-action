fn main() {
    let mut letters = vec!["a", "b", "c"]; // создание изменяемого вектора букв

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone()); // копирование каждой буквы и добавление ее к концу вектора letters
    }
    // код не пройдет компиляцию, тк Rust не позволяет переменной letters измениться в блоке итерации.
}