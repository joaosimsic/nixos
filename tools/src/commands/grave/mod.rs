pub mod parser;
pub mod ui;
pub mod zellij;

pub fn run() {
    if let Some(selected_session) = ui::start_selector() {
        zellij::attach_session(&selected_session);
    }
}
