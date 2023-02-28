use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

type InteriorHashMap = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data {
    pub projector: HashMap<PathBuf, InteriorHashMap>,
}

#[derive(Debug)]
pub struct Projector {
    config: Config,
    data: Data,
}

impl From<Config> for Projector {
    fn from(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = std::fs::read_to_string(&config.config);
            let contents = contents.unwrap_or("{\"projector\":{}}".to_string());
            let data = serde_json::from_str(&contents).unwrap_or(Data::default());
            return Self::new(config, data);
        }

        Self::new(config, Data::default())
    }
}

impl Projector {
    pub fn new(config: Config, data: Data) -> Self {
        Self { config, data }
    }

    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut paths = vec![];

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter())
            }
        }

        out
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }

            curr = p.parent()
        }

        out
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        self.data
            .projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key.into(), value.into());
    }

    pub fn remove_value(&mut self, key: &str) {
        if let Some(inner) = self.data.projector.get_mut(&self.config.pwd) {
            inner.remove(key);
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use collection_macros::hashmap;

    use crate::config::Config;
    use crate::config::Operation;

    use super::Data;
    use super::Projector;

    fn get_projector(pwd: PathBuf) -> Projector {
        Projector::new(
            Config {
                config: PathBuf::from(""),
                pwd,
                operation: Operation::Print(None),
            },
            get_data(),
        )
    }

    fn get_data() -> Data {
        Data {
            projector: hashmap! {
                PathBuf::from("/") => hashmap! {
                    "foo".into() => "bar1".into(),
                    "fem".into() => "is_great".into()
                },
                PathBuf::from("/foo") => hashmap! {
                    "foo".into() => "bar2".into()
                },
                PathBuf::from("/foo/bar") => hashmap! {
                    "foo".into() => "bar3".into()
                }
            },
        }
    }

    #[test]
    fn test_get_value() {
        let proj = get_projector(PathBuf::from("/"));
        assert_eq!(proj.get_value("foo"), Some(&"bar1".to_string()));
        assert_eq!(proj.get_value("fem"), Some(&"is_great".to_string()));

        let proj = get_projector(PathBuf::from("/foo"));
        assert_eq!(proj.get_value("foo"), Some(&"bar2".to_string()));
        assert_eq!(proj.get_value("fem"), Some(&"is_great".to_string()));

        let proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&"bar3".to_string()));
    }

    #[test]
    fn test_set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&"bar3".to_string()));

        proj.set_value("foo", "baz");
        assert_eq!(proj.get_value("foo"), Some(&"baz".to_string()));

        proj.set_value("fem", "is_super_great");
        assert_eq!(proj.get_value("fem"), Some(&"is_super_great".to_string()));
    }

    #[test]
    fn test_remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&"bar3".to_string()));

        proj.remove_value("foo");
        assert_eq!(proj.get_value("foo"), Some(&"bar2".to_string()));

        proj.remove_value("fem");
        assert_eq!(proj.get_value("fem"), Some(&"is_great".to_string()));
    }
}
