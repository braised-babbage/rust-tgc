use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Symbol {
    idx: usize,
}

pub struct Interner<'a> {
    entries: HashMap<&'a str, Symbol>,
    names: Vec<&'a str>,
}

// todo: this works, but can we do this without leaking?
fn intern(name: &str) -> &'static str {
    Box::leak(Box::new(name.to_string()))
}

impl<'a> Interner<'a> {
    pub fn new() -> Self {
	Interner { entries: HashMap::new(), names: vec![] }
    }

    pub fn symbol<'b>(&mut self, name: &'b str) -> Symbol {
	if let Some(sym) = self.entries.get(name) {
	    return *sym;
	}
	let interned = intern(name);
	let idx = self.names.len();
	let sym = Symbol{idx};
	self.names.push(interned);
	self.entries.insert(interned, sym);
	sym
    }

    pub fn name(&mut self, sym: Symbol) -> &'a str {
	&self.names[sym.idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_equality_check() {
	let mut tbl = Interner::new();

	let s1 = tbl.symbol("foo");
	let s2 = tbl.symbol("bar");
	let s3 = tbl.symbol("foo");

	assert_eq!(s1, s3);
	assert_ne!(s1, s2);
	assert_eq!(tbl.name(s1), "foo");
	assert_eq!(tbl.name(s2), "bar");
    }

    #[test]
    fn mixed_lifetimes() {
	let mut tbl = Interner::new();

	let s1 = tbl.symbol("foo");
	let s2 = tbl.symbol("foo".to_string().as_str());

	assert_eq!(s1, s2);
    }
}
