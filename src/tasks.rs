pub struct Task {
    id: u8,
    name: String,
    description: String,
    priority: u8, // From 1 to 5
}

fn priority_check(priority: u8) -> bool {
    priority > 0 && priority <= 5;
    true
}
