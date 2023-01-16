//! Тестовый проект, показывающий,
//! как хранить в стеке значения разных типов.
//! 
//! Может быть полезен для написания стековой ВМ.

/// Представляет типы данных.
enum ValueType {
    INT,
    FLOAT,
    CHAR,
}

/// Представляет значение.
struct Value {
    value_type: ValueType,

    // Одно из этих полей должно быть не [`None`].
    int_value: Option<i32>,
    float_value: Option<f32>,
    char_value: Option<char>,
}

impl Value {
    fn new_int(value: i32) -> Self {
        Value {
            value_type: ValueType::INT,
            int_value: Some(value),
            float_value: None,
            char_value: None
        }
    }

    fn new_float(value: f32) -> Self {
        Value {
            value_type: ValueType::FLOAT,
            int_value: None,
            float_value: Some(value),
            char_value: None
        }
    }

    fn new_char(value: char) -> Self {
        Value {
            value_type: ValueType::CHAR,
            int_value: None,
            float_value: None,
            char_value: Some(value)
        }
    }
}

fn main() {
    // Для простоты стек заполняется фиксированными значениями.
    let stack = [
        Value::new_int(5),
        Value::new_char('k'),
        Value::new_int(123),
        Value::new_float(10.6),
        Value::new_int(18),
    ];

    // Вывод типов и значений в стеке.
    for el in stack {
        match el.value_type {
            ValueType::INT => println!("It's int: {}", el.int_value.expect("Error")),
            ValueType::FLOAT => println!("It's float: {}", el.float_value.expect("Error")),
            ValueType::CHAR => println!("It's char: {}", el.char_value.expect("Error")),
        }
    }
}
