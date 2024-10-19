use clap::{Arg, Command};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::fs;

use std::time::Instant;

// Структура, хранящая информацию о работе программы, впоследстве будет преобразована в json
#[derive(Serialize, Deserialize)]
struct Output {
    elapsed: String,
    result: BTreeMap<char, usize>,
}

fn main() {
    // Указываем, что ожидаем флаг -t и имя файла
    let matches = Command::new("My Program")
        .version("1.0")
        .about("fl")
        .arg(
            Arg::new("threads")
                .short('t')
                .help("select only these fields")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("input").help("Input file to use").required(true), // .index(1),
        )
        .get_matches();

    // Проверяем, установлен ли флаг "-t", получаем значение количества потоков
    let threads_num = match matches.contains_id("threads") {
        true => *matches.get_one::<usize>("threads").unwrap(),
        false => 1,
    };

    // Массив, хранящий буквы латинского алфавита в виде байтов
    let alphabet: [u8; 26] = [
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
        88, 89, 90,
    ];

    // Здесь храним имя файла
    let input = matches.get_one::<String>("input").unwrap();

    // Считываем содержимое файла и записываем в String
    let contents = fs::read_to_string(input).expect("Should have been able to read the file");

    // Создаем пул потоков при помощи библиотеки Rayon
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads_num) // указываем нужное количество потоков
        .build()
        .unwrap();

    // Код, который будет выполняться параллельно
    pool.install(|| {
        let start = Instant::now(); // начинаем отсчет времени

        // Подсчитываем количество повторений каждого символа в тексте
        let counts: Vec<(char, usize)> = alphabet
            .par_iter() // создаем параллельный итератор для массива содержащего алфавит
            .map(|&item| {
                // Для каждого элемента алфавита проходим по строке входных данных и сравниваем элемент строки с буквой алфавита
                let lower_item = item + 32;
                // Итерируемся по байтам строки входных данных
                let count = contents
                    .as_bytes()
                    .iter()
                    .filter(|&&x| x == item || x == lower_item) // если данный элемент совпадает с текущим элементом массива - увеличивем счетчик
                    .count();

                // Возвращаем текущий символ и количество нахождений
                (item as char, count)
            })
            .collect(); // собираем в вектор

        // Преобразовываем результат из вектора в MAP
        let mut res = BTreeMap::new();
        for (item, count) in counts {
            res.insert(item, count);
        }
        // Заканчиваем отсчет времени, сохраняем результат
        let duration = start.elapsed();
        // Преобразуем результат подсчета времени в строку
        let time = format!("{}.{} s", duration.as_secs(), duration.as_millis());
        // Формируем экземпляр стркуктуры с результатом работы
        let output = Output {
            elapsed: time,
            result: res,
        };
        // Преобразовываем в json и выводим в терминал
        let json_string = serde_json::to_string(&output).unwrap();
        println!("{}", json_string);
    });
}
