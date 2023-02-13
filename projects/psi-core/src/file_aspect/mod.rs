// mod file_node;
use dashmap::{mapref::multiple::RefMulti, DashMap};
use std::{io::empty, sync::LazyLock};
pub mod manager;

pub struct FileID(usize);

pub struct FileInstance {
    name: String,
    description: String,
    mime_type: String,
    extensions: Vec<String>,
    is_binary: bool,
}

pub trait FileType {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn mime_type(&self) -> &str;
    fn extensions(&self) -> &[String];
    fn is_binary(&self) -> bool;
}
