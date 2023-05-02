//rainfrog/src/dropdown.rs
use std::rc::Rc;
use yew::prelude::*;

pub enum Msg<T> {
    Select(Rc<T>),
    Toggle,
}

pub struct Dropdown<T: DropdownItemDisplay> {
    options: Vec<Rc<T>>,
    selected_index: Option<usize>,
    on_select: Callback<Rc<T>>,
    is_open: bool,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props<T: DropdownItemDisplay> {
    pub options: Vec<Rc<T>>,
    pub on_select: Callback<Rc<T>>,
}


impl<T: DropdownItemDisplay> Component for Dropdown<T> {
    type Message = Msg<T>;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Dropdown {
            options: ctx.props().options.clone(),
            selected_index: None,
            on_select: ctx.props().on_select.clone(),
            is_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(option) => {
                if let Some(index) = self.options.iter().position(|opt| Rc::ptr_eq(opt, &option)) {
                    self.selected_index = Some(index);
                    self.on_select.emit(option.clone());
                    self.is_open = false;
                }
                true
            }
            Msg::Toggle => {
                self.is_open = !self.is_open;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, props: &Self::Properties) -> bool {
        self.options = props.options.clone();
        self.on_select = props.on_select.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected_text = match &self.selected_index {
            Some(selected) => self.options[*selected].render(),
            None => html! { "Select Option" },
        };

        let options = self.options.iter().enumerate().map(|(_, option)| {
            let option_clone = option.clone();
            html! {
                <div onclick={ctx.link().callback(move |_| Msg::Select(option_clone.clone()))}>
                    { option.render() }
                </div>
            }
        });

        let dropdown_class = if self.is_open {
            "dropdown is-open"
        } else {
            "dropdown"
        };

        html! {
            <div class={dropdown_class}>
                <div class="dropdown-selected" onclick={ctx.link().callback(move |_| Msg::Toggle)}>
                    { selected_text }
                </div>
                { if self.is_open {
                    html! { <div class="dropdown-options"> { for options } </div> }
                  } else {
                    html! {}
                  }
                }
            </div>
        }
    }
}


pub trait DropdownItemDisplay: PartialEq + Clone + 'static {
    fn render(&self) -> Html;
}
