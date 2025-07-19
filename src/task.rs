use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
    
    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

#[derive(Debug, Default)]
pub struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
    
    pub fn add_task(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        let task = Task::new(id, title, description);
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }
    
    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }
    
    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    
    pub fn toggle_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.toggle_completed();
            true
        } else {
            false
        }
    }
    
    pub fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }
    
    pub fn get_completed_count(&self) -> usize {
        self.tasks.values().filter(|t| t.completed).count()
    }
    
    pub fn get_total_count(&self) -> usize {
        self.tasks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "Test Task".to_string(), "Test Description".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert!(!task.completed);
    }
    
    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Test Task".to_string(), "Test Description".to_string());
        
        assert_eq!(manager.get_total_count(), 1);
        assert_eq!(manager.get_completed_count(), 0);
        
        manager.toggle_task(id);
        assert_eq!(manager.get_completed_count(), 1);
    }
}