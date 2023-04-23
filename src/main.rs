use yew::prelude::*;
mod matrices;
use crate::matrices::Matrix;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;


#[derive(Properties, PartialEq)]
struct MatrixProps{
   matrix: Matrix,
}
#[function_component(MatrixComp)]
fn matrix_component(props: &MatrixProps) -> Html{
    let rows = props.matrix.get_rows();
    html!{
        <div class="matrix-table">
            <p>{"("}</p>
            <table>
            {
                rows.into_iter().map(|row|{
                    html!{
                        <tr>
                        {
                            row.into_iter().map(|val| {
                                html!{<th> {val } </th>}
                            }).collect::<Html>()
                        }
                        </tr>
                    }
                }).collect::<Html>()
            }
            </table>
            <p>{")"}</p>
        </div>
    }
}

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
            <SubmitButton label = "Reduce" name="calculate"/>
        </form>
    }
}

/*
#[function_component]
fn App() -> Html {
    let matrix_state = use_state(|| "2 2 0 0 0 0".to_owned());
    let cloned_matrix_state = matrix_state.clone();
    let form_onsubmit = Callback::from(move |data| {
        cloned_matrix_state.set(data);
    });
    let mut m =Matrix::create_from_string(&*matrix_state);
    m.echelon_form();
    html! {
        <div>
            <FormMatrix onsubmit={ form_onsubmit }/>
            <MatrixComp matrix={m}/>
        </div>
    }
}
*/
pub enum Msg {
    Reduce(Matrix),
}

enum MatrixState{
    Exists(Matrix),
    Empty,
}
struct MatrixCalc{
    state: MatrixState,
}
impl Component for MatrixCalc{
    type Message = Msg;
    type Properties  = ();
    fn create(ctx: &Context<Self>) -> Self {
        let state = MatrixState::Empty;
        Self{
            state: state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reduce(m) => {
                let mut m_clone = m.clone();
                m_clone.echelon_form();
                self.state = MatrixState::Exists(m_clone);
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let form_onsubmit = ctx.link().callback(move |data: String| {
            let mat = Matrix::create_from_string(&(&*data).to_string());
            Msg::Reduce(mat)
        });
        match &self.state {
            MatrixState::Exists(m) =>{
                html! {
                    <div>
                        <FormMatrix onsubmit={ form_onsubmit }/>
                        <MatrixComp matrix={m.clone()}/>
                    </div>
                }
            },
            MatrixState::Empty => {
                html!{
                    <div>
                        <FormMatrix onsubmit={ form_onsubmit }/>
                    </div>
                }
            },
        }
    }


}
fn main() {
    /*let mut m = Matrix::new(4, 4);
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
    */
    yew::Renderer::<MatrixCalc>::new().render();
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_echelon_form() {
        let mut m = Matrix::new(4, 4);
        let init = vec![1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
                ];
        m.load_from_vector(init);
        let s = "4 4 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1".to_owned();
        let x = Matrix::create_from_string(&s);
        m.echelon_form();
        assert_eq!(m, x);
    }

    #[test]
    fn test_identity(){
        let mut m = Matrix::new(4, 4);
        let init = vec![1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
                ];
        m.load_from_vector(init);
        assert_eq!(m, Matrix::identity(4));
    }

    #[test]
    fn test_deteminant(){
        let mut m = Matrix::new(4, 4);
        let init = vec![1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
                ];
        m.load_from_vector(init);
        assert_eq!(m.determinant(), Option::Some(1.0));
    }
    #[test]
    fn test_deteminant_fail(){
        let mut m = Matrix::new(2, 2);
        let init = vec![1.0, 1.0, 1.0, 1.0,];
        m.load_from_vector(init);
        assert_eq!(m.determinant(), Option::Some(0.0));
    }
    #[test]
    fn test_inverse(){
        let mut m = Matrix::new(4, 4);
        let init = vec![-1.0, 0.0, 0.0, 0.0,
                0.0, -1.0, 0.0, 0.0,
                0.0, 0.0, -1.0, 0.0,
                0.0, 0.0, 0.0, -1.0
                ];
        m.load_from_vector(init);
        let mut i  = Matrix::new(4, 4);
        let init = vec![-1.0, 0.0, 0.0, 0.0,
                0.0, -1.0, 0.0, 0.0,
                0.0, 0.0, -1.0, 0.0,
                0.0, 0.0, 0.0, -1.0
                ];
        i.load_from_vector(init);
        assert_eq!(m.inverse(), Option::Some(i));
    }
}
