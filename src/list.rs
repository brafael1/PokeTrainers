use crate::Data;

pub struct List {
    names: Vec<String>,
}

impl List {
    pub fn read() -> Self {
        let mut names: Vec<String> = Data::iter()
            .map(|path| path.as_ref().trim_end_matches(".png").to_owned())
            .collect();

        names.sort_unstable();

        Self { names }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.names
            .binary_search_by(|n| n.as_str().cmp(name))
            .ok()
            .map(|i| &self.names[i])
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    pub fn random(&self) -> &str {
        let idx = rand::random_range(0..self.names.len());
        &self.names[idx]
    }

    pub fn format_name(&self, filename: &str) -> String {
        filename
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.names.iter()
    }
}