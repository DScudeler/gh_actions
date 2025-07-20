use egui::{Context, CentralPanel, Layout, Align, ScrollArea, Color32};
use eframe::App;

#[derive(Default)]
pub struct TaskManagerApp {
    current_view: AppView,
    // Task-related state
    new_task_title: String,
    new_task_description: String,
}

#[derive(Default, PartialEq)]
enum AppView {
    #[default]
    TaskManager,
    KpiDashboard,
}

impl TaskManagerApp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl App for TaskManagerApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        match self.current_view {
            AppView::TaskManager => {
                self.show_task_manager(ctx, frame);
            }
            AppView::KpiDashboard => {
                self.show_kpi_dashboard(ctx, frame);
            }
        }
    }
}

impl TaskManagerApp {
    fn show_task_manager(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🚀 WASM Task Manager");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("📊 View KPIs").clicked() {
                        self.current_view = AppView::KpiDashboard;
                    }
                });
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            // Task statistics
            self.show_task_statistics(ui);
            
            ui.add_space(20.0);
            
            // Add new task form
            self.show_add_task_form(ui);
            
            ui.add_space(20.0);
            
            // Task list
            self.show_task_list(ui);
        });
    }
    
    fn show_kpi_dashboard(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📊 Task Management KPIs");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("← Back to Tasks").clicked() {
                        self.current_view = AppView::TaskManager;
                    }
                });
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            // Show KPI content inline instead of using separate app
            self.show_kpi_content(ui);
        });
    }
    
    fn show_task_statistics(&self, ui: &mut egui::Ui) {
        ui.heading("📈 Task Statistics");
        ui.add_space(5.0);
        
        // Get task counts from WASM functions
        let total_count = crate::wasm::get_task_count();
        let completed_count = crate::wasm::get_completed_count();
        let remaining_count = total_count - completed_count;
        
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Tasks");
                    ui.heading(total_count.to_string());
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Completed");
                    ui.heading(completed_count.to_string());
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Remaining");
                    ui.heading(remaining_count.to_string());
                });
            });
            
            if total_count > 0 {
                let completion_rate = (completed_count as f32 / total_count as f32 * 100.0) as u32;
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Completion Rate");
                        ui.heading(format!("{}%", completion_rate));
                    });
                });
            }
        });
    }
    
    fn show_add_task_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("➕ Add New Task");
        ui.add_space(5.0);
        
        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.new_task_title);
        });
        
        ui.horizontal(|ui| {
            ui.label("Description:");
            ui.text_edit_multiline(&mut self.new_task_description);
        });
        
        ui.add_space(5.0);
        
        if ui.button("Add Task").clicked() {
            if !self.new_task_title.trim().is_empty() {
                crate::wasm::add_task(
                    self.new_task_title.clone(),
                    self.new_task_description.clone()
                );
                self.new_task_title.clear();
                self.new_task_description.clear();
            }
        }
    }
    
    fn show_task_list(&self, ui: &mut egui::Ui) {
        ui.heading("📋 Tasks");
        ui.add_space(5.0);
        
        // Get tasks from WASM
        let tasks_json = crate::wasm::get_all_tasks_json();
        match serde_json::from_str::<Vec<crate::task::Task>>(&tasks_json) {
            Ok(tasks) => {
                if tasks.is_empty() {
                    ui.label("No tasks yet. Add one above!");
                } else {
                    ScrollArea::vertical().show(ui, |ui| {
                        for task in tasks.iter() {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.checkbox(&mut task.completed.clone(), "").clicked() {
                                        crate::wasm::toggle_task(task.id);
                                    }
                                    
                                    ui.vertical(|ui| {
                                        ui.strong(&task.title);
                                        if !task.description.is_empty() {
                                            ui.label(&task.description);
                                        }
                                    });
                                    
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.button("🗑").clicked() {
                                            crate::wasm::remove_task(task.id);
                                        }
                                    });
                                });
                            });
                            ui.add_space(5.0);
                        }
                    });
                }
            }
            Err(_) => {
                ui.label("Error loading tasks");
            }
        }
    }
    
    fn show_kpi_content(&self, ui: &mut egui::Ui) {
        // Inline KPI content instead of delegating to separate app
        
        ui.horizontal(|ui| {
            let _ = ui.selectable_label(true, "📊 Overview");
            let _ = ui.selectable_label(false, "📈 Task Creation");
            let _ = ui.selectable_label(false, "⏱️ Completion Time");
            let _ = ui.selectable_label(false, "🚀 Productivity");
        });
        
        ui.separator();
        ui.add_space(10.0);
        
        // Get real task data for KPIs
        let total_tasks = crate::wasm::get_task_count();
        let completed_tasks = crate::wasm::get_completed_count();
        let completion_rate = if total_tasks > 0 {
            (completed_tasks as f32 / total_tasks as f32 * 100.0) as u32
        } else {
            0
        };
        
        ui.heading("KPI Overview");
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Tasks");
                    ui.heading(total_tasks.to_string());
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Completed");
                    ui.heading(completed_tasks.to_string());
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Completion Rate");
                    ui.heading(format!("{}%", completion_rate));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Avg. Time (hours)");
                    ui.heading("2.3"); // Mock data for now
                });
            });
        });
        
        ui.add_space(20.0);
        
        // Simple plot placeholder
        ui.heading("📈 Task Completion Trend");
        ui.add_space(10.0);
        
        use egui_plot::{Line, Plot, PlotPoints};
        
        // Generate sample trend data
        let mut points = Vec::new();
        for i in 0..30 {
            let x = i as f64;
            let y = (completed_tasks as f64 / 30.0) * i as f64 + (i as f64 * 0.1).sin();
            points.push([x, y]);
        }
        
        Plot::new("completion_trend")
            .height(200.0)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from(points))
                        .color(Color32::from_rgb(100, 200, 100))
                        .name("Tasks Completed")
                );
            });
            
        ui.add_space(10.0);
        ui.label("📊 Cumulative task completion over the last 30 days");
        
        ui.add_space(20.0);
        ui.label("📈 Insights:");
        ui.label("• Task completion is steady");
        ui.label("• Consider setting daily goals to improve productivity");
        if completion_rate > 70 {
            ui.label("• Great job! You're completing most of your tasks");
        } else {
            ui.label("• Focus on completing existing tasks before adding new ones");
        }
    }
}