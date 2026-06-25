use std::collections::{BTreeMap, BTreeSet};

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

pub fn element_symbols_in_formula(formula: &str) -> BTreeSet<String> {
    element_counts_in_formula(formula).into_keys().collect()
}

/// Counts atoms for each valid element symbol in a molecular formula.
///
/// Missing numeric subscripts are interpreted as `1`.
///
/// # Parameters
/// - `formula`: Molecular formula string, such as `C6H5ClBrNO2`.
pub fn element_counts_in_formula(formula: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let mut chars = formula.chars().peekable();

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

        let mut count_text = String::new();

        while let Some(next) = chars.peek()
            && next.is_ascii_digit()
        {
            count_text.push(*next);
            chars.next();
        }

        if !is_valid_element_symbol(&symbol) {
            continue;
        }

        let count =
            if count_text.is_empty() { 1 } else { count_text.parse::<usize>().unwrap_or(1) };

        *counts.entry(symbol).or_default() += count;
    }

    counts
}

/// Returns the atom count for one element in a molecular formula.
///
/// Returns `0` when the formula does not contain the element.
pub fn atom_count_for_element(formula: &str, target: &str) -> usize {
    element_counts_in_formula(formula).get(target).copied().unwrap_or_default()
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
    fn counts_single_letter_elements() {
        assert_eq!(atom_count_for_element("C6H6F", "F"), 1);
        assert_eq!(atom_count_for_element("C20H25IN2O5", "I"), 1);
        assert_eq!(atom_count_for_element("C10H14N2", "N"), 2);
    }

    #[test]
    fn counts_two_letter_elements() {
        assert_eq!(atom_count_for_element("C6H5Cl", "Cl"), 1);
        assert_eq!(atom_count_for_element("C6H5Br", "Br"), 1);
        assert_eq!(atom_count_for_element("C6H5Na", "Na"), 1);
    }

    #[test]
    fn does_not_count_substrings_inside_other_elements() {
        assert_eq!(atom_count_for_element("C6H5Fe", "F"), 0);
        assert_eq!(atom_count_for_element("C6H5Si", "I"), 0);
        assert_eq!(atom_count_for_element("NaCl", "N"), 0);
        assert_eq!(atom_count_for_element("NaCl", "C"), 0);
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

    #[test]
    fn counts_atoms_in_formula() {
        let counts = element_counts_in_formula("C6H5ClBrNO2");

        assert_eq!(counts.get("C"), Some(&6));
        assert_eq!(counts.get("H"), Some(&5));
        assert_eq!(counts.get("Cl"), Some(&1));
        assert_eq!(counts.get("Br"), Some(&1));
        assert_eq!(counts.get("N"), Some(&1));
        assert_eq!(counts.get("O"), Some(&2));
    }

    #[test]
    fn atom_count_for_element_returns_zero_when_absent() {
        assert_eq!(atom_count_for_element("C6H6", "F"), 0);
        assert_eq!(atom_count_for_element("C6H5F", "F"), 1);
        assert_eq!(atom_count_for_element("C6H4F2", "F"), 2);
    }

    #[test]
    fn missing_numeric_subscripts_count_as_one() {
        let counts = element_counts_in_formula("C20H25IN2O5");

        assert_eq!(counts.get("I"), Some(&1));
        assert_eq!(counts.get("N"), Some(&2));
        assert_eq!(counts.get("O"), Some(&5));
    }

    #[test]
    fn repeated_elements_are_summed() {
        let counts = element_counts_in_formula("C6H5OO");

        assert_eq!(counts.get("O"), Some(&2));
    }
}
