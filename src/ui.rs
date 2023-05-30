use std::sync::atomic::Ordering;
use eframe::{App, CreationContext, egui, Frame};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::{Context, FontId, Slider};
use eframe::egui::TextStyle::*;
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;
use crate::convert::convert_from_midi;
use crate::font::load_fonts;
use crate::midi::{GEN_SHIN, init, IS_PLAY, Midi, PAUSE, playback, SPEED, VR_CHAT};

#[derive(Debug, Clone)]
pub struct Play {
    midi: Midi,
    tuned: bool,
    speed: f64,
    mode: i32,
    state: String,
}

impl Play {
    pub fn new(cc: &CreationContext) -> Self {
        load_fonts(&cc.egui_ctx);
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);

        Self {
            midi: Midi::new(),
            tuned: false,
            speed: 1.0,
            mode: 0,
            state: format!("已停止"),
        }
    }
}

impl App for Play {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lyred");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("选择MIDI文件");
                if ui.button("打开").clicked() {
                    IS_PLAY.store(false, Ordering::Relaxed);
                    PAUSE.store(false, Ordering::Relaxed);
                    init(self.midi.clone());
                }
                if ui.button("从MIDI转换").clicked() {
                    if let Some(path) = self.midi.file_name.lock().unwrap().as_ref() {
                        let name = path.file_name().unwrap().to_string_lossy().into_owned();
                        convert_from_midi(name, self.midi.clone());
                    }
                }
            });
            if let Some(path) = self.midi.clone().file_name.lock().unwrap().as_ref() {
                ui.label(&format!("当前文件: {}", path.to_str().unwrap()));
            }
            ui.separator();
            ui.label("选择模式");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.mode, GEN_SHIN, "GenShin");
                ui.radio_value(&mut self.mode, VR_CHAT, "VRChat-中文吧");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(Slider::new(&mut self.speed, 0.1..=5.0).prefix("播放速度: "));
                if ui.button("还原").clicked() {
                    self.speed = 1.0;
                }
            });
            SPEED.store(self.speed, Ordering::Relaxed);
            ui.horizontal(|ui| {
                if ui.button("减速0.1x").clicked() {
                    if SPEED.load(Ordering::Relaxed) > 0.1 {
                        self.speed -= 0.1;
                        SPEED.store(self.speed, Ordering::Relaxed);
                    }
                }
                if ui.button("加速0.1x").clicked() {
                    self.speed += 0.1;
                    SPEED.store(self.speed, Ordering::Relaxed);
                }
            });
            ui.checkbox(&mut self.tuned, "开启自动调音");
            ui.separator();
            ui.label(&self.state);
            ui.separator();
            ui.label("按下Space键开始播放 | 继续播放");
            ui.label("按下Backspace键暂停播放");
            ui.label("按下Ctrl键停止播放");

            if get_global_keystate(VKey::Space) {
                PAUSE.store(false, Ordering::Relaxed);
                if !IS_PLAY.load(Ordering::Relaxed) {
                    IS_PLAY.store(true, Ordering::Relaxed);
                    playback(self.midi.clone(), self.tuned, self.mode);
                }
            }
            if get_global_keystate(VKey::Control) {
                PAUSE.store(false, Ordering::Relaxed);
                IS_PLAY.store(false, Ordering::Relaxed);
            }
            if get_global_keystate(VKey::Back) {
                if !PAUSE.load(Ordering::Relaxed) {
                    PAUSE.store(true, Ordering::Relaxed);
                }
            }
            if IS_PLAY.load(Ordering::Relaxed) && !PAUSE.load(Ordering::Relaxed) {
                self.state = format!("播放中...");
            }
            if !IS_PLAY.load(Ordering::Relaxed) {
                self.state = format!("已停止");
            }
            if PAUSE.load(Ordering::Relaxed) && IS_PLAY.load(Ordering::Relaxed) {
                self.state = format!("已暂停");
            }
        });
    }
}