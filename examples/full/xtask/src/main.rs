use reign_task::Tasks;

fn main() {
    Tasks::new("reign")
        .tasks(reign_common_tasks::tasks())
        .parse();
}
