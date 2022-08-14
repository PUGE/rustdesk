use hbb_common::{bail, ResultType};
use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

#[allow(dead_code)]
pub fn extract_resources(root_path: &str) -> ResultType<()> {
    let mut resources: Vec<(&str, &[u8])> = Vec::new();
