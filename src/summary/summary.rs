use super::node_summary::NodeSummary;

const UNINIT: i64 = -1;
const INIT: i64 = 0;

pub struct Node {
    pub node_summary: NodeSummary,
    pub children: Vec<Node>,
    pub file_count: i64,
    pub dir_count: i64,
    pub total_size: i64,
}

pub struct Summary {
    pub full_dir_path: String,
    pub root_node: Node,
}

impl Summary {
    pub fn new(full_dir_path: &str) -> Summary {
        Summary {
            full_dir_path: full_dir_path.to_string(),
            root_node: Node::new(full_dir_path, full_dir_path),
        }
    }

    pub fn get_file_count(&mut self) -> i64 {
        self.root_node.get_file_count()
    }

    pub fn get_dir_count(&mut self) -> i64 {
        self.root_node.get_dir_count()
    }

    pub fn get_total_size(&mut self) -> i64 {
        self.root_node.get_total_size()
    }
}

impl Node {
    fn new(full_dir_path: &str, parent_dir: &str) -> Node {
        Node {
            node_summary: NodeSummary::new(full_dir_path, parent_dir),
            children: vec![],
            file_count: UNINIT,
            dir_count: UNINIT,
            total_size: UNINIT,
        }
    }

    fn init_node(&mut self) {
        self.file_count = UNINIT;
        self.dir_count = UNINIT;
        self.total_size = UNINIT;
    }

    fn get_file_count(&mut self) -> i64 {
        if self.node_summary.file_type == crate::utils::file::FileType::File {
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
        if self.node_summary.file_type == crate::utils::file::FileType::File {
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

    fn get_total_size(&mut self) -> i64 {
        if self.node_summary.file_type == crate::utils::file::FileType::File {
            self.node_summary.file_size as i64
        } else {
            if self.total_size != UNINIT {
                return self.total_size;
            }
            self.total_size = INIT;

            for child in &mut self.children {
                self.total_size += child.get_total_size();
            }
            self.total_size
        }
    }
}
