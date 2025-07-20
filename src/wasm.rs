use wasm_bindgen::prelude::*;
use crate::task::{TaskManager, Task};
use std::sync::Mutex;

// Global task manager instance
lazy_static::lazy_static! {
    static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for console.log
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn init() {
    console_log!("WASM Task Manager initialized!");
}

#[wasm_bindgen]
pub fn add_task(title: String, description: String) -> u32 {
    let mut manager = TASK_MANAGER.lock().unwrap();
    let id = manager.add_task(title, description);
    console_log!("Added task with id: {}", id);
    id
}

#[wasm_bindgen]
pub fn toggle_task(id: u32) -> bool {
    let mut manager = TASK_MANAGER.lock().unwrap();
    let success = manager.toggle_task(id);
    console_log!("Toggled task {}: {}", id, success);
    success
}

#[wasm_bindgen]
pub fn remove_task(id: u32) -> bool {
    let mut manager = TASK_MANAGER.lock().unwrap();
    let success = manager.remove_task(id);
    console_log!("Removed task {}: {}", id, success);
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