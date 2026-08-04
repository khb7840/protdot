use std::collections::{BTreeMap, BTreeSet};

use crate::atom::Atom;

pub struct ChainSummary {
    pub id: String,
    pub atom_count: usize,
    pub residue_count: usize,
}

pub struct StructureSummary {
    pub atom_count: usize,
    pub residue_count: usize,
    pub chain_summaries: Vec<ChainSummary>,
    pub element_counts: Vec<(String, usize)>,
    pub residue_group_counts: Vec<(String, usize)>,
    pub residue_type_counts: Vec<(String, usize)>,
    pub min_residue_num: i32,
    pub max_residue_num: i32,
}

impl StructureSummary {
    pub fn from_atoms(atoms: &[Atom]) -> Self {
        let mut chain_atom_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut chain_residue_numbers: BTreeMap<String, BTreeSet<i32>> = BTreeMap::new();
        let mut element_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut residues: BTreeMap<(String, i32), String> = BTreeMap::new();

        for atom in atoms {
            *chain_atom_counts.entry(atom.chain.clone()).or_default() += 1;
            chain_residue_numbers
                .entry(atom.chain.clone())
                .or_default()
                .insert(atom.residue_num);
            *element_counts.entry(atom.element.clone()).or_default() += 1;
            residues
                .entry((atom.chain.clone(), atom.residue_num))
                .or_insert_with(|| atom.residue.clone());
        }

        let mut chain_summaries: Vec<_> = chain_atom_counts
            .into_iter()
            .map(|(id, atom_count)| ChainSummary {
                residue_count: chain_residue_numbers
                    .get(&id)
                    .map(BTreeSet::len)
                    .unwrap_or_default(),
                id,
                atom_count,
            })
            .collect();
        chain_summaries.sort_by(|a, b| {
            b.atom_count
                .cmp(&a.atom_count)
                .then_with(|| b.residue_count.cmp(&a.residue_count))
                .then_with(|| a.id.cmp(&b.id))
        });

        let mut residue_group_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut residue_type_counts: BTreeMap<String, usize> = BTreeMap::new();

        for residue in residues.values() {
            if let Some(group) = Atom::amino_acid_group(residue) {
                *residue_group_counts.entry(group.to_string()).or_default() += 1;
            }
            *residue_type_counts.entry(residue.clone()).or_default() += 1;
        }

        let min_residue_num = residues
            .keys()
            .map(|(_, residue_num)| *residue_num)
            .min()
            .unwrap_or(0);
        let max_residue_num = residues
            .keys()
            .map(|(_, residue_num)| *residue_num)
            .max()
            .unwrap_or(0);

        Self {
            atom_count: atoms.len(),
            residue_count: residues.len(),
            chain_summaries,
            element_counts: sort_counts_desc(element_counts),
            residue_group_counts: sort_counts_desc(residue_group_counts),
            residue_type_counts: sort_counts_desc(residue_type_counts),
            min_residue_num,
            max_residue_num,
        }
    }
}

fn sort_counts_desc(counts: BTreeMap<String, usize>) -> Vec<(String, usize)> {
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    counts
}
