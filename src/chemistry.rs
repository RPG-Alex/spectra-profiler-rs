use std::collections::BTreeSet;

pub fn normalize_element_symbol(input: &str) -> Option<String> {
    let input = input.trim();

    if input.is_empty() {
        return None;
    }

    let mut chars = input.chars();

    let first = chars.next()?.to_ascii_uppercase();
    let rest = chars.as_str().to_ascii_lowercase();

    Some(format!("{first}{rest}"))
}

pub fn contains_element(formula: &str, target: &str) -> bool {
    formula_symbols(formula).any(|symbol| symbol == target)
}

pub fn element_symbols_in_formula(formula: &str) -> BTreeSet<String> {
    formula_symbols(formula).collect()
}

fn formula_symbols(formula: &str) -> impl Iterator<Item = String> + '_ {
    let mut chars = formula.chars().peekable();

    std::iter::from_fn(move || {
        while let Some(ch) = chars.next() {
            if !ch.is_ascii_uppercase() {
                continue;
            }

            let mut symbol = String::from(ch);

            if let Some(next) = chars.peek()
                && next.is_ascii_lowercase()
            {
                symbol.push(*next);
                chars.next();
            }

            return Some(symbol);
        }

        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_element_symbols() {
        assert_eq!(normalize_element_symbol("f"), Some("F".to_string()));
        assert_eq!(normalize_element_symbol("F"), Some("F".to_string()));
        assert_eq!(normalize_element_symbol("cl"), Some("Cl".to_string()));
        assert_eq!(normalize_element_symbol("CL"), Some("Cl".to_string()));
        assert_eq!(normalize_element_symbol(" br "), Some("Br".to_string()));
    }

    #[test]
    fn rejects_empty_element_symbols() {
        assert_eq!(normalize_element_symbol(""), None);
        assert_eq!(normalize_element_symbol("   "), None);
    }

    #[test]
    fn detects_single_letter_elements() {
        assert!(contains_element("C6H6F", "F"));
        assert!(contains_element("C20H25IN2O5", "I"));
        assert!(contains_element("C10H14N2", "N"));
    }

    #[test]
    fn detects_two_letter_elements() {
        assert!(contains_element("C6H5Cl", "Cl"));
        assert!(contains_element("C6H5Br", "Br"));
        assert!(contains_element("C6H5Na", "Na"));
    }

    #[test]
    fn does_not_match_substrings_inside_other_elements() {
        assert!(!contains_element("C6H5Fe", "F"));
        assert!(!contains_element("C6H5Si", "I"));
        assert!(!contains_element("NaCl", "N"));
        assert!(!contains_element("NaCl", "C"));
    }

    #[test]
    fn extracts_unique_elements_from_formula() {
        let symbols = element_symbols_in_formula("C6H5ClBrNO2");

        assert!(symbols.contains("C"));
        assert!(symbols.contains("H"));
        assert!(symbols.contains("Cl"));
        assert!(symbols.contains("Br"));
        assert!(symbols.contains("N"));
        assert!(symbols.contains("O"));
        assert_eq!(symbols.len(), 6);
    }
}
