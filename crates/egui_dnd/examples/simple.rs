use eframe::{egui, NativeOptions};
use egui::{CentralPanel, CollapsingHeader, Rect, Scene};
use egui_dnd::dnd;

pub fn main() -> eframe::Result<()> {
    let mut items = vec!["alfred", "bernhard", "christian"];

    let mut rect = Rect::ZERO;

    eframe::run_simple_native(
        "DnD Simple Example",
        NativeOptions::default(),
        move |ctx, _frame| {
            CentralPanel::default().show(ctx, |ui| {
                    dnd(ui, "dnd_example").with_animation_time(1.0).show_vec(&mut items, |ui, item, handle, state| {
                        ui.horizontal(|ui| {
                            handle.ui(ui, |ui| {
                                if state.dragged {
                                    ui.label("dragging");
                                } else {
                                    ui.label("drag");
                                }
                            });

                            CollapsingHeader::new(format!("OCR Pipeline Config {}",  item)).show_unindented(ui, |ui| {
                                if state.dragged {
                                    dbg!("dragging");
                                }
                                ui.label(*item);
                                ui.label(*item);
                                ui.label(*item);
                                ui.label(*item);
                                ui.label(*item);
                            });
                        });
                });
            });
        },
    )
}
