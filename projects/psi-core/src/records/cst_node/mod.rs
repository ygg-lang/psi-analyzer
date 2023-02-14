use std::{any::type_name, fmt::Display};

use crate::LanguageID;

pub struct FileID {
    hash: u64,
}

pub trait FileType: Display {
    fn debug_name() -> &'static str {
        type_name::<Self>()
    }

    fn language(&self) -> LanguageID;

    fn localized_name(&self) -> Option<String> {
        None
    }
    // weather to use hex editor
    fn icon(&self) -> Option<String> {
        None
    }
    fn is_binary(&self) -> bool {
        false
    }
    fn is_readonly(&self) -> bool {
        false
    }
    // weather to show in project tree
    fn is_hide(&self) -> bool {
        false
    }
    fn is_source(&self) -> bool {
        false
    }
    fn is_test(&self) -> bool {
        false
    }
    fn is_exclude(&self) -> bool {
        false
    }
    fn is_generated_source(&self) -> bool {
        self.is_generated() && self.is_source()
    }
    fn is_generated(&self) -> bool {
        false
    }
    // weather to use decompiler
    fn is_decompiled(&self) -> bool {
        false
    }
}

pub struct FileInstance {
    id: FileID,
    language: LanguageID,
}

pub struct NodeID {
    file: FileID,
    hash: u64,
}

pub struct PsiNode {
    this: NodeID,
    parent: Option<NodeID>,
    children: Vec<NodeID>,
}
