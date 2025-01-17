use std::borrow::Cow;

use serde_json::Value;

use crate::{
    registry::{MetaSchema, MetaSchemaRef, Registry},
    types::{ParseError, ParseFromJSON, ParseResult, ToJSON, Type},
};

impl<T: Type, const LEN: usize> Type for [T; LEN] {
    fn name() -> Cow<'static, str> {
        format!("[{}]", T::name()).into()
    }

    impl_raw_value_type!();

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema {
            items: Some(Box::new(T::schema_ref())),
            max_length: Some(LEN),
            min_length: Some(LEN),
            ..MetaSchema::new("array")
        }))
    }

    fn register(registry: &mut Registry) {
        T::register(registry);
    }
}

impl<T: ParseFromJSON, const LEN: usize> ParseFromJSON for [T; LEN] {
    fn parse_from_json(value: Value) -> ParseResult<Self> {
        match value {
            Value::Array(values) => {
                if values.len() != LEN {
                    return Err(ParseError::custom(format!(
                        "the length of the list must be `{}`.",
                        LEN
                    )));
                }

                let mut res = Vec::with_capacity(values.len());
                for value in values {
                    res.push(T::parse_from_json(value).map_err(ParseError::propagate)?);
                }

                Ok(res.try_into().ok().unwrap())
            }
            _ => Err(ParseError::expected_type(value)),
        }
    }
}

impl<T: ToJSON, const LEN: usize> ToJSON for [T; LEN] {
    fn to_json(&self) -> Value {
        let mut values = Vec::with_capacity(self.len());
        for item in self {
            values.push(item.to_json());
        }
        Value::Array(values)
    }
}
