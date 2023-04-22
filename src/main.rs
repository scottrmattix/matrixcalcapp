use yew::prelude::*;
mod matrices;
use crate::matrices::Matrix;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;


#[derive(Properties, PartialEq)]
struct FormMatrixProps{
    onsubmit: Callback<String>,
}

#[derive(Properties, PartialEq)]
struct SubmitButtonProps{
    label: String,
    name: String,
}
#[derive(Properties, PartialEq)]
struct TextInputProps{
    name: String,
    handle_onchange: Callback<String>,
}

#[function_component(SubmitButton)]
fn submit_button(props: &SubmitButtonProps) ->Html{
    html!{
        <button name={props.name.clone()}>{&props.label}</button>
    }
}
#[function_component(TextInput)]
fn text_input(props: &TextInputProps) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        handle_onchange.emit(input.value());
    });
    html!{
        <input type="text" name={props.name.clone()} onchange={onchange} />
    }
}

#[function_component(FormMatrix)]
fn matrix_form(props: &FormMatrixProps) -> Html {
    let matrix_state = use_state(|| "2 2 0 0 0 0".to_owned());
    let cloned_matrix_state = matrix_state.clone();
    let matrix_changed = Callback::from(move |matrix| {
        cloned_matrix_state.set(matrix);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_matrix_state = matrix_state.clone();
    let onsubmit = Callback::from(move |event : SubmitEvent| {
        event.prevent_default();
        let data = cloned_matrix_state.clone();
        form_onsubmit.emit(data.to_string());
    });
    html! {
        <form onsubmit={onsubmit}>
            <TextInput name = "matrix"  handle_onchange={ matrix_changed}/>
            <SubmitButton label = "submit" name="calculate"/>
        </form>
    }
}


#[function_component]
fn App() -> Html {
    let matrix_state = use_state(|| "2 2 0 0 0 0".to_owned());
    let cloned_matrix_state = matrix_state.clone();
    let form_onsubmit = Callback::from(move |data| {
        cloned_matrix_state.set(data);
    });
    let mut m =Matrix::create_from_string(&*matrix_state);
    m.echelon_form();
    let res = m.to_string();
    html! {
        <div>
            <FormMatrix onsubmit={ form_onsubmit }/>
            <p>{"Matrix: "}{res}</p>
        </div>
    }
}

fn main() {
    let mut m = Matrix::new(4, 4);
    let init = vec![1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
                                    ];
    m.load_from_vector(init);
    let s = "2 2 1 0 0 1".to_owned();
    let mut x = Matrix::create_from_string(&s);
    m.echelon_form();
    println!("{}", m.to_string());
    yew::Renderer::<App>::new().render();
}

