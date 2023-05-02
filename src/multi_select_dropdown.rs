use std::rc::Rc;
use yew::prelude::*;

pub trait MultiSelectDropdownItemDisplay: PartialEq + Clone + 'static {
    type Layout: Layout;
    fn render(&self) -> Html;
}

pub trait Layout {
    fn layout(items: &[Html]) -> Html;
}

pub struct ListLayout;
impl Layout for ListLayout {
    fn layout(items: &[Html]) -> Html {
        html! {
            <div class="list">
                { for items.iter().map(|item| item.to_owned()) }
            </div>
        }
    }
}

pub struct GridLayout;
impl Layout for GridLayout {
    fn layout(items: &[Html]) -> Html {
        html! {
            <div class="grid">
                { for items.iter().map(|item| item.to_owned()) }
            </div>
        }
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props<T: MultiSelectDropdownItemDisplay> {
    pub options: Vec<Rc<T>>,
    pub on_select: Callback<Vec<Rc<T>>>,
    pub max_selections: Option<usize>,
}

pub enum Msg<T> {
    Toggle(Rc<T>),
    ToggleDropdown,
}

pub struct MultiSelectDropdown<T: MultiSelectDropdownItemDisplay> {
    props: Props<T>,
    options: Vec<Rc<T>>,
    selected_options: Vec<Rc<T>>,
    on_select: Callback<Vec<Rc<T>>>,
    max_selections: Option<usize>,
    is_open: bool,
}

impl<T: MultiSelectDropdownItemDisplay> Component for MultiSelectDropdown<T> {
    type Message = Msg<T>;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        MultiSelectDropdown {
            options: ctx.props().options.clone(),
            selected_options: vec![],
            props: ctx.props().clone(),
            on_select: ctx.props().on_select.clone(),
            max_selections: ctx.props().max_selections.clone(),
            is_open: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        if self.props != *ctx.props() {
            self.props = ctx.props().clone();
            self.options = ctx.props().options.clone();
            true
        } else {
            false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle(option) => {
                if let Some(index) = self.selected_options.iter().position(|selected| Rc::ptr_eq(selected, &option)) {
                    self.selected_options.remove(index);
                } else {
                    if let Some(max_selections) = self.max_selections {
                        if self.selected_options.len() < max_selections {
                            self.selected_options.push(option);
                        }
                    } else {
                        self.selected_options.push(option);
                    }
                }
                self.on_select.emit(self.selected_options.clone());
                true
            }
            Msg::ToggleDropdown => {
                self.is_open = !self.is_open;
                true
            }
        }
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items = self.options.iter().map(|option| {
            let is_selected = self.selected_options.iter().any(|selected| Rc::ptr_eq(selected, option));
            let option_class = if is_selected { "item selected" } else { "item" };
            let link = ctx.link().clone();
            let option_clone = Rc::clone(&option);
            html! {
                <div class={option_class} onclick={link.callback(move |_| Msg::Toggle(Rc::clone(&option_clone)))}>
                    { option.render() }
                </div>
            }
        }).collect::<Vec<_>>();
    
        html! {
            <div class="multiselect-dropdown">
                <button onclick={ctx.link().callback(|_| Msg::ToggleDropdown)}>
                    { if self.is_open { "Close" } else { "Open" } }
                </button>
                { if self.is_open {
                    T::Layout::layout(&items)
                  } else {
                    html! {}
                  }
                }
            </div>
        }
    }
    
    
}

