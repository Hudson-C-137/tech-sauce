mod models;
mod engine;

fn main() {
    let mut task_01 = models::SauceTask {
        id: 137,
        name: String::from("Analisar anomalia dimensional"),
        status: models::TaskStatus::Pending,
    };

    engine::apply_sauce(&mut task_01);
}
