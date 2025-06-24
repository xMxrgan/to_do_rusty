enum Action {
    Create,
    Edit,
    Delete,
}
enum Kanban {
    Active,
    Completed,
    Deleted,
}
struct Task {
    id: i32,
    name: String,
    description: String,
    priority: usize, // From 1 to 5
}
