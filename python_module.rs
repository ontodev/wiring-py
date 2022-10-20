use pyo3::prelude::*;
use serde_json::{Value};
//use serde_json::json; 
use crate::thick_2_ofn::translation::thick_2_ofn as t2ofn; 
use crate::ofn_typing::translation::type_ofn as typing; 
use crate::ofn_typing::typing::extract_typing as extract_typing; 
use crate::ofn_labeling::translation::label_ofn as labeling; 
use crate::ofn_labeling::labeling::extract_labeling as extract_labeling; 
use crate::ofn_2_thick::translation::ofn_2_thick as ofn2t; 
use crate::ldtab_2_ofn::translation::thick_triple_2_ofn as translate_triple; 
use crate::util::parser::parse_thick_triple_object as parse_object; 
use crate::ldtab_2_ofn::class_translation::translate as object_translation; 
use crate::ofn_2_rdfa::class_translation::translate as rdfa_object_translation; 
use crate::util::signature::extract as extract_signature; 
use crate::ofn_2_man::translation::ofn_2_man as ofn2man;
use crate::ofn_2_ldtab::translation::ofn_2_thick_triple as ofn2ldtab; //currently only supports OWL class expression axioms
use crate::ofn_2_ldtab::util::sort_value as sort_value; 
use std::collections::HashMap;
use std::collections::HashSet;
use crate::owl::thick_triple as tt;

#[pyfunction]
fn get_signature(ofn: &str) -> HashSet<String> { 

    let v : Value = serde_json::from_str(ofn).unwrap(); 
    let signature : Vec<Value> = extract_signature(&v);

    let mut res = HashSet::new();

    for s in signature.iter() {
        res.insert(String::from(s.as_str().unwrap()));
    } 
    res 
}

#[pyfunction]
fn object_2_ofn(obj: &str) -> String { 

    let object : tt::OWL = parse_object(obj);
    let ofn = object_translation(&object);
    format!("{}", ofn)
}

#[pyfunction]
fn object_2_rdfa(obj: &str, m : HashMap<String, String>) -> String {

    let ofn : Value = serde_json::from_str(obj).unwrap(); 
    let rdfa = rdfa_object_translation(&ofn, &m, None);
    format!("{}", rdfa) 
} 

#[pyfunction]
fn ldtab_2_ofn(t: &str) -> String { 

    let triple : Value = serde_json::from_str(t).unwrap();
    let ofn : Value = translate_triple(&triple);
    format!("{}", ofn)
}

#[pyfunction]
fn thick_2_ofn(t : &str) -> String { 

    let triple : Value = serde_json::from_str(t).unwrap();
    let ofn = t2ofn(&triple);
    //let ofn = t2ofn(triple.to_string().as_str());
    format!("{}", ofn)
}

#[pyfunction]
fn ofn_2_thick(t : &str) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let ofn = ofn2t(&v);
    ofn.to_string()
}

#[pyfunction] 
fn extract_types(t: &str) -> HashMap<String,HashSet<String>> { 
    extract_typing(t) 
}

#[pyfunction] 
fn ofn_typing(t: &str, m : HashMap<String, HashSet<String>>) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let typed = typing(&v, &m);
    format!("{}", typed) 
}

#[pyfunction] 
fn extract_labels(t: &str) -> HashMap<String,String> { 
    extract_labeling(t) 
}

#[pyfunction] 
fn ofn_labeling(t: &str, m : HashMap<String, String>) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let labeled = labeling(&v, &m);
    format!("{}", labeled) 
}

#[pyfunction] 
fn ofn_2_man(t: &str) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let man = ofn2man(&v);
    format!("{}", man) 
}

#[pyfunction] 
fn ofn_2_ldtab(t: &str) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let json = ofn2ldtab(&v);
    format!("{}", json) 
}

#[pyfunction] 
fn sort_json(t: &str) -> String { 

    let ofn : Value = serde_json::from_str(t).unwrap(); 
    let ofn_sorted = sort_value(&ofn); 
    format!("{}", ofn_sorted) 
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn wiring_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(thick_2_ofn, m)?)?;
    m.add_function(wrap_pyfunction!(ldtab_2_ofn, m)?)?;
    m.add_function(wrap_pyfunction!(object_2_ofn, m)?)?;
    m.add_function(wrap_pyfunction!(object_2_rdfa, m)?)?;
    m.add_function(wrap_pyfunction!(get_signature, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_typing, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_labeling, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_2_man, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_2_thick, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_2_ldtab, m)?)?;
    m.add_function(wrap_pyfunction!(extract_labels, m)?)?;
    m.add_function(wrap_pyfunction!(extract_types, m)?)?;
    m.add_function(wrap_pyfunction!(sort_json, m)?)?;

    Ok(())
}
