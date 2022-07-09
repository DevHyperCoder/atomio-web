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
