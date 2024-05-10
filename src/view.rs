pub mod ui;
pub mod widgets;

pub trait View {
    fn display_crate_categories(&self) -> String;
    // fn display_crate_options(&self) -> String;
}
