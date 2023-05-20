use super::Value;

pub fn tidy(value: Value) -> Value {
    flatten_single_vecs(
        parse_string_types(value)
    )
}

fn flatten_single_vecs(value: Value) -> Value {
    value.apply_to_all(&|value| {
        match value {
            &Value::Group(ref values) => {
                if values.len() == 1 {
                    values[0].clone()
                } else {
                    value.clone()
                }
            },

            _ => value.clone(),
        }
    })
}

fn parse_string_types(value: Value) -> Value {
    value.apply_to_all_strings(&|value| {
        let Value::String(value) = value 
            else { unreachable!("apply_to_all should only give Value::Strings or other non-Value-containing types."); };

        if let Ok(int_value) = str::parse::<i128>(value) {
            Value::I128(int_value)
        } else if let Some(string_value) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            Value::String(string_value.into())
        } else if value.chars().all(|c| c.is_alphabetic() || c == '_' ) {
            Value::Name(value.into())
        } else {
            panic!("could not understand value: {value:?}");
        }
    })
}