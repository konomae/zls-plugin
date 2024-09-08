use serde::Deserialize;
use std::collections::HashMap;

// https://github.com/zigtools/release-worker#v1zlsindexjson

#[derive(Deserialize)]
pub struct ZlsDist {
    #[serde(flatten)]
    pub versions: HashMap<String, Empty>,
}

#[derive(Deserialize)]
pub struct Empty {}
