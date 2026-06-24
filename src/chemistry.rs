use std::collections::BTreeSet;

/// Canonical chemical element symbols.
pub const ELEMENT_SYMBOLS: &[&str] = &[
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk",
    "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh",
    "Fl", "Mc", "Lv", "Ts", "Og",
];

/// Returns `true` if `symbol` is a canonical element symbol.
pub fn is_valid_element_symbol(symbol: &str) -> bool {
    ELEMENT_SYMBOLS.contains(&symbol)
}

/// Normalizes and validates an element symbol.
///
/// Returns `None` when the input is empty or not a known element symbol.
///
/// # Parameters
/// - `input`: Element symbol passed from CLI input.
pub fn normalize_element_symbol(input: &str) -> Option<String> {
    let input = input.trim();

    if input.is_empty() {
        return None;
    }

    let mut chars = input.chars();

    let first = chars.next()?.to_ascii_uppercase();
    let rest = chars.as_str().to_ascii_lowercase();
    let symbol = format!("{first}{rest}");

    is_valid_element_symbol(&symbol).then_some(symbol)
}

pub fn contains_element(formula: &str, target: &str) -> bool {
    formula_symbols(formula).any(|symbol| symbol == target)
}

pub fn element_symbols_in_formula(formula: &str) -> BTreeSet<String> {
    formula_symbols(formula).filter(|symbol| is_valid_element_symbol(symbol)).collect()
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

    #[test]
    fn rejects_invalid_element_symbols() {
        assert_eq!(normalize_element_symbol("Bl"), None);
        assert_eq!(normalize_element_symbol("Xx"), None);
        assert_eq!(normalize_element_symbol("Water"), None);
    }

    #[test]
    fn all_real_halogen_symbols_are_valid() {
        assert_eq!(normalize_element_symbol("F"), Some("F".to_string()));
        assert_eq!(normalize_element_symbol("Cl"), Some("Cl".to_string()));
        assert_eq!(normalize_element_symbol("Br"), Some("Br".to_string()));
        assert_eq!(normalize_element_symbol("I"), Some("I".to_string()));
    }
}
