use std::{rc::Rc, ops::Deref};
use yew::prelude::*;
use log::debug;



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

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct Props<T: MultiSelectDropdownItemDisplay> {
    pub options: Vec<Rc<T>>,
    pub on_select: Callback<Vec<Rc<T>>>,
    pub max_selections: Option<usize>,
}

pub enum Msg<T> {
    MultiItemToggle(Rc<T>),
    ToggleMultiDropdown,
}
#[derive(Debug)]
pub struct MultiSelectDropdown<T: MultiSelectDropdownItemDisplay> {
    props: Props<T>,
    options: Vec<Rc<T>>,
    selected_options: Vec<Rc<T>>,
    on_select: Callback<Vec<Rc<T>>>,
    max_selections: Option<usize>,
    is_open: bool,
}


impl<T: MultiSelectDropdownItemDisplay + std::fmt::Debug> Component for MultiSelectDropdown<T> {
    type Message = Msg<T>;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        debug!("Test Debug");
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
            // Update the selected_options
            self.selected_options.retain(|selected_option| {
            self.options.iter().any(|option| *option.deref() == *selected_option.deref())
        });
            true
        } else {
            false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MultiItemToggle(option) => {
                debug!("MultiItemToggle Message");
                if let Some(index) = self.selected_options.iter().position(|selected| *selected.deref() == *option.deref()) {                    debug!("Option {:?} Deselected", self.selected_options.get(index));
                    self.selected_options.remove(index);
                } else {
                    if let Some(max_selections) = self.max_selections {
                        if self.selected_options.len() < max_selections {
                            debug!("Selection {:?} Pushed", option);
                            self.selected_options.push(option);
                        }
                    } else {
                        debug!("Selection {:?} Pushed", option);
                        self.selected_options.push(option);
                    }
                }
            
                self.on_select.emit(self.selected_options.clone());
                true
            }
            Msg::ToggleMultiDropdown => {
                self.is_open = !self.is_open;
                true
            }
        }
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        let create_option = |option: &Rc<T>, ctx: &Context<Self>| {
            let is_selected = self.selected_options.iter().any(|selected| *selected.deref() == *option.deref());
            let mut option_class = vec!["item"];
            if is_selected {
                option_class.push("selected");
            }
            let option_clone = Rc::clone(&option);
            html! {
                <div class={option_class.join(" ")} onclick={ctx.link().callback(move |_| Msg::MultiItemToggle(Rc::clone(&option_clone)))}>
                    { option.render() }
                </div>
            }
        };
    
        let items = self.options.iter().map(|option| create_option(option, ctx)).collect::<Vec<_>>();
    
        html! {
            <div class="multiselect-dropdown">
                <button onclick={ctx.link().callback(|_| Msg::ToggleMultiDropdown)}>
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

pub trait MultiSelectDropdownItemDisplay: PartialEq + Clone + 'static {
    type Layout: Layout;
    fn render(&self) -> Html;
}

pub trait Layout {
    fn layout(items: &[Html]) -> Html;
}