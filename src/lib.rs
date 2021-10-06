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

#[pyfunction]
fn dumps(object: PyObject) -> PyResult<String> {
    
    let val = object_to_value(object);

    Ok(String::from(val.to_string()))
}

// PyObject to DataValue
fn object_to_value(object: PyObject) -> DataValue {
    
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let temp = object.extract::<String>(py);
    if temp.is_ok() {
        return DataValue::String(temp.unwrap());
    }

    let temp = object.extract::<bool>(py);
    if temp.is_ok() {
        return DataValue::Boolean(temp.unwrap());
    }

    let temp = object.extract::<f64>(py);
    if temp.is_ok() {
        return DataValue::Number(temp.unwrap());
    }

    let temp = object.extract::<(PyObject, PyObject)>(py);
    if temp.is_ok() {

        let temp = temp.unwrap();

        return DataValue::Tuple((
            Box::new( object_to_value(temp.0) ),
            Box::new( object_to_value(temp.1) ),
        ));
    }

    let temp = object.extract::<Vec<PyObject>>(py);
    if temp.is_ok() {

        let temp = temp.unwrap();
        let mut result: Vec<DataValue> = vec![];

        for item in temp {
            let val = object_to_value(item);
            result.push(val);
        }

        return DataValue::List(result);
    }

    let temp = object.extract::<HashMap<String, PyObject>>(py);
    if temp.is_ok() {

        let temp = temp.unwrap();
        let mut result: HashMap<String, DataValue> = Default::default();

        for (key, item) in temp {
            let val = object_to_value(item);
            result.insert(key, val);
        }

        return DataValue::Dict(result);
    }

    DataValue::None
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn doson4py(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;

    Ok(())

}