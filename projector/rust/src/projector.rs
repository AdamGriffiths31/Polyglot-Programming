use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

fn default_data() -> Data {
    Data {
        projector: HashMap::new(),
    }
}

impl Projector {
    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .remove(&key);
    }

    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut current = Some(self.pwd.as_path());
        let mut paths = vec![];
        while let Some(p) = current {
            paths.push(p);
            current = p.parent();
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter());
            }
        }

        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut current = Some(self.pwd.as_path());
        let mut out = None;
        while let Some(p) = current {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            current = p.parent();
        }

        return out;
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if !std::fs::metadata(p).is_ok() {
                std::fs::create_dir_all(p)?;
            }
        }

        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;

        return Ok(());
    }

    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or(String::from("{\"projector\": {}}"));
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data {
                projector: HashMap::new(),
            });

            return Projector { config, pwd, data };
        }
        return Projector {
            config,
            pwd,
            data: default_data(),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::projector::Data;
    use crate::projector::Projector;
    use collection_macros::hashmap;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
        PathBuf::from("/") => hashmap!{
            "foo".into() => "bar1".into(),
            "test".into() => "test1".into(),
        },
        PathBuf::from("/foo") => hashmap!{
            "foo".into() => "bar2".into(),
        },
        PathBuf::from("/foo/bar") => hashmap!{
            "foo".into() => "bar3".into(),
        },
                };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: PathBuf::from(""),
            pwd,
            data: Data {
                projector: get_data(),
            },
        };
    }

    #[test]
    fn get_value() {
        let projector = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(projector.get_value("foo".into()), Some(&"bar3".into()));
        assert_eq!(projector.get_value("test".into()), Some(&"test1".into()));
    }

    #[test]
    fn set_value() {
        let mut projector = get_projector(PathBuf::from("/foo/bar"));
        projector.set_value("foo".into(), "bar4".into());
        assert_eq!(projector.get_value("foo".into()), Some(&"bar4".into()));
        assert_eq!(projector.get_value("test".into()), Some(&"test1".into()));
    }

    #[test]
    fn remove_value() {
        let mut projector = get_projector(PathBuf::from("/foo/bar"));
        projector.remove_value("foo".into());
        projector.remove_value("test".into());
        assert_eq!(projector.get_value("foo".into()), Some(&"bar2".into()));
        assert_eq!(projector.get_value("test".into()), Some(&"test1".into()));
    }
}
