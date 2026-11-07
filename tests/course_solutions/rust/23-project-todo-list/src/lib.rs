use std::fs;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

#[derive(Debug, PartialEq)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn load(path: &Path) -> Result<TodoList, String> {
        match fs::read_to_string(path) {
            Ok(text) => Ok(TodoList::from_text(&text)),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(TodoList::new()),
            Err(error) => Err(format!("Couldn't read {}: {error}", path.display())),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        fs::write(path, self.to_text()).map_err(|error| format!("Couldn't save {}: {error}", path.display()))
    }

    pub fn add(&mut self, title: &str) -> Result<(), String> {
        let title = title.trim();
        if title.is_empty() {
            return Err("A task needs a title".to_string());
        }
        self.tasks.push(Task { title: title.to_string(), done: false });
        Ok(())
    }

    fn check_number(&self, n: usize) -> Result<(), String> {
        if n == 0 || n > self.tasks.len() {
            Err(format!("There's no task {n}"))
        } else {
            Ok(())
        }
    }

    pub fn complete(&mut self, n: usize) -> Result<(), String> {
        self.check_number(n)?;
        self.tasks[n - 1].done = true;
        Ok(())
    }

    pub fn remove(&mut self, n: usize) -> Result<Task, String> {
        self.check_number(n)?;
        Ok(self.tasks.remove(n - 1))
    }

    pub fn summary(&self) -> String {
        let done = self.tasks.iter().filter(|task| task.done).count();
        let total = self.tasks.len();
        let word = if total == 1 { "task" } else { "tasks" };
        format!("{total} {word}, {done} done")
    }

    pub fn to_text(&self) -> String {
        let mut text = String::new();
        for task in &self.tasks {
            text.push_str(if task.done { "[x] " } else { "[ ] " });
            text.push_str(&task.title);
            text.push('\n');
        }
        text
    }

    pub fn from_text(text: &str) -> TodoList {
        let mut list = TodoList::new();
        for line in text.lines() {
            if let Some(title) = line.strip_prefix("[x] ") {
                list.tasks.push(Task { title: title.to_string(), done: true });
            } else if let Some(title) = line.strip_prefix("[ ] ") {
                list.tasks.push(Task { title: title.to_string(), done: false });
            }
        }
        list
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TodoList {
        let mut list = TodoList::new();
        list.add("Buy milk").unwrap();
        list.add("  Walk the dog  ").unwrap();
        list
    }

    #[test]
    fn adding() {
        let list = sample();
        assert_eq!(list.tasks.len(), 2);
        assert_eq!(list.tasks[1].title, "Walk the dog", "spaces are trimmed from the ends");
        assert!(!list.tasks[0].done, "new tasks aren't done");
        assert_eq!(TodoList::new().add("   "), Err("A task needs a title".to_string()));
    }

    #[test]
    fn completing() {
        let mut list = sample();
        assert_eq!(list.complete(1), Ok(()));
        assert!(list.tasks[0].done, "task 1 is the first task");
        assert_eq!(list.complete(5), Err("There's no task 5".to_string()));
        assert_eq!(list.complete(0), Err("There's no task 0".to_string()));
    }

    #[test]
    fn removing() {
        let mut list = sample();
        let removed = list.remove(1).unwrap();
        assert_eq!(removed.title, "Buy milk");
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.remove(3), Err("There's no task 3".to_string()));
    }

    #[test]
    fn summaries() {
        let mut list = sample();
        list.complete(2).unwrap();
        assert_eq!(list.summary(), "2 tasks, 1 done");
        list.remove(1).unwrap();
        assert_eq!(list.summary(), "1 task, 1 done");
        assert_eq!(TodoList::new().summary(), "0 tasks, 0 done");
    }

    #[test]
    fn text_format() {
        let mut list = sample();
        list.complete(1).unwrap();
        assert_eq!(list.to_text(), "[x] Buy milk\n[ ] Walk the dog\n");
        let read_back = TodoList::from_text("[x] Buy milk\n\nnonsense\n[ ] Walk the dog\n");
        assert_eq!(read_back, list, "blank and unknown lines are skipped");
    }

    #[test]
    fn saving_and_loading() {
        let path = std::env::temp_dir().join(format!("waypoint-todo-test-{}.txt", std::process::id()));
        let list = sample();
        list.save(&path).unwrap();
        assert_eq!(TodoList::load(&path).unwrap(), list);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(TodoList::load(&path).unwrap(), TodoList::new(), "a missing file is an empty list");
    }
}
