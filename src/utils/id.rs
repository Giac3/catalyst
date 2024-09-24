use std::collections::HashMap;
use rand::{distributions::Alphanumeric, Rng};


static CID_TYPES_STR: &str = "cre";

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
    fn get_type_from_char(type_char: char) -> CIDTypes {
        match type_char {
             'c' => CIDTypes::Collection,
             'r' => CIDTypes::Run,
             'e' => CIDTypes::Endpoint,
             _ => panic!("invalid CID type"),
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
    has_loaded: bool,
    cids: HashMap<String, CID>,
}

impl CIDStore {
    pub fn new () -> CIDStore {
        return CIDStore {
            has_loaded: false,
            cids: HashMap::new()
        }
    }
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

    fn cid_from_string(&mut self, cid_str: &String) -> CID {
        let mut chars = cid_str.chars();

        let _type = CIDTypes::get_type_from_char(chars.next().unwrap());
        chars.next();

        let noise = chars.as_str().to_string();

        CID { _type, noise }

    }

    pub fn load_cids(&mut self, cids: Vec<String>) {
        for cid_str in cids {
            if validate_cid(&cid_str) {
                let cid = self.cid_from_string(&cid_str);
                self.cids.insert(cid_str, cid);
            }
        }
        self.has_loaded = true
    }
    
}

pub fn validate_cid(cid_to_check: &str) -> bool {
    if cid_to_check.len() != 9 {
        return false;
    }

    let mut chars = cid_to_check.chars();

    
    match chars.next() {
        Some(c) if CID_TYPES_STR.contains(c) => (),
        _ => return false,
    };

    
    if chars.next() != Some(':') {
        return false;
    }

    let noise = chars.as_str();
    noise.len() == 7 && noise.chars().all(char::is_alphanumeric)
}


#[cfg(test)]
mod tests {
    use super::validate_cid;

    #[test]
    fn test_valid_cid() {
        assert!(validate_cid("c:abcdefg"));
        assert!(validate_cid("r:1234567"));
        assert!(validate_cid("e:ABCDEFG"));
    }

    #[test]
    fn test_invalid_length() {
        assert!(!validate_cid("c:abcd")); 
        assert!(!validate_cid("c:abcdefgh"));
    }

    #[test]
    fn test_invalid_type() {
        assert!(!validate_cid("x:abcdefg"));
        assert!(!validate_cid("1:abcdefg"));
        assert!(!validate_cid("::abcdefg"));
    }

    #[test]
    fn test_invalid_separator() {
        assert!(!validate_cid("c;abcdefg"));
        assert!(!validate_cid("c abcdefg"));
    }

    #[test]
    fn test_invalid_noise() {
        assert!(!validate_cid("c:abcde*f"));
        assert!(!validate_cid("c:abcd@ef"));
        assert!(!validate_cid("c:abcde_f"));
    }
}