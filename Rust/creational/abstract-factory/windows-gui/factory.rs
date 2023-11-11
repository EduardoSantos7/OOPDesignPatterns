use gui::{Button, Checkbox, GuiFactory};

use crate::{button::WindowsButton, checkbox::WindowsCheckbox};

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> WindowsButton {
        WindowsButton
    }

    fn create_checkbox(&self) -> WindowsCheckbox {
        WindowsCheckbox
    }
}