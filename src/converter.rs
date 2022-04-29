use serde_json::{Number, Value};

pub struct Converter;

impl Converter {
    pub fn run(hocon: hocon::Hocon) -> Result<String, hocon::Error> {
        let json: Option<Value> = Converter::hocon_to_json(hocon);

        let output = serde_json::to_string_pretty(&json).map_err(|e| hocon::Error::Deserialization { message: e.to_string() })?;

        Ok(output)
    }

    fn hocon_to_json(hocon: hocon::Hocon) -> Option<Value> {
        match hocon {
            hocon::Hocon::Boolean(b) => Some(Value::Bool(b)),
            hocon::Hocon::Integer(i) => Some(Value::Number(Number::from(i))),
            hocon::Hocon::Real(f) => Some(Value::Number(Number::from_f64(f).unwrap_or(Number::from(0)))),
            hocon::Hocon::String(s) => Some(Value::String(s)),
            hocon::Hocon::Array(vec) => Some(Value::Array(
                vec.into_iter().map(Converter::hocon_to_json).filter_map(|i| i).collect(),
            )),
            hocon::Hocon::Hash(map) => Some(Value::Object(
                map
                    .into_iter()
                    .map(|(k, v)| (k, Converter::hocon_to_json(v)))
                    .filter_map(|(k, v)| v.map(|v| (k, v)))
                    .collect(),
            )),
            hocon::Hocon::Null => Some(Value::Null),
            hocon::Hocon::BadValue(_) => None,
        }
    }
}
