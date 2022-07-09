use atomio::parser::Parsed;
use yew::prelude::*;

use crate::input::TextInput;
use crate::stats::Stats;

#[function_component]
pub fn App() -> Html {
    let chem_formula:UseStateHandle<String> = use_state(|| "".into());
    let chem_formula_error:UseStateHandle<String> = use_state(|| "".into());
    let parsed: UseStateHandle<Option<Parsed>> = use_state(|| None);

    let on_change = {
        let chem_formula_error = chem_formula_error.clone();
        let parsed = parsed.clone();

        Callback::from(move |a: String| {
            let formula_str = a.as_str();
            let tp: Result<Parsed,String> = formula_str.try_into();

            match tp {
                Ok(p) => {
                    chem_formula_error.set("".into());
                    parsed.set(Some(p))
                }
                Err(e) => {
                    chem_formula_error.set(e);
                    parsed.set(None);
                }
            }
        })
    };

    let stats = match &(*parsed) {
        None => html!(<> </>),
        Some(parsed) => html!(<Stats parsed={parsed.clone()} />)
    };

    html!(<>
        <main class="w-3/4 h-full mx-auto">
            <h1 class="text-slate-900 font-black text-3xl text-center my-5">{"ATOMIO - Molecular Mass and Composition"}</h1>
            <TextInput placeholder={"Enter chemical formula..."} 
                label={"Chemical Formula:"}
                error_msg={(*chem_formula_error).clone()}
                value={(*chem_formula).clone()}
                {on_change}
            />

            {stats}
        </main>
    </>)
}
