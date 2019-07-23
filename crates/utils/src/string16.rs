#[derive(Default)]
pub struct String16 {
    utf16: Vec<u16>,
}

impl String16 {
    pub fn new() -> Self {
        String16 { utf16: vec![] }
    }

    pub fn len(&self) -> usize {
        self.utf16.len()
    }

    pub fn as_bytes(&self) -> &[u16] {
        &self.utf16
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u16] {
        &mut self.utf16
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        let mut counter = idx;
        for u in string.encode_utf16() {
            self.utf16.insert(counter, u);
            counter += 1;
        }
    }

    pub fn insert_string(&mut self, idx: usize, string: String) {
        let mut counter = idx;
        for u in string.encode_utf16() {
            self.utf16.insert(counter, u);
            counter += 1;
        }
    }

    pub fn remove(&mut self, idx: usize) {
        self.utf16.remove(idx);
    }

    pub fn is_empty(&self) -> bool {
        self.utf16.is_empty()
    }

    pub fn to_string(&self) -> String {
        if let Ok(string) = String::from_utf16(&self.utf16) {
            return string;
        }

        String::from("")
    }
}

impl From<&str> for String16 {
    fn from(s: &str) -> Self {
        String16 {
            utf16: s.encode_utf16().collect(),
        }
    }
}

impl From<String> for String16 {
    fn from(string: String) -> Self {
        String16 {
            utf16: string.encode_utf16().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_string() {
        let string16 = String16::from(String::from("Übung"));
        assert_eq!(string16.len(), 5);

        let string16 = String16::from(String::from("World"));
        assert_eq!(string16.len(), 5);
    }

     #[test]
    fn from_str() {
        let string16 = String16::from("Übung");
        assert_eq!(string16.len(), 5);

        let string16 = String16::from("World");
        assert_eq!(string16.len(), 5);
    }
}
