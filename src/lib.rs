use std::collections::HashMap;

use doson::DataValue;
use pyo3::prelude::*;

#[pyfunction]
fn loads(value: String) -> PyResult<PyObject> {
    
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    
    let result = match DataValue::from(&value) {
        DataValue::None => py.None(),
        DataValue::String(v) => v.into_py(py),
        DataValue::Number(v) => v.into_py(py),
        DataValue::Boolean(v) => v.into_py(py),
        DataValue::List(v) => {
            let mut obj: Vec<PyObject> = vec![];
            for item in v {
                obj.push(loads(item.to_string())?);
            }
            obj.into_py(py)
        },
        DataValue::Dict(v) => {
            let mut obj: HashMap<String, PyObject> = Default::default();
            for (key, val) in v {
                obj.insert(key.clone(), loads(val.to_string())?);
            }
            obj.into_py(py)
        },
        DataValue::Tuple(v) => {
            (loads(v.0.to_string())?, loads(v.1.to_string())?).into_py(py)
        },
    };

    Ok(result)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn doson4py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    Ok(())
}