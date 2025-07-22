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
        let incomplete_tasks = total_tasks - completed_tasks;
        let completion_rate = if total_tasks > 0 {
            (completed_tasks as f32 / total_tasks as f32 * 100.0) as u32
        } else {
            0
        };
        let avg_completion_time = crate::wasm::get_average_completion_time();
        
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
                    ui.heading(if avg_completion_time > 0.0 {
                        format!("{:.1}", avg_completion_time)
                    } else {
                        "N/A".to_string()
                    });
                });
            });
        });
        
        ui.add_space(20.0);
        
        // Time series charts with real data
        ui.heading("📈 Task Time Series Analysis");
        ui.add_space(10.0);
        
        use egui_plot::{Line, Plot, PlotPoints};
        
        // Get real time series data
        let completed_series_json = crate::wasm::get_completed_tasks_time_series(30);
        let incomplete_series_json = crate::wasm::get_incomplete_tasks_time_series(30);
        let cumulative_series_json = crate::wasm::get_cumulative_completed_time_series(30);
        
        Plot::new("time_series_plot")
            .height(250.0)
            .show(ui, |plot_ui| {
                // Parse and plot completed tasks per day
                if let Ok(completed_data) = serde_json::from_str::<Vec<[f64; 2]>>(&completed_series_json) {
                    if !completed_data.is_empty() {
                        plot_ui.line(
                            Line::new(PlotPoints::from(completed_data))
                                .color(Color32::from_rgb(100, 200, 100))
                                .name("Tasks Completed/Day")
                        );
                    }
                }
                
                // Parse and plot incomplete tasks
                if let Ok(incomplete_data) = serde_json::from_str::<Vec<[f64; 2]>>(&incomplete_series_json) {
                    if !incomplete_data.is_empty() {
                        plot_ui.line(
                            Line::new(PlotPoints::from(incomplete_data))
                                .color(Color32::from_rgb(200, 100, 100))
                                .name("Incomplete Tasks")
                        );
                    }
                }
                
                // Parse and plot cumulative completed tasks
                if let Ok(cumulative_data) = serde_json::from_str::<Vec<[f64; 2]>>(&cumulative_series_json) {
                    if !cumulative_data.is_empty() {
                        plot_ui.line(
                            Line::new(PlotPoints::from(cumulative_data))
                                .color(Color32::from_rgb(100, 100, 200))
                                .name("Cumulative Completed")
                        );
                    }
                }
            });
            
        ui.add_space(10.0);
        ui.label("📊 Real-time task metrics over the last 30 days");
        
        ui.add_space(20.0);
        
        // Task Completion Predictions
        ui.heading("🔮 Task Completion Predictions");
        ui.add_space(10.0);
        
        let predictions_json = crate::wasm::get_task_completion_predictions();
        match serde_json::from_str::<Vec<serde_json::Value>>(&predictions_json) {
            Ok(predictions) => {
                if predictions.is_empty() {
                    ui.label("No incomplete tasks to predict");
                } else {
                    ui.label(format!("Predictions for {} incomplete tasks:", predictions.len()));
                    ui.add_space(5.0);
                    
                    // Show predictions in a scrollable area
                    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for prediction in predictions.iter().take(10) { // Show max 10 predictions
                            if let (Some(task_id), Some(predicted_hours)) = (
                                prediction["task_id"].as_u64(),
                                prediction["predicted_hours"].as_f64()
                            ) {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Task #{}: ", task_id));
                                    if predicted_hours < 1.0 {
                                        ui.label(format!("{:.0} minutes", predicted_hours * 60.0));
                                    } else if predicted_hours < 24.0 {
                                        ui.label(format!("{:.1} hours", predicted_hours));
                                    } else {
                                        ui.label(format!("{:.1} days", predicted_hours / 24.0));
                                    }
                                });
                            }
                        }
                        
                        if predictions.len() > 10 {
                            ui.label(format!("... and {} more", predictions.len() - 10));
                        }
                    });
                }
            }
            Err(_) => {
                ui.label("Error loading predictions");
            }
        }
        
        ui.add_space(20.0);
        ui.label("📈 Insights:");
        ui.label("• Task completion trends are based on real historical data");
        ui.label("• Predictions use average completion time and task age factors");
        if completion_rate > 70 {
            ui.label("• Great job! You're completing most of your tasks");
        } else {
            ui.label("• Focus on completing existing tasks before adding new ones");
        }
        
        if incomplete_tasks > 0 {
            ui.label(format!("• You have {} incomplete tasks - consider prioritizing older ones", incomplete_tasks));
        }
    }
}