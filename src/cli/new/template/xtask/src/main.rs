use reign_common_tasks::default;
use reign_task::Tasks;

fn main() {
    Tasks::new("reign").tasks(default()).parse();
}
