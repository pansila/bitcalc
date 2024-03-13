#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::widgets::text_edit::TextBuffer;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
enum DigitWidth {
    DW8 = 1,
    DW16 = 2,
    DW32 = 4,
    DW64 = 8,
}

impl DigitWidth {
    fn get_name(self) -> String {
        match self {
            DigitWidth::DW64 => String::from("64 Bits"),
            DigitWidth::DW32 => String::from("32 Bits"),
            DigitWidth::DW16 => String::from("16 Bits"),
            DigitWidth::DW8 => String::from("8 Bits"),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1580.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut digit_mum: DigitWidth = DigitWidth::DW32;
    let mut input: String = String::from("0");

    eframe::run_simple_native("Bit Calculator", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for d in DigitWidth::iter() {
                    if ui.radio(digit_mum == d, d.get_name()).clicked() {
                        digit_mum = d;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Bit:");
                egui::Frame::group(ui.style())
                    // .stroke(egui::Stroke {
                    //     width: 1.,
                    //     color: egui::Color32::BLACK,
                    // })
                    .show(ui, |ui| {
                        let groups = digit_mum as usize;

                        // ui.spacing_mut().item_spacing = egui::Vec2::from([ui.spacing().item_spacing.x, 0.]);
                        for i in (0..groups).rev() {
                            // ui.spacing_mut().item_spacing = egui::Vec2::from([ui.spacing().item_spacing.x, 0.]);
                            // ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                            egui::Grid::new(format!("grid{i}"))
                                .min_col_width(15.)
                                .spacing(egui::Vec2::ZERO)
                                .show(ui, |ui| {
                                    for j in (i * 8..i * 8 + 8).rev() {
                                        let btn = egui::Button::new(
                                            egui::RichText::from(format!("{j:2}")).monospace(),
                                        );
                                        let btn = btn.stroke(egui::Stroke {
                                            width: 1.,
                                            color: egui::Color32::DARK_GRAY,
                                        });
                                        let btn = btn.fill(egui::Color32::LIGHT_BLUE);
                                        let _ = ui.add(btn);
                                    }
                                    ui.end_row();

                                    for j in (i * 8..i * 8 + 8).rev() {
                                        let mut val = 0;
                                        if input.starts_with("0x") {
                                            if let Ok(v) = usize::from_str_radix(
                                                input.char_range(2..input.len()),
                                                16,
                                            ) {
                                                val = v;
                                            }
                                        } else if let Ok(v) = input.parse::<usize>() {
                                            val = v;
                                        }
                                        let v = val & (0x1 << j);
                                        let v = (v != 0) as usize;

                                        let btn = egui::Button::new(
                                            egui::RichText::from(format!("{v:2}")).monospace(),
                                        );
                                        let btn = btn.stroke(egui::Stroke {
                                            width: 1.,
                                            color: egui::Color32::DARK_GRAY,
                                        });
                                        if ui.add(btn).clicked() {
                                            let new_val = val ^ (1 << j);
                                            if input.starts_with("0x") {
                                                if input.contains(|c: char| c.is_ascii_lowercase())
                                                {
                                                    input = format!("{new_val:#x}");
                                                } else {
                                                    input = format!("{new_val:#X}");
                                                }
                                            } else {
                                                input = format!("{new_val}");
                                            }
                                        }
                                    }
                                    ui.end_row();
                                });
                        }
                    });
            });
            ui.text_edit_singleline(&mut input);
        });
    })
}
