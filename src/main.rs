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
    button_name: String,
    input_name: String,
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
            <TextInput name = { props.input_name.clone() }  handle_onchange={ matrix_changed}/>
            <SubmitButton label = {props.button_name.clone() } name={ props.button_name.clone()}/>
        </form>
    }
}

pub enum Msg {
    Reduce(Matrix),
    Invert(Matrix),
    Determinant(Matrix),
    Transpose(Matrix),
    Rank(Matrix),
}

enum AnswerState{
    Exists(Matrix),
    Scalar(f64),
    USize(usize),
    DoesNotExist,
    Empty,
}
struct MatrixCalc{
    state: AnswerState,
}
impl Component for MatrixCalc{
    type Message = Msg;
    type Properties  = ();
    fn create(ctx: &Context<Self>) -> Self {
        let state = AnswerState::Empty;
        Self{
            state: state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reduce(m) => {
                let mut m_clone = m.clone();
                m_clone.echelon_form();
                self.state = AnswerState::Exists(m_clone);
            },
            Msg::Transpose(m) =>{
                let m_trans = m.transpose();
                self.state = AnswerState::Exists(m_trans);
            },
            Msg::Invert(m) =>{
                let m_inv= m.inverse();
                match m_inv {
                    Some(m) =>{
                        self.state = AnswerState::Exists(m);
                    }
                    None =>{
                        self.state = AnswerState::DoesNotExist;
                    }
                }
            },
            Msg::Determinant(m) =>{
                let det = m.determinant(); 
                match det {
                    Some(n) =>{
                        self.state = AnswerState::Scalar(n);
                    }
                    None => {
                        self.state = AnswerState::DoesNotExist;
                    }
                }
            }
            Msg::Rank(m) =>{
                let res = m.rank();
                self.state = AnswerState::USize(res);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ech_form_onsubmit = ctx.link().callback(move |data: String| {
            let mat = Matrix::create_from_string(&(&*data).to_string());
            Msg::Reduce(mat)
        });
        let inv_form_onsubmit = ctx.link().callback(move |data: String| {
            let mat = Matrix::create_from_string(&(&*data).to_string());
            Msg::Invert(mat)
        });
        let det_form_onsubmit = ctx.link().callback(move |data: String| {
            let mat = Matrix::create_from_string(&(&*data).to_string());
            Msg::Determinant(mat)
        });
        let trans_form_onsubmit = ctx.link().callback(move |data: String| {
            let mat = Matrix::create_from_string(&(&*data).to_string());
            Msg::Transpose(mat)
        });
        let rank_form_onsubmit = ctx.link().callback(move |data: String| {
            let mat = Matrix::create_from_string(&(&*data).to_string());
            Msg::Rank(mat)
        });
        html!{
            <div>
                <FormMatrix button_name = {"reduce"} input_name ={"matrix1"} onsubmit={ ech_form_onsubmit }/>
                <FormMatrix button_name = {"inverse"} input_name ={"matrix2"} onsubmit={ inv_form_onsubmit }/>
                <FormMatrix button_name = {"determinant"} input_name ={"matrix3"} onsubmit={ det_form_onsubmit }/>
                <FormMatrix button_name = {"transpose"} input_name ={"matrix4"} onsubmit={ trans_form_onsubmit }/>
                <FormMatrix button_name = {"rank"} input_name ={"matrix5"} onsubmit={ rank_form_onsubmit }/>
                <div class="answer-field">
                { 
                    match &self.state {
                        AnswerState::Exists(m) =>{
                            html! {
                                <MatrixComp matrix={m.clone()}/>
                            }
                        },
                        AnswerState::Scalar(n) =>{
                            html!{
                                <p>{ n }</p>
                            }
                        },
                        AnswerState::DoesNotExist => {
                            html!{
                                <p>{ "This Answer Does Not Exist" }</p>
                            }
                        },
                        AnswerState::USize(n) =>{
                            html!{
                                <p>{ n }</p>
                            }
                        },
                        AnswerState::Empty => {
                            html!{
                                <></>
                            }
                        },
                    }
                }
            </div>
            </div>
        }
        /*
        match &self.state {
            AnswerState::Exists(m) =>{
                html! {
                    <div>
                        <FormMatrix button_name = {"reduce"} onsubmit={ form_onsubmit }/>
                        <MatrixComp matrix={m.clone()}/>
                    </div>
                }
            },
            AnswerState::Scalar(n) =>{
                html!{
                    <div>
                        <FormMatrix button_name = {"reduce"} onsubmit={ form_onsubmit }/>
                        <p>{ n }</p>
                    </div>
                }
            },
            AnswerState::DoesNotExist => {
                html!{
                    <div>
                        <FormMatrix button_name = {"reduce"} onsubmit={ form_onsubmit }/>
                    </div>
                }
            },
            AnswerState::Empty => {
                html!{
                    <div>
                        <FormMatrix button_name = {"reduce"} onsubmit={ form_onsubmit }/>
                    </div>
                }
            },
        }
        */
    }


}
fn main() {
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
