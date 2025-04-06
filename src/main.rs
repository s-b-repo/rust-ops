use eframe::{egui, App, Frame};

struct PsyopApp {
    scores: [u8; 20],
    pdf_status: Option<String>,
}

impl Default for PsyopApp {
    fn default() -> Self {
        Self {
            scores: [1; 20],
            pdf_status: None,
        }
    }
}

impl App for PsyopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PSYOPS Likelihood Assessment");

            let questions = [
                "1. Timing", "2. Emotional Manipulation", "3. Uniform Messaging", "4. Missing Information",
                "5. Simplistic Narratives", "6. Tribal Division", "7. Authority Overload", "8. Call for Urgent Action",
                "9. Overuse of Novelty", "10. Financial/Political Gain", "11. Suppression of Dissent",
                "12. False Dilemmas", "13. Bandwagon Effect", "14. Emotional Repetition", "15. Cherry-Picked Data",
                "16. Logical Fallacies", "17. Manufactured Outrage", "18. Framing Techniques", "19. Rapid Behavior Shifts",
                "20. Historical Parallels",
            ];

            for (i, question) in questions.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(*question);
                    egui::ComboBox::from_id_salt(i)
                        .selected_text(format!("{}", self.scores[i]))
                        .show_ui(ui, |ui| {
                            for score in 1..=5 {
                                ui.selectable_value(&mut self.scores[i], score, score.to_string());
                            }
                        });
                });
            }

            let total_score: u32 = self.scores.iter().map(|&x| x as u32).sum();
            let likelihood = match total_score {
                0..=25 => "Low likelihood of a PSYOP",
                26..=50 => "Moderate likelihood—look deeper",
                51..=75 => "Strong likelihood—manipulation likely",
                _ => "Overwhelming signs of a PSYOP",
            };

            ui.separator();
            ui.label(format!("Total Score: {}", total_score));
            ui.label(format!("Interpretation: {}", likelihood));

            if ui.button("Save Results as PDF").clicked() {
                match save_as_pdf(&self.scores, questions, total_score, likelihood) {
                    Ok(_) => self.pdf_status = Some("PDF saved as 'psyops_results.pdf'.".to_string()),
                    Err(e) => self.pdf_status = Some(format!("Failed to save PDF: {}", e)),
                }
            }

            if let Some(status) = &self.pdf_status {
                ui.label(status);
            }
        });
    }
}

// ✅ Working PDF generation with font-kit + genpdf
fn save_as_pdf(
    scores: &[u8],
    questions: [&str; 20],
    total_score: u32,
    interpretation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use genpdf::{Document};
    use genpdf::elements::{Paragraph, Break};
    use genpdf::style::Style;
    use genpdf::fonts;

    // ✅ Load font family from the ./fonts directory
    let font_family = fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");

    // ✅ Create document using loaded font
    let mut doc = Document::new(font_family);
    doc.set_title("PSYOPS Assessment Report");

    // Title
    let title = Paragraph::new("")
        .styled_string("PSYOPS Assessment Results", Style::new().bold().with_font_size(20));
    doc.push(title);
    doc.push(Break::new(1));

    // Question scores
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





fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "PSYOPS Likelihood Assessment",
        native_options,
        Box::new(|_cc| Ok(Box::new(PsyopApp::default()))),
    )
}

