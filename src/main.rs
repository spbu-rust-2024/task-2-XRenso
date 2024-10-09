use std::io;
fn longest_palindrome(s: &str) -> String {
    // Представление строки в байтах
    let s = s.as_bytes();

    //Длина строки
    let str_len = s.len();

    if str_len == 0 {
        return String::new(); //пустая строка, не особо пока понимаю
                              //как еще можно
    }
    //начало поиска
    let mut start = 0;

    //макс длина палиндрома
    let mut max_palindrome_len = 1;

    for i in 0..str_len {
        // Поиск нечетных палиндромов. Берем правую и левую границу в точке i
        // Так мы сможем отсчитать наш палиндром
        let (lo, ro) = left_and_right_border_palindrom(s, i, i);

        // Если длина палиндрома, что мы нашли больше максимально, значит
        // Это и есть наш самый большой
        // А старт отсчета мы ставим на левую границу
        if ro - lo > max_palindrome_len {
            start = lo;
            max_palindrome_len = ro - lo;
        }

        // Поиск четных палиндромов, потому правая граница больше на 1
        // В остальном алгоритм индентичен с нечетным
        let (le, re) = left_and_right_border_palindrom(s, i, i + 1);
        if re - le > max_palindrome_len {
            start = le;
            max_palindrome_len = re - le;
        }
    }
    // Конечный результат это наша строка, что является
    // Набором символов из оригинальной строки
    // Она берется из точки начала + длина
    return String::from_utf8_lossy(&s[start..start + max_palindrome_len]).to_string();
}

// Функция чтобы смотреть на сколько мы можем расшириться по сторонам
fn left_and_right_border_palindrom(s: &[u8], left: usize, right: usize) -> (usize, usize) {
    let mut l = left;
    let mut r = right;

    // Здесь мы проверяем, что есть куда идти влево и вправо
    // Еще учитываем, что наши символы равны
    while l > 0 && r < s.len() && s[l - 1] == s[r] {
        l -= 1;
        r += 1;
    }
    // результат это кортеж из левой и правой границы нашего палиндрома
    return (l, r);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Can't read");
    let ans = longest_palindrome(&input);
    println!("{}", ans);
}
