use egui::*;
use egui_plot::{Line, Plot, PlotPoints};
use chrono::{DateTime, Utc, Duration};
use crate::task::TaskManager;
use std::collections::HashMap;

#[derive(Default)]
pub struct KpiApp {
    current_view: KpiView,
}

#[derive(Default, PartialEq)]
enum KpiView {
    #[default]
    Overview,
    TaskCreation,
    CompletionTime,
    Productivity,
}

impl KpiApp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for KpiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📊 Task Management KPIs");
            ui.add_space(10.0);
            
            // Navigation buttons
            ui.horizontal(|ui| {
                if ui.button("Overview").clicked() {
                    self.current_view = KpiView::Overview;
                }
                if ui.button("Task Creation").clicked() {
                    self.current_view = KpiView::TaskCreation;
                }
                if ui.button("Completion Time").clicked() {
                    self.current_view = KpiView::CompletionTime;
                }
                if ui.button("Productivity").clicked() {
                    self.current_view = KpiView::Productivity;
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Back to Tasks").clicked() {
                        // This will be handled by the parent app
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            match self.current_view {
                KpiView::Overview => self.show_overview(ui),
                KpiView::TaskCreation => self.show_task_creation_chart(ui),
                KpiView::CompletionTime => self.show_completion_time_chart(ui),
                KpiView::Productivity => self.show_productivity_chart(ui),
            }
        });
    }
}

impl KpiApp {
    fn show_overview(&mut self, ui: &mut egui::Ui) {
        ui.heading("KPI Overview");
        ui.add_space(10.0);
        
        // Get mock data for now - in real implementation, this would come from TASK_MANAGER
        let total_tasks = 45;
        let completed_tasks = 32;
        let avg_completion_time = 2.3;
        let completion_rate = (completed_tasks as f32 / total_tasks as f32 * 100.0) as u32;
        
        ui.columns(4, |columns| {
            columns[0].vertical(|ui| {
                ui.label("Total Tasks");
                ui.heading(total_tasks.to_string());
            });
            
            columns[1].vertical(|ui| {
                ui.label("Completed");
                ui.heading(completed_tasks.to_string());
            });
            
            columns[2].vertical(|ui| {
                ui.label("Completion Rate");
                ui.heading(format!("{}%", completion_rate));
            });
            
            columns[3].vertical(|ui| {
                ui.label("Avg. Time (hours)");
                ui.heading(format!("{:.1}", avg_completion_time));
            });
        });
        
        ui.add_space(20.0);
        ui.label("📈 Quick insights:");
        ui.label("• Task completion is trending upward");
        ui.label("• Average completion time has improved by 15% this week");
        ui.label("• Most productive hours are between 10 AM - 2 PM");
    }
    
    fn show_task_creation_chart(&mut self, ui: &mut egui::Ui) {
        ui.heading("Task Creation Over Time");
        ui.add_space(10.0);
        
        // Generate sample data for the last 30 days
        let mut points = Vec::new();
        for i in 0..30 {
            let x = i as f64;
            let y = (3.0 + 2.0 * (i as f64 * 0.2).sin() + (i as f64 * 0.05).cos()) as f64;
            points.push([x, y]);
        }
        
        Plot::new("task_creation_plot")
            .height(300.0)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from(points))
                        .color(Color32::from_rgb(100, 200, 100))
                        .name("Tasks Created per Day")
                );
            });
            
        ui.add_space(10.0);
        ui.label("📊 Tasks created per day over the last 30 days");
    }
    
    fn show_completion_time_chart(&mut self, ui: &mut egui::Ui) {
        ui.heading("Task Completion Time Analysis");
        ui.add_space(10.0);
        
        // Generate sample completion time data
        let mut points = Vec::new();
        for i in 0..20 {
            let x = i as f64;
            let y = 1.5 + 0.3 * (i as f64 * 0.3).sin() + 0.1 * (i as f64 * 0.1).cos();
            points.push([x, y]);
        }
        
        Plot::new("completion_time_plot")
            .height(300.0)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from(points))
                        .color(Color32::from_rgb(200, 100, 100))
                        .name("Average Completion Time (hours)")
                );
            });
            
        ui.add_space(10.0);
        ui.label("⏱️ Average time to complete tasks over the last 20 completed tasks");
    }
    
    fn show_productivity_chart(&mut self, ui: &mut egui::Ui) {
        ui.heading("Productivity Analysis");
        ui.add_space(10.0);
        
        // Generate productivity data (tasks completed per day)
        let mut completed_points = Vec::new();
        let mut created_points = Vec::new();
        
        for i in 0..14 {
            let x = i as f64;
            let completed = 2.0 + 1.5 * (i as f64 * 0.4).sin() + 0.5 * (i as f64 * 0.1).cos();
            let created = 3.0 + 1.2 * (i as f64 * 0.3).cos() + 0.3 * (i as f64 * 0.2).sin();
            
            completed_points.push([x, completed]);
            created_points.push([x, created]);
        }
        
        Plot::new("productivity_plot")
            .height(300.0)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from(completed_points))
                        .color(Color32::from_rgb(100, 200, 100))
                        .name("Tasks Completed per Day")
                );
                
                plot_ui.line(
                    Line::new(PlotPoints::from(created_points))
                        .color(Color32::from_rgb(100, 100, 200))
                        .name("Tasks Created per Day")
                );
            });
            
        ui.add_space(10.0);
        ui.label("🚀 Daily productivity: tasks created vs completed over the last 2 weeks");
    }
}