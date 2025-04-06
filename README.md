# 🧠 PSYOPS Likelihood Assessment Tool

An interactive Rust-based desktop application to evaluate the likelihood of psychological operations (PSYOPS) influence in messaging, media, or behaviors. This tool provides a visual score breakdown, allows exporting results to PDF, and is built using `eframe`, `egui`, and `genpdf`.

---
## credits
https://www.youtube.com/@chasehughesofficial

https://www.youtube.com/watch?v=b3AN2wY4qAM

## 📖 What Are PSYOPS?

**Psychological Operations (PSYOPS)** are operations intended to convey selected information and indicators to audiences to influence their emotions, motives, reasoning, and behavior. PSYOPS are widely used in:

- Information warfare
- Propaganda dissemination
- Political manipulation
- Social media influence campaigns

Common characteristics include emotional manipulation, repetition of simplistic narratives, appeals to authority, suppression of dissent, and urgent calls to action.

---

## 🛠️ Features

- ✅ GUI-based Likelihood Assessment
- ✅ Interactive slider-based scoring
- ✅ Animated score graph (circular)
- ✅ PDF export of full results
- ✅ Reset/Replay capability
- ✅ Offline-capable, 100% local
- ✅ Dark mode by default

---

## 📸 Screenshots

Here's a look at the PSYOPS Assessment Tool UI:

![Preview](https://github.com/s-b-repo/rust-ops/raw/main/Screenshot_20250406_222548.png)

---

## 🚀 Usage

1. Launch the application.
2. You will see **20 traits/questions** related to psychological manipulation.
3. Rate each trait on a scale from **1 (Not Present)** to **5 (Highly Present)**.
4. Your **total PSYOPS score** will be calculated and visualized with an animated circular graph.
5. Click **"🖨️ Print Results"** to export a detailed PDF with your scores and interpretation.
6. Click **"🔄 Reset Scores"** to start over.

---

## 📦 Installation

### 🧱 Requirements

- Rust (stable)
- Cargo
- Linux/macOS/Windows
- A TTF font file (e.g., `LiberationSans-Regular.ttf`)

### 🏗️ Build Instructions


# Clone the repository
git clone https://github.com/s-b-repo/rust-ops.git
cd psyops-assessment

# Build and run
cargo run --release

📄 PDF Output

The tool generates a clean PDF with:

    Timestamp

    Full question list and scores

    Total score

    Interpretation of the result (Low → Overwhelming)

Saved as: psyops_results.pdf
🧠 Scoring Interpretation
Score Range	Interpretation
0–25	Low likelihood of a PSYOP
26–50	Moderate – look deeper
51–75	Strong – manipulation likely
76–100	Overwhelming signs of a PSYOP
🌑 Theming

The UI uses Dark Mode by default for eye comfort and focus. Light mode support is coming soon.
📚 Technologies Used

    eframe – GUI Framework (based on egui)

    egui – Immediate mode GUI

    genpdf – PDF generation

    chrono – Time formatting

🔐 Privacy & Security

This application is offline-first, and does not collect or transmit any data. All assessment and PDF generation happens locally on your machine.
🤖 Future Enhancements

🙌 Contributing

Pull requests are welcome! Please fork the repository and open a PR.

    Clone the project

    Create a new branch

    Commit your changes

    Open a Pull Request



