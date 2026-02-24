use crate::models::{SauceTask, TaskStatus};

pub fn apply_sauce(task: &mut SauceTask) {
    println!("🧪 Aplicando tech-sauce na tarefa: {}", task.name);
    task.status = TaskStatus::Processing;
    task.status = TaskStatus::Done;
    println!("✅ Tarefa {} concluída com sucesso.", task.id);
}
