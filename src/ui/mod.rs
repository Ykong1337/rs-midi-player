use std::sync::atomic::Ordering;

use eframe::{App, egui, Frame};
use eframe::egui::{Context, Ui};

use crate::ui::play::Play;

pub mod play;

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}

pub trait Module: View {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &Context, open: &mut bool);
}

impl App for Play<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default()
            .show(ctx, |ui| self.ui(ui));
        egui::Window::new("音轨").open(&mut self.tracks_enable).show(ctx, |ui| {
            for (enable, index) in self.midi.track_num.lock().unwrap().iter_mut() {
                if ui.checkbox(enable, format!("Track {}", index)).changed() {
                    self.notify_merge = true;
                }
            }
            if self.notify_merge {
                let range = self.midi.track_num.lock().unwrap().iter().filter_map(|(enable, index)| {
                    if *enable {
                        Some(*index)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>();
                self.midi.merge_tracks(&range);
                self.midi.hit_rate.store(self.midi.detection(self.offset), Ordering::Relaxed);
                self.notify_merge = false;
            }
        });
    }
}