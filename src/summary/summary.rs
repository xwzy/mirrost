use super::node_summary::NodeSummary;

const UNINIT: i64 = -1;
const INIT: i64 = 0;

pub struct Node {
    pub summary: NodeSummary,
    pub children: Vec<Node>,
    pub file_count: i64,
    pub dir_count: i64,
    pub total_size: i64,
}

pub struct Summary {
    pub full_dir_path: String,
    pub summaries: Node,
}

impl Node {
    fn new(summary: NodeSummary) -> Node {
        Node {
            summary,
            children: vec![],
            file_count: UNINIT,
            dir_count: UNINIT,
            total_size: UNINIT,
        }
    }

    fn get_file_count(&mut self) -> i64 {
        if self.summary.file_type == crate::utils::file::FileType::File {
            1
        } else {
            if self.file_count != UNINIT {
                return self.file_count;
            }
            self.file_count = INIT;

            for child in &mut self.children {
                self.file_count += child.get_file_count();
            }
            self.file_count
        }
    }

    fn get_dir_count(&mut self) -> i64 {
        if self.summary.file_type == crate::utils::file::FileType::File {
            0
        } else {
            if self.dir_count != UNINIT {
                return self.dir_count;
            }
            self.dir_count = INIT;

            for child in &mut self.children {
                self.dir_count += child.get_dir_count();
            }
            self.dir_count
        }
    }
}
