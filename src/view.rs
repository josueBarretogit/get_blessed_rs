pub mod crate_category;
pub mod view;
pub mod widgets;

pub trait View {
    fn display_crate_categories(&self) -> String;
    // fn display_crate_options(&self) -> String;
}
