//rainfrog/src/table.rs

use yew::prelude::*;
use std::rc::Rc;

// Traits for Tableable Items and Tableable Structures
pub trait TableBlocks: PartialEq + Clone + 'static {
    fn headers() -> Vec<String>;
    fn fields(&self) -> Vec<Box<dyn TableItemDisplay>>;
}

pub trait TableItemDisplay {
    fn render(&self) -> Html;
}

pub enum Msg<T> {
    Update(Rc<Vec<T>>),
}

pub struct Table<T: TableBlocks> {
    content: Vec<T>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: TableBlocks> {
    pub content: Vec<T>,
}

impl <T: TableBlocks> Component for Table<T> {
    type Message = Msg<T>;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Table {
            content: ctx.props().content.clone(),
        }
    }
    

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(update_content) => {
                self.content = update_content.to_vec();
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, props: &Self::Properties) -> bool {
        self.content = props.content.clone();
        true
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let headers = T::headers()
            .into_iter()
            .map(|header| html! { <th> {header} </th> })
            .collect::<Html>();
    
        let rows = self.content.iter().map(|item| {
            let fields = item.fields().into_iter().map(|field| {
                let content = field.render();
                html! { <td> {content} </td> }
            }).collect::<Html>();
            html! { <tr> {fields} </tr> }
        }).collect::<Html>();
    
        html! {
            <table>
                <thead>
                    <tr>
                        { headers }
                    </tr>
                </thead>
                <tbody>
                    { rows }
                </tbody>
            </table>
        }
    }  
}
