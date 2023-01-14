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
    /// Создает объект [Value] со стандартными значениями, в зависимости от типа.
    /// Стандартные значения для разных типов:
    ///   INT   = 5
    ///   FLOAT = 6.2
    ///   CHAR  = 'H'
    fn new(value_type: ValueType) -> Self {
        match value_type {
        ValueType::INT => Value {
            value_type: ValueType::INT,

            int_value: Some(5),
            float_value: None,
            char_value: None
        },
        ValueType::FLOAT => Value {
            value_type: ValueType::FLOAT,

            int_value: None,
            float_value: Some(6.2),
            char_value: None
        },
        ValueType::CHAR => Value {
            value_type: ValueType::CHAR,

            int_value: None,
            float_value: None,
            char_value: Some('H')
        },
    }
    }
}

fn main() {
    // Для простоты стек заполняется фиксированными значениями.
    let stack = [
        Value::new(ValueType::INT),
        Value::new(ValueType::CHAR),
        Value::new(ValueType::INT),
        Value::new(ValueType::FLOAT),
        Value::new(ValueType::INT),
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
