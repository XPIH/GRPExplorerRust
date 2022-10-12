use super::*;

pub struct BlankEditor;

impl EditorImpl for BlankEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        ui.label("editor not implemented yet!");
        EditorResponse::default()
    }
}