use yew::prelude::*;
use crate::table::TableItemDisplay;

pub struct StringDisplay {
    value: String,
}

impl StringDisplay {
    pub fn new(value: String) -> Self {
        StringDisplay { value }
    }
}

impl TableItemDisplay for StringDisplay {
    fn render(&self) -> Html {
        html! {
            { &self.value }
        }
    }
}
