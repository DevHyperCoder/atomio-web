use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub label: String,
    pub placeholder: String,
    pub error_msg: String,
    pub on_change: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        label,
        value,
        placeholder,
        error_msg,
        on_change,
    } = props.clone();

    let text_input_value: UseStateHandle<String> = use_state(|| value);
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        use_effect_with_deps(
            move |node_ref| {
                let input_elem = node_ref.cast::<HtmlInputElement>().unwrap();
                input_elem.focus().unwrap();

                // Cleanup func
                || ()
            },
            input_ref,
        );
    }

    let oninput = {
        let text_input_value = text_input_value.clone();
        Callback::from(move |input_event: InputEvent| {
            text_input_value.set(get_value_from_input_event(input_event));
        })
    };

    let onsubmit = {
        let text_input_value = text_input_value.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            on_change.emit((*text_input_value).clone())
        })
    };

    html! {
        <div class="flex gap-1 flex-col">
            <label class="text-slate-800 font-bold text-sm">{label}</label>
            <form {onsubmit} class="w-full flex drop-shadow-md items-center bg-white">
                <input
                    ref={input_ref.clone()}
                    class="text-lg block w-full px-2 font-bold placeholder:text-slate-500 py-2 border-0 border-b-2 border-white focus:ring-0 focus:border-black "
                    type="text" {placeholder} value={(*text_input_value).clone()} {oninput} />
                <button type="submit">
                    <img class="h-8 mr-2" src="res/search.svg" alt="search img" />
                </button>
            </form>
            <p class="text-sm font-semibold text-red-500">{error_msg}</p>
        </div>
    }
}
