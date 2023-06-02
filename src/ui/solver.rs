use egui::plot;

use crate::second_linear_constant::initial_value as iv;
use crate::second_linear_constant::boundary_value as bv;
use crate::second_linear_constant::solution as sl;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// Partial Equal and Equal required by egui
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
enum SolutionType
{
    LinearConstant,
    NotYet,
}

impl Default for SolutionType
{
    fn default() -> Self
    {
        Self::LinearConstant
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Solver
{
    solution_type: SolutionType,
    
    #[serde(skip)]
    problem: iv::Problem,
    #[serde(skip)]
    solution: sl::Solution,
}

impl Solver
{
    pub fn new() -> Self
    {
        Self::default()
    }
    pub fn draw(&mut self, ctx: &egui::Context)
    {
        egui::SidePanel::left("Settings").show(ctx, |ui|
        {
           egui::ComboBox::from_label("Choisisez le type d'équation à résoudre :")
            .selected_text
            (
                match self.solution_type
                {
                    SolutionType::LinearConstant => "y'' + ay' + by = 0",
                    SolutionType::NotYet => "Pas d'autre pour l'instant :(",
                }
            )
            .show_ui(ui, |ui|
            {
                ui.selectable_value(&mut self.solution_type, SolutionType::LinearConstant, "y'' + ay' + by = 0");
                ui.selectable_value(&mut self.solution_type, SolutionType::NotYet, "Pas d'autre pour l'instant :(");
            });
            
            ui.label(format!("y'' + {}y' + {}y = 0", self.problem.a, self.problem.b));
            ui.add(egui::Slider::new(&mut self.problem.a, 0.0..=100.0).text("a"));
            ui.add(egui::Slider::new(&mut self.problem.b, 0.0..=100.0).text("b"));
            ui.add(egui::Slider::new(&mut self.problem.y0, 0.0..=100.0).text("y(0)"));
            ui.add(egui::Slider::new(&mut self.problem.y0_prime, 0.0..=100.0).text("y'(0)"));
            
            if ui.add(egui::Button::new("Résoudre")).clicked()
            {
                self.solution = sl::Solution::from(&self.problem);
            }
        });
        
        egui::CentralPanel::default().show(ctx, |ui|
        {
            let solution_graph: plot::PlotPoints = (0..1000).map(|i|
            {
                let x = i as f64 * 0.01;
                [x, self.solution.y(x)]
            }).collect();
            let line = plot::Line::new(solution_graph);
            plot::Plot::new("Solution").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
        });

        egui::TopBottomPanel::bottom("Footer").show(ctx, |ui|
        {
            egui::warn_if_debug_build(ui);
        });

    }
}

impl Default for Solver
{
    fn default() -> Self
    {
        let problem = iv::Problem{ a: 4.0, b: 3.0, y0: 13.0, y0_prime: 11.5 };
        let solution = sl::Solution::from(&problem);
    
        Self
        {
            problem,
            solution,
            solution_type: SolutionType::default(),
        }
    }
}

