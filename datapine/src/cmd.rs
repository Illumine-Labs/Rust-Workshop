use crate::error::DatapineErr;

pub enum Cmd {
    Ping,
    Insert(String, Vec<f32>),
    Get(String),
    Remove(String),
    Cos(String, String),
    Knn(String, usize),
    Dump(String),
}

impl Cmd {
    pub fn parse(input: &str) -> Result<Cmd, std::io::Error> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
    
        if tokens.is_empty() {
            return Err(DatapineErr::EmptyCommand).map_err(|e| e.into());
        }
    
        match tokens[0].to_lowercase().as_str() {
            "ping" => Ok(Cmd::Ping),

            "insert" => {
                if tokens.len() < 3 {
                    return Err(DatapineErr::InvalidInsert).map_err(|e| e.into());
                }
                let key = tokens[1].to_string();
                let values = tokens[2..]
                    .iter()
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect();
    
                Ok(Cmd::Insert(key, values))
            },

            "get" => {
                if tokens.len() != 2 {
                    return Err(DatapineErr::InvalidGet).map_err(|e| e.into());
                }
                let key = tokens[1].to_string();
                Ok(Cmd::Get(key))
            },

            "remove" => {
                if tokens.len() != 2 {
                    return Err(DatapineErr::InvalidRemove).map_err(|e| e.into());
                }
                let key = tokens[1].to_string();
                Ok(Cmd::Remove(key))
            },

            "cosine" => {
                if tokens.len() != 3 {
                    return Err(DatapineErr::InvalidCosine).map_err(|e| e.into());
                }
                let key1 = tokens[1].to_string();
                let key2 = tokens[2].to_string();
                Ok(Cmd::Cos(key1, key2))
            },

            "knn" => {
                if tokens.len() != 3 {
                    return Err(DatapineErr::InvalidKNN).map_err(|e| e.into());
                }
                let key = tokens[1].to_string();
                let k = tokens[2].parse::<usize>().unwrap();
                Ok(Cmd::Knn(key, k))
            },

            "dump" => {
                if tokens.len() != 2 {
                    return Err(DatapineErr::InvalidDump).map_err(|e| e.into());
                }
                let key = tokens[1].to_string();
                Ok(Cmd::Dump(key))
            },

            _ => Err(DatapineErr::UnknownCommand).map_err(|e| e.into()),
        }
    }
}