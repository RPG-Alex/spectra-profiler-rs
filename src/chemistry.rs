use std::collections::{BTreeMap, BTreeSet};

use molecular_formulas::prelude::{MolecularFormula, MolecularFormulaMetadata};

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

/// Returns the valid element symbols present in a structured molecular formula.
pub fn element_symbols_in_formula<F>(formula: &F) -> BTreeSet<String>
where
    F: MolecularFormula,
    u32: From<<F as MolecularFormulaMetadata>::Count>,
{
    element_counts_in_formula(formula).into_keys().collect()
}

/// Counts atoms for each valid element symbol in a structured molecular
/// formula.
pub fn element_counts_in_formula<F>(formula: &F) -> BTreeMap<String, usize>
where
    F: MolecularFormula,
    u32: From<<F as MolecularFormulaMetadata>::Count>,
{
    let Ok(counts) = formula.element_count_vector::<u32>() else {
        return BTreeMap::new();
    };

    counts
        .into_iter()
        .enumerate()
        .filter_map(|(index, count)| {
            if count == 0 {
                return None;
            }

            let symbol = ELEMENT_SYMBOLS.get(index)?;

            Some(((*symbol).to_string(), count as usize))
        })
        .collect()
}

/// Returns the atom count for one element in a structured molecular formula.
pub fn atom_count_for_element<F>(formula: &F, target: &str) -> usize
where
    F: MolecularFormula,
    u32: From<<F as MolecularFormulaMetadata>::Count>,
{
    let Some(index) = ELEMENT_SYMBOLS.iter().position(|symbol| *symbol == target) else {
        return 0;
    };

    let Ok(counts) = formula.element_count_vector::<u32>() else {
        return 0;
    };

    counts.get(index).copied().unwrap_or_default() as usize
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use molecular_formulas::prelude::ChemicalFormula;

    use super::*;

    fn formula(input: &str) -> ChemicalFormula<u32, i32> {
        ChemicalFormula::from_str(input).unwrap()
    }
    #[test]
    fn counts_single_letter_elements() {
        let first = formula("C6H6F");
        let second = formula("C20H25IN2O5");
        let third = formula("C10H14N2");

        assert_eq!(atom_count_for_element(&first, "F"), 1);
        assert_eq!(atom_count_for_element(&second, "I"), 1);
        assert_eq!(atom_count_for_element(&third, "N"), 2);
    }

    #[test]
    fn counts_two_letter_elements() {
        let chlorine = formula("C6H5Cl");
        let bromine = formula("C6H5Br");
        let sodium = formula("C6H5Na");

        assert_eq!(atom_count_for_element(&chlorine, "Cl"), 1);
        assert_eq!(atom_count_for_element(&bromine, "Br"), 1);
        assert_eq!(atom_count_for_element(&sodium, "Na"), 1);
    }

    #[test]
    fn does_not_count_substrings_inside_other_elements() {
        let iron = formula("C6H5Fe");
        let silicon = formula("C6H5Si");
        let salt = formula("NaCl");

        assert_eq!(atom_count_for_element(&iron, "F"), 0);
        assert_eq!(atom_count_for_element(&silicon, "I"), 0);
        assert_eq!(atom_count_for_element(&salt, "N"), 0);
        assert_eq!(atom_count_for_element(&salt, "C"), 0);
    }

    #[test]
    fn counts_atoms_in_formula() {
        let formula = formula("C6H5ClBrNO2");
        let counts = element_counts_in_formula(&formula);

        assert_eq!(counts.get("C"), Some(&6));
        assert_eq!(counts.get("H"), Some(&5));
        assert_eq!(counts.get("Cl"), Some(&1));
        assert_eq!(counts.get("Br"), Some(&1));
        assert_eq!(counts.get("N"), Some(&1));
        assert_eq!(counts.get("O"), Some(&2));
    }

    #[test]
    fn atom_count_for_element_returns_zero_when_absent() {
        let benzene = formula("C6H6");
        let fluoro_benzene = formula("C6H5F");
        let difluoro_benzene = formula("C6H4F2");

        assert_eq!(atom_count_for_element(&benzene, "F"), 0);
        assert_eq!(atom_count_for_element(&fluoro_benzene, "F"), 1);
        assert_eq!(atom_count_for_element(&difluoro_benzene, "F"), 2);
    }

    #[test]
    fn counts_large_carbon_and_hydrogen_values() {
        let formula = formula("C28H23ClO7");

        assert_eq!(atom_count_for_element(&formula, "C"), 28);
        assert_eq!(atom_count_for_element(&formula, "H"), 23);
        assert_eq!(atom_count_for_element(&formula, "Cl"), 1);
        assert_eq!(atom_count_for_element(&formula, "O"), 7);
    }
}
