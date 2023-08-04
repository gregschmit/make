#[derive(Debug)]
pub struct LogicalLine {
    pub physical_lines: Vec<String>,
    pub content: String,
    pub index: usize,
    pub breaks: Vec<usize>,
}
