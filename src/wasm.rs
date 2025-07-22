use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::task::{TaskManager, Task};
use crate::app::TaskManagerApp;
use std::sync::Mutex;
use web_sys::HtmlCanvasElement;

// Global task manager instance
lazy_static::lazy_static! {
    static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    // localStorage bindings
    #[wasm_bindgen(js_name = saveToLocalStorage)]
    fn save_to_local_storage(data: &str);
    
    #[wasm_bindgen(js_name = loadFromLocalStorage)]
    fn load_from_local_storage() -> String;
}

// Macro for console.log
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Helper function to save tasks to localStorage
fn save_tasks() {
    let manager = TASK_MANAGER.lock().unwrap();
    let tasks = manager.get_all_tasks();
    match serde_json::to_string(&tasks) {
        Ok(json) => {
            save_to_local_storage(&json);
            console_log!("Tasks saved to localStorage");
        },
        Err(e) => console_log!("Failed to serialize tasks: {:?}", e),
    }
}

// Helper function to load tasks from localStorage
fn load_tasks() {
    let json = load_from_local_storage();
    if !json.is_empty() && json != "null" {
        match serde_json::from_str::<Vec<Task>>(&json) {
            Ok(tasks) => {
                let mut manager = TASK_MANAGER.lock().unwrap();
                // Clear existing tasks and load from storage
                *manager = TaskManager::new();
                for task in tasks {
                    // Find the maximum ID to continue sequence
                    if task.id >= manager.next_id {
                        manager.next_id = task.id + 1;
                    }
                    manager.tasks.insert(task.id, task);
                }
                console_log!("Loaded {} tasks from localStorage", manager.get_total_count());
            },
            Err(e) => console_log!("Failed to parse tasks from localStorage: {:?}", e),
        }
    } else {
        console_log!("No tasks found in localStorage");
    }
}

#[wasm_bindgen]
pub fn init() {
    console_log!("WASM Task Manager initialized!");
    load_tasks();
}

#[wasm_bindgen]
pub fn add_task(title: String, description: String) -> u32 {
    let mut manager = TASK_MANAGER.lock().unwrap();
    let id = manager.add_task(title, description);
    console_log!("Added task with id: {}", id);
    drop(manager); // Release the lock before saving
    save_tasks();
    id
}

#[wasm_bindgen]
pub fn toggle_task(id: u32) -> bool {
    let mut manager = TASK_MANAGER.lock().unwrap();
    let success = manager.toggle_task(id);
    console_log!("Toggled task {}: {}", id, success);
    drop(manager); // Release the lock before saving
    save_tasks();
    success
}

#[wasm_bindgen]
pub fn remove_task(id: u32) -> bool {
    let mut manager = TASK_MANAGER.lock().unwrap();
    let success = manager.remove_task(id);
    console_log!("Removed task {}: {}", id, success);
    drop(manager); // Release the lock before saving
    save_tasks();
    success
}

#[wasm_bindgen]
pub fn get_task_count() -> u32 {
    let manager = TASK_MANAGER.lock().unwrap();
    manager.get_total_count() as u32
}

#[wasm_bindgen]
pub fn get_completed_count() -> u32 {
    let manager = TASK_MANAGER.lock().unwrap();
    manager.get_completed_count() as u32
}

#[wasm_bindgen]
pub fn get_all_tasks_json() -> String {
    let manager = TASK_MANAGER.lock().unwrap();
    let tasks = manager.get_all_tasks();
    match serde_json::to_string(&tasks) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_completed_tasks_time_series(days: u32) -> String {
    let manager = TASK_MANAGER.lock().unwrap();
    let series = manager.get_completed_tasks_time_series(days);
    
    // Convert to format suitable for plotting: [[day_offset, count], ...]
    let plot_data: Vec<[f64; 2]> = series.iter().enumerate()
        .map(|(i, (_, count))| [i as f64, *count as f64])
        .collect();
    
    match serde_json::to_string(&plot_data) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_incomplete_tasks_time_series(days: u32) -> String {
    let manager = TASK_MANAGER.lock().unwrap();
    let series = manager.get_incomplete_tasks_time_series(days);
    
    // Convert to format suitable for plotting: [[day_offset, count], ...]
    let plot_data: Vec<[f64; 2]> = series.iter().enumerate()
        .map(|(i, (_, count))| [i as f64, *count as f64])
        .collect();
    
    match serde_json::to_string(&plot_data) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_cumulative_completed_time_series(days: u32) -> String {
    let manager = TASK_MANAGER.lock().unwrap();
    let series = manager.get_cumulative_completed_time_series(days);
    
    // Convert to format suitable for plotting: [[day_offset, count], ...]
    let plot_data: Vec<[f64; 2]> = series.iter().enumerate()
        .map(|(i, (_, count))| [i as f64, *count as f64])
        .collect();
    
    match serde_json::to_string(&plot_data) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_average_completion_time() -> f64 {
    let manager = TASK_MANAGER.lock().unwrap();
    manager.get_average_completion_time_hours().unwrap_or(0.0)
}

#[wasm_bindgen]
pub fn get_task_completion_predictions() -> String {
    let manager = TASK_MANAGER.lock().unwrap();
    let predictions = manager.predict_task_completion_times();
    
    // Convert to JSON format: [{"task_id": 1, "predicted_hours": 2.5}, ...]
    let prediction_objects: Vec<serde_json::Value> = predictions.iter()
        .map(|(task_id, hours)| {
            serde_json::json!({
                "task_id": task_id,
                "predicted_hours": hours
            })
        })
        .collect();
    
    match serde_json::to_string(&prediction_objects) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    }
}

#[wasm_bindgen]
pub struct WasmTask {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

#[wasm_bindgen]
impl WasmTask {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }
    
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn completed(&self) -> bool {
        self.completed
    }
}

impl From<&Task> for WasmTask {
    fn from(task: &Task) -> Self {
        WasmTask {
            id: task.id,
            title: task.title.clone(),
            description: task.description.clone(),
            completed: task.completed,
        }
    }
}

#[wasm_bindgen]
pub fn start_egui_app(canvas_id: &str) {
    console_log!("Starting egui app on canvas: {}", canvas_id);
    
    let web_options = eframe::WebOptions::default();
    let canvas_id = canvas_id.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id(&canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        
        let result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(TaskManagerApp::new()))),
            )
            .await;
            
        if let Err(e) = result {
            console_log!("Failed to start egui app: {:?}", e);
        }
    });
}