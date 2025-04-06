use chrono::Local;
use eframe::{App, Frame};
use egui::{self, Frame as EguiFrame};
use genpdf::{
    elements::{Break, Paragraph},
    fonts, style::Style, Document,
};

#[allow(dead_code)]
#[derive(Default)]
struct PsyopApp {
    scores: [u8; 20],
    pdf_status: Option<String>,
    theme: Theme,
    animated_score: f32,
}
#[allow(dead_code)]
#[derive(Default)]
enum Theme {
    Light,
    #[default]
    Dark,
}

impl App for PsyopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("🧠 PSYOPS Likelihood Assessment");
            ui.label("Evaluate psychological operations traits on a scale of 1 to 5.");
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🖨️ Print Results").clicked() {
                    let total_score: u32 = self.scores.iter().map(|&x| x as u32).sum();
                    let (likelihood, _) = match total_score {
                        0..=25 => ("Low likelihood of a PSYOP", egui::Color32::from_rgb(0, 200, 0)),
                        26..=50 => ("Moderate likelihood—look deeper", egui::Color32::YELLOW),
                        51..=75 => ("Strong likelihood—manipulation likely", egui::Color32::LIGHT_RED),
                        _ => ("Overwhelming signs of a PSYOP", egui::Color32::RED),
                    };

                    match save_as_pdf(&self.scores, get_questions(), total_score, likelihood) {
                        Ok(_) => self.pdf_status = Some("✅ PDF saved as 'psyops_results.pdf'.".to_string()),
                        Err(e) => self.pdf_status = Some(format!("❌ Failed to save PDF: {}", e)),
                    }
                }
            });
        });

        egui::SidePanel::left("left_panel").resizable(true).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, question) in get_questions().iter().enumerate() {
                    EguiFrame::group(ui.style())
                        .fill(ui.visuals().extreme_bg_color)
                        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.inactive.bg_fill))
                        .corner_radius(8.0)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(*question).strong());
                            ui.add(
                                egui::Slider::new(&mut self.scores[i], 1..=5)
                                    .text("Score")
                                    .step_by(1.0),
                            );
                        });
                    ui.add_space(8.0);
                }

                ui.horizontal(|ui| {
                    if ui.button("🔄 Reset Scores").clicked() {
                        self.scores = [1; 20];
                    }

                    if let Some(status) = &self.pdf_status {
                        ui.label(status);
                    }
                });
            });
        });

        egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
            let total_score: u32 = self.scores.iter().map(|&x| x as u32).sum();
            let (likelihood, color) = match total_score {
                0..=25 => ("Low likelihood of a PSYOP", egui::Color32::from_rgb(0, 200, 0)),
                26..=50 => ("Moderate likelihood—look deeper", egui::Color32::YELLOW),
                51..=75 => ("Strong likelihood—manipulation likely", egui::Color32::LIGHT_RED),
                _ => ("Overwhelming signs of a PSYOP", egui::Color32::RED),
            };

            egui::Frame::group(ui.style())
                .fill(ui.visuals().extreme_bg_color)
                .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.inactive.bg_fill))
                .corner_radius(8.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("📊 Total Score").heading());
                        ui.label(format!("{}", total_score));
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("🧾 Interpretation").heading());
                        ui.colored_label(color, likelihood);
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("📈 Score Progress").heading());

                let target_score: f32 = self.scores.iter().map(|&x| x as f32).sum();
                let speed = 5.0;

                if (self.animated_score - target_score).abs() > 0.1 {
                    self.animated_score += (target_score - self.animated_score) / speed;
                    ctx.request_repaint();
                } else {
                    self.animated_score = target_score;
                }

                draw_circular_score_graph(ui, self.animated_score);
            });
        });
    }
}

fn draw_circular_score_graph(ui: &mut egui::Ui, total_score: f32) {
    use egui::{Color32, Pos2, Stroke, Vec2};

    let max_score = 100.0;
    let progress = total_score.clamp(0.0, max_score) / max_score;

    let (rect, _) = ui.allocate_exact_size(Vec2::splat(200.0), egui::Sense::hover());
    let painter = ui.painter();
    let center = rect.center();
    let radius = rect.width().min(rect.height()) / 2.0 - 10.0;
    let start_angle = std::f32::consts::FRAC_PI_2 * 3.0;
    let sweep_angle = std::f32::consts::TAU * progress;

    let color = match total_score as u32 {
        0..=25 => Color32::from_rgb(0, 200, 0),
        26..=50 => Color32::YELLOW,
        51..=75 => Color32::LIGHT_RED,
        _ => Color32::RED,
    };

    painter.circle_stroke(center, radius, Stroke::new(8.0, Color32::DARK_GRAY));

    let steps = 100;
    for i in 0..steps {
        let t1 = i as f32 / steps as f32;
        let t2 = (i + 1) as f32 / steps as f32;

        let angle1 = start_angle + sweep_angle * t1;
        let angle2 = start_angle + sweep_angle * t2;

        let point1 = Pos2::new(center.x + angle1.cos() * radius, center.y + angle1.sin() * radius);
        let point2 = Pos2::new(center.x + angle2.cos() * radius, center.y + angle2.sin() * radius);

        painter.line_segment([point1, point2], Stroke::new(8.0, color));
    }

    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        format!("{:.0} / 100", total_score),
        egui::FontId::proportional(20.0),
        color,
    );
}

fn save_as_pdf(
    scores: &[u8],
    questions: [&str; 20],
    total_score: u32,
    interpretation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // You MUST include a TTF file in your ./fonts/ folder
       // ✅ Load font family from the ./fonts directory
    let font_family = fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");

    // ✅ Create document using loaded font
    let mut doc = Document::new(font_family);
    doc.set_title("PSYOPS Assessment Report");

    doc.push(
        Paragraph::new("")
            .styled_string("PSYOPS Assessment Results", Style::new().bold().with_font_size(20)),
    );
    doc.push(Paragraph::new(format!(
        "Generated on: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    )));
    doc.push(Break::new(1));

    for (i, question) in questions.iter().enumerate() {
        let line = format!("{} - Score: {}", question, scores[i]);
        doc.push(Paragraph::new(line));
    }

    doc.push(Break::new(1));
    doc.push(Paragraph::new(format!("Total Score: {}", total_score)));
    doc.push(Paragraph::new(format!("Interpretation: {}", interpretation)));

    let mut file = std::fs::File::create("psyops_results.pdf")?;
    doc.render(&mut file)?;

    Ok(())
}


fn get_questions() -> [&'static str; 20] {
    [
        "1. Timing", "2. Emotional Manipulation", "3. Uniform Messaging", "4. Missing Information",
        "5. Simplistic Narratives", "6. Tribal Division", "7. Authority Overload", "8. Call for Urgent Action",
        "9. Overuse of Novelty", "10. Financial/Political Gain", "11. Suppression of Dissent",
        "12. False Dilemmas", "13. Bandwagon Effect", "14. Emotional Repetition", "15. Cherry-Picked Data",
        "16. Logical Fallacies", "17. Manufactured Outrage", "18. Framing Techniques", "19. Rapid Behavior Shifts",
        "20. Historical Parallels",
    ]
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "PSYOPS Likelihood Assessment",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(PsyopApp::default()))
        }),
    )
}
