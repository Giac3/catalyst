use std::collections::HashMap;
use rand::{distributions::Alphanumeric, Rng};


pub enum CIDTypes {
    Collection,
    Run,
    Endpoint,
}

impl CIDTypes {
    fn get_str(&self) -> &str {
        match self {
            CIDTypes::Collection => "c",
            CIDTypes::Run => "r",
            CIDTypes::Endpoint => "e",
        }
    }
}

impl Clone for CIDTypes {
    fn clone(&self) -> CIDTypes {
        match self {
            CIDTypes::Collection => return CIDTypes::Collection,
            CIDTypes::Run => return CIDTypes::Run,
            CIDTypes::Endpoint => return CIDTypes::Endpoint,
        }
    }
}

#[derive(Clone)]
struct CID {
    _type: CIDTypes,
    noise: String,
}

impl CID {
    pub fn format(&self) -> String {
        let str_type = self._type.get_str();
        format!("{str_type}:{}", self.noise)
    }
}

pub struct CIDStore {
    cids: HashMap<String, CID>,
}

impl CIDStore {
    pub fn new_cid(&mut self, _type: CIDTypes) -> String {
        loop {
            let noise: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();

            let cid = CID {
                _type: _type.clone(),
                noise: noise.clone(),
            };

            let formatted_cid = cid.format();

            if !self.cids.contains_key(&formatted_cid) {
                self.cids.insert(formatted_cid.clone(), cid);
                return formatted_cid;
            }
        }
    }
}