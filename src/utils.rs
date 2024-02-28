use std::collections::HashMap;

use atomio::parser::Parsed;
use crate::data::ELEMENTS;

pub fn calculate_mass(p: &Parsed) -> f32 {
    let mut mass = 0.0;
    let comps = p.root_unit.get_composition();

    for (k,v) in comps {
        let m = ELEMENTS.get(k).unwrap();
        mass += m * (v as f32)
    }

    mass
}

pub fn get_mass_composition(p: &Parsed) -> HashMap<&String,f32> {
    let mass = calculate_mass(&p);
    let mut map = HashMap::new();
    let formula_composition = p.root_unit.get_composition();
    for (k, val) in formula_composition {
        map.insert(
            k, 
            (val as f32) * ELEMENTS.get(k).unwrap() * 100. / mass
        );
    }

    map
}
