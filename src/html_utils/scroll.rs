use web_sys::{window, ScrollBehavior, ScrollIntoViewOptions};

pub fn try_scroll_to(id: &str) {
    match window() {
        Some(window) => {
            let document = window.document().unwrap();
            if let Some(element) = document.get_element_by_id(id) {
                let mut options = ScrollIntoViewOptions::new();
                options.behavior(ScrollBehavior::Smooth);
                element.scroll_into_view_with_scroll_into_view_options(&options);
            }
        }
        None => {
            log::error!("Could not get window");
        }
    }
}