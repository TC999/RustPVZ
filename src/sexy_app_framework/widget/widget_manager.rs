// [TRANSLATION_NOTE]: WidgetManager.h -> Rust stub

pub struct WidgetManager {
    pub m_widgets: Vec<*mut dyn crate::sexy_app_framework::widget::widget_traits::WidgetTrait>,
}

impl WidgetManager {
    pub fn new() -> Self {
        WidgetManager {
            m_widgets: Vec::new(),
        }
    }
}
