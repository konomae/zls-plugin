use serde::Deserialize;
use std::collections::HashMap;

// https://github.com/zigtools/zls/issues/1472

#[derive(Deserialize)]
pub struct ZlsDist {
    pub latest: String,
    pub versions: HashMap<String, Empty>,
}

#[derive(Deserialize)]
pub struct Empty {}
