/*
    [!] Висячие указатели - прямые ссылки на данные, ставшие недействительными в ходе выполнения программы =
    [!] Состояния гонки - неспособность из-за изменения внешних фактров определить, как программа будет
        себя вести от запуска к запуску
    [!] Переполнение буфера - попытка обращения к 12-му элеменут массива, состоящего из 6 элементов
    [!] Недействительность итератора - проблема, вызываемая проходом какого-то объекта-итератора, претерпевшего
        изменения уже в ходе итерации
*/

#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);

    println!("{:?}", grains);
}