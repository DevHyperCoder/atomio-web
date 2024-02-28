use std::collections::HashMap;

use atomio::parser::Parsed;
use yew::prelude::*;

use crate::utils::{calculate_mass, get_mass_composition};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct StatsProps {
    pub parsed: Parsed,
}

#[function_component]
pub fn Stats(props: &StatsProps) -> Html {
    let parsed = props.parsed.clone();

    let mass = calculate_mass(&parsed);
    let mass_composition = get_mass_composition(&parsed);

    let formula_composition = parsed.root_unit.get_composition();

    let num_formula_units: u32 = formula_composition.iter().map(|(_, v)| v).sum();

    let mut formula_composition_perc = HashMap::new();
    for (key, val) in formula_composition {
        formula_composition_perc.insert(key, (val as f64) * 100. / (num_formula_units as f64));
    }

    html!(
        <section class="mt-5">
            <h2 class="text-slate-800 font-black text-3xl">{format!("Mass: {} g/mol",mass)}</h2>

            <h2 class="text-slate-800 font-black text-3xl mt-3">{"Formula Composition: "}</h2>
            <div class="flex flex-col gap-3 bg-white w-full rounded-md drop-shadow-xl mt-4 px-2 py-2">
                {for formula_composition_perc.iter().map(|(k,v)| {
                        html!(
                            <div data-element={k.to_string()} data-comp={format!("{:.2}",v)} class="w-full flex gap-2 items-center bar-graph">
                                <div class={format!("h-6 px-1 py-4 rounded-md bg-orange-500")} style={format!("width: {}%",v)}> </div>
                            </div>
                        )
                })}
            </div>

            <h2 class="text-slate-800 font-black text-3xl mt-3">{"Mass Composition: "}</h2>
            <div class="flex flex-col gap-3 bg-white w-full rounded-md drop-shadow-xl mt-4 px-2 py-2">
                {for mass_composition.iter().map(|(k,v)| {
                        html!(
                            <div data-element={k.to_string()} data-comp={format!("{:.2}",v)} class="w-full flex gap-2 items-center bar-graph">
                                <div class={format!("h-6 px-1 py-4 rounded-md bg-orange-500")} style={format!("width: {}%",v)}> </div>
                            </div>
                        )
                })}
            </div>
        </section>
    )
}
