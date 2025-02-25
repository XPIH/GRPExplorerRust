use std::collections::HashMap;

use super::*;

#[derive(Default)]
pub struct WorldEditor {
    mat_map: Option<HashMap<YKey, Vec<YKey>>>,
    display_order: Vec<YKey>,
}

impl EditorImpl for WorldEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, tctx: &EditorTabContext) {
        if let None = self.mat_map {
            let mut mat_map = HashMap::new();
            for mat in tctx.load_set.loaded_by_type(ObjectType::mat).unwrap() {
                let shd_key = ectx.bf.object_table[mat].references.last().unwrap();
                mat_map.entry(*shd_key).or_insert(vec![*mat]).push(*mat);
            }
            self.display_order = mat_map.keys().map(|b| *b).collect::<Vec<YKey>>();
            self.display_order.sort_by(|a, b| {
                let a = ectx.bf.file_table[a].get_name();
                let b = ectx.bf.file_table[b].get_name();
                a.cmp(b)
            });
            self.mat_map = Some(mat_map);
        }

        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }

        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            for shd in &self.display_order {
                ui.collapsing(format!("{:#010X} {}", shd, ectx.bf.file_table[shd].get_name_ext()), |ui| {
                    if ui.button("Open shader...").clicked() {
                        ectx.respond(EditorResponse::OpenNewTab(*shd));
                    }
    
                    for mat in &self.mat_map.as_ref().unwrap()[shd] {
                        if ui.button(format!("{:#010X} {}", mat, ectx.bf.file_table[mat].get_name_ext())).clicked() {
                            ectx.respond(EditorResponse::OpenNewTab(*mat));
                        }
                    }
                });
            }
        });
    }
}