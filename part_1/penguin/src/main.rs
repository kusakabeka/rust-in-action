fn main() {
    // \ - отключение завершающего символа новой строки
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in  records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // пропуск строки заголовка и строк, сотоящих из одних пробелов
            continue;
        }
    }

    let fields: Vec<_> = record // начало со строки текста
        .split(',') // разбиение записи на поля
        .map(|field| field.trim()) // обрезка пробелов в каждом поле
        .collect(); // сборка набора полей
        if cfg!(debug_assertins) { // cfg! - проверяет конфигурацию в процессе компиляции
            eprintln!("debug: {:?} -> {:?}", record, fields); // eprintln! - выводит данные на стандартное устройство сообщений об ошибках (stderr)
        }

    let name = fields[0];
    if let Ok(length) = fields[1].parse::<f32>() { // попытка выполнения парсинга поля в виде числа с плавающей точкой
        println!("{}, {}cm", name, length); // println! - помещает данные на стандартное устройство вывода(stdout)
    }
}