use crate::Data;

/// All the available trainers, derived from the embedded sprite filenames.
///
/// No external manifest is needed: the list always matches the sprites that
/// are compiled into the binary by `rust-embed`.
pub struct List {
    /// The trainer filenames (without the `.png` extension), sorted.
    names: Vec<String>,
}

impl List {
    /// Reads a new [`List`] from the embedded sprite files.
    pub fn read() -> Self {
        let mut names: Vec<String> = Data::iter()
            .map(|path| path.as_ref().trim_end_matches(".png").to_owned())
            .collect();

        names.sort_unstable();

        Self { names }
    }

    /// Looks up a trainer by its exact filename.
    pub fn get(&self, name: &str) -> Option<&String> {
        self.names
            .binary_search_by(|n| n.as_str().cmp(name))
            .ok()
            .map(|i| &self.names[i])
    }

    /// The number of available trainers.
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Gets a random trainer's filename.
    pub fn random(&self) -> &str {
        let idx = rand::random_range(0..self.names.len());
        &self.names[idx]
    }

    /// Formats a sprite filename into a proper display name.
    ///
    /// # Examples
    ///
    /// ```
    /// use poketrainers::list::List;
    /// let list = List::read();
    /// assert_eq!(list.format_name("allister-unmasked"), "Allister Unmasked")
    /// ```
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

    /// Iterates over all the available trainer filenames.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.names.iter()
    }
}