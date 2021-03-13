use reign_task::Tasks;

fn main() {
    Tasks::new("{{ name }}")
        .tasks(reign_common_tasks::tasks())
        .parse();
}
