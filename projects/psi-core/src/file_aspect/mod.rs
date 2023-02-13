// mod file_node;

pub struct FileType {
    name: String,
    description: String,
    mime_type: String,
    extensions: Vec<String>,
    is_binary: bool,
}

pub struct FileID(usize);
