use super::Value;

pub fn tidy(value: Value) -> Value {
    let value = flatten_single_vecs(value);

    let value = parse_string_types(value);

    value
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
    value.apply_to_all_raw_leafs(&|value| {
        let Value::RawLeaf(value) = value 
            else { unreachable!("apply_to_all should only give Value::Strings or other non-Value-containing types."); };

        if let Ok(int_value) = str::parse::<i128>(value) {
            Value::I128(int_value)
        } else if let Some(string_value) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            Value::String(string_value.to_string())
        } else if value.chars().all(|c| c.is_alphabetic() || c == '_' ) {
            Value::Name(value.into())
        } else {
            panic!("could not understand value: <{value}>");
        }
    })
}