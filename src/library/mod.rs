pub use std::collections::HashMap;
pub use gtk::{
    traits::*,
    Inhibit as Перешкодження,
    main_quit as вихід,
};
pub use gtk::Orientation::{Vertical as Вертикальна, Horizontal as Горизонтальна};
pub use relm::Widget as Віджет;
pub use relm_derive::{Msg as Повідомлення, widget as віджет};