pub enum TaskStatus {
    Pending,
    Processing,
    Done,
}

pub struct SauceTask {
    pub id: u32,
    pub name: String,
    pub status: TaskStatus,
}
