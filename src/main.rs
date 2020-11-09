use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Exactly 2 arguments are expected");
    }
    let conversion_type = args[1].as_str();
    let filename = &args[2];
    match conversion_type {
        "enru" => en_to_ru(filename),
        "ruen" => ru_to_en(filename),
        _ => panic!("Invalid conversion type"),
    }
}

fn en_to_ru(filename: &String) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let output: String = content.chars().map(|c| en_to_ru_map(c)).collect();
    fs::write(format!("{}_fixed",filename), output).expect("Write failed");
}

fn en_to_ru_map(character: char) -> char {
    match character {
        '`' => 'ё', '~' => 'Ё', '@' => '\"', '#' => '№', '$' => ';', '^' => ':', '&' => '?', 'q' => 'й', 'Q' => 'Й', 'w' => 'ц', 'W' => 'Ц', 'e' => 'у', 'E' => 'У', 'r' => 'к', 'R' => 'К', 't' => 'е',
        'T' => 'Е', 'y' => 'н', 'Y' => 'Н', 'u' => 'г', 'U' => 'Г', 'i' => 'ш', 'I' => 'Ш', 'o' => 'щ', 'O' => 'Щ', 'p' => 'з', 'P' => 'З', '[' => 'х', '{' => 'Х', ']' => 'ъ', '}' => 'Ъ', 'a' => 'ф',
        'A' => 'Ф', 's' => 'ы', 'S' => 'Ы', 'd' => 'в', 'D' => 'В', 'f' => 'а', 'F' => 'А', 'g' => 'п', 'G' => 'П', 'h' => 'р', 'H' => 'Р', 'j' => 'о', 'J' => 'О', 'k' => 'л', 'K' => 'Л', 'l' => 'д',
        'L' => 'Д', ';' => 'ж', ':' => 'Ж', '\'' => 'э', '\"' => 'Э', 'z' => 'я', 'Z' => 'Я', 'x' => 'ч', 'X' => 'Ч', 'c' => 'с', 'C' => 'С', 'v' => 'м', 'V' => 'М', 'b' => 'и', 'B' => 'И', 'n' => 'т',
        'N' => 'Т', 'm' => 'ь', 'M' => 'Ь', ',' => 'б', '<' => 'Б', '.' => 'ю', '>' => 'Ю', '/' => '.', '?' => ',', _ => character ,
    }
}

fn ru_to_en(filename: &String) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let output: String = content.chars().map(|c| ru_to_en_map(c)).collect();
    fs::write(format!("{}_fixed",filename), output).expect("Write failed");
}

fn ru_to_en_map(character: char) -> char {
    match character {
        'ё' => '`', 'Ё' => '~', '\"' => '@', '№' => '#', ';' => '$', ':' => '^', '?' => '&', 'й' => 'q', 'Й' => 'Q', 'ц' => 'w', 'Ц' => 'W', 'у' => 'e', 'У' => 'E', 'к' => 'r', 'К' => 'R', 'е' => 't',
        'Е' => 'T', 'н' => 'y', 'Н' => 'Y', 'г' => 'u', 'Г' => 'U', 'ш' => 'i', 'Ш' => 'I', 'щ' => 'o', 'Щ' => 'O', 'з' => 'p', 'З' => 'P', 'х' => '[', 'Х' => '{', 'ъ' => ']', 'Ъ' => '}', 'ф' => 'a',
        'Ф' => 'A', 'ы' => 's', 'Ы' => 'S', 'в' => 'd', 'В' => 'D', 'а' => 'f', 'А' => 'F', 'п' => 'g', 'П' => 'G', 'р' => 'h', 'Р' => 'H', 'о' => 'j', 'О' => 'J', 'л' => 'k', 'Л' => 'K', 'д' => 'l',
        'Д' => 'L', 'ж' => ';', 'Ж' => ':', 'э' => '\'', 'Э' => '\"', 'я' => 'z', 'Я' => 'Z', 'ч' => 'x', 'Ч' => 'X', 'с' => 'c', 'С' => 'C', 'м' => 'v', 'М' => 'V', 'и' => 'b', 'И' => 'B', 'т' => 'n',
        'Т' => 'N', 'ь' => 'm', 'Ь' => 'M', 'б' => ',', 'Б' => '<', 'ю' => '.', 'Ю' => '>', '.' => '/', ',' => '?', _ => character ,
    }
}
