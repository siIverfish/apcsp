use super::*;

pub(crate) fn parse_procedure_calls(values: Vec<Value>) -> Vec<Value> {
    let mut new_values: Vec<Value> = Vec::with_capacity(values.len());
    let mut iter = values.into_iter().peekable();

    while let Some(value) = iter.next() {
        if let Value::RawLeaf(ref name) = value {
            if let Some(&Value::Group(_)) = iter.peek() {
                // it's a procedure call!!
                let Some(Value::Group(procedure_call_args)) = iter.next()
                    else { unreachable!() };
                
                // ok, now split the tokens in the call by the commas
                // with lots of allocations :(
                let mut split_args: Vec<Vec<Value>> = vec![Vec::new()];

                for argument_token in procedure_call_args {
                    if let Value::Operator(Operator::Comma) = argument_token {
                        split_args.push(Vec::new());
                    } else {
                        split_args.last_mut().unwrap().push(argument_token);
                    }
                }

                // each argument should be recursively parsed of function calls
                // and turned into a group
                let args = split_args
                    .into_iter()
                    .map(parse_procedure_calls)
                    .map(Value::Group)
                    .collect::<Vec<_>>();

                // voila!
                new_values.push(Value::ProcedureCall { name: name.to_string(), args })
            } else {
                new_values.push(value);
            }
        } else {
            // it's not a procedure call..
            new_values.push(value);
        }
    }

    new_values
}