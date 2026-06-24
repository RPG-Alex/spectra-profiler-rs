# `Cl` profile for `annotated_ms2`

This report summarizes how often the target element `Cl` appears across metadata groups in `annotated_ms2`.


## Numeric summary

| Metric | Value |
|---|---:|
| Total spectra | 443905 |
| Positive count | 27230 |
| Negative count | 416675 |
| Positive percentage | 6.1342% |

## Top enriched groups

These are the most target-enriched metadata groups with at least `30` total spectra.

| Metadata group | Value | Total | Positive | Positive % | % of positives |
|---|---|---:|---:|---:|---:|
| NPC classes | Vancomycins and Teicoplanins | 52 | 52 | 100.00% | 0.19% |
| NPC classes | Halogenated hydrocarbons | 124 | 119 | 95.97% | 0.44% |
| NPC classes | Griseofulvins | 343 | 310 | 90.38% | 1.14% |
| NPC classes | Azaphilones | 197 | 104 | 52.79% | 0.38% |
| NPC classes | Pyrrole alkaloids | 241 | 123 | 51.04% | 0.45% |
| NPC superclasses | Proline alkaloids | 254 | 123 | 48.43% | 0.45% |
| NPC classes | Benzodiazepine alkaloids | 1332 | 452 | 33.93% | 1.66% |
| Organism | GNPS-NIH-SMALLMOLECULEPHARMACOLOGICALLYACTIVE | 906 | 296 | 32.67% | 1.09% |

## Low-support warning summary

| Warning | Count |
|---|---:|
| `LOW_TARGET_SUPPORT` | 77 |
| `LOW_TOTAL_SUPPORT` | 204 |
| `NO_TARGET_POSITIVES` | 411 |
## Summary

- [Summary table](tables/summary.csv)
- Tables are in [`tables/`](tables/)
- Figures are in [`figures/`](figures/)

## How to read the figures

- **Target count** shows which groups contribute the most target-positive spectra.
- **Percent target** shows which groups are most enriched for the target element.
- Small groups can look highly enriched, so check the linked CSV tables for support counts.

## NPC pathways

Natural-product pathway-level distribution for the target element.

[CSV table](tables/contains_by_npc_pathways.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_npc_pathways_by_target_count.svg" alt="NPC pathways by target count" /></td>
<td width="50%"><img src="figures/top_npc_pathways_by_percent_target.svg" alt="NPC pathways by percent target" /></td>
</tr>
</table>

## NPC superclasses

Natural-product superclass-level distribution for the target element.

[CSV table](tables/contains_by_npc_superclasses.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_npc_superclasses_by_target_count.svg" alt="NPC superclasses by target count" /></td>
<td width="50%"><img src="figures/top_npc_superclasses_by_percent_target.svg" alt="NPC superclasses by percent target" /></td>
</tr>
</table>

## NPC classes

Natural-product class-level distribution for the target element.

[CSV table](tables/contains_by_npc_classes.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_npc_classes_by_target_count.svg" alt="NPC classes by target count" /></td>
<td width="50%"><img src="figures/top_npc_classes_by_percent_target.svg" alt="NPC classes by percent target" /></td>
</tr>
</table>

## Source dataset

Distribution by original source dataset.

[CSV table](tables/contains_by_source_dataset.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_source_dataset_by_target_count.svg" alt="Source dataset by target count" /></td>
<td width="50%"><img src="figures/top_source_dataset_by_percent_target.svg" alt="Source dataset by percent target" /></td>
</tr>
</table>

## Organism

Distribution by organism/source organism metadata.

[CSV table](tables/contains_by_organism.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_organism_by_target_count.svg" alt="Organism by target count" /></td>
<td width="50%"><img src="figures/top_organism_by_percent_target.svg" alt="Organism by percent target" /></td>
</tr>
</table>

## Ion mode

Distribution by recorded ion mode.

[CSV table](tables/contains_by_ion_mode.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_ion_mode_by_target_count.svg" alt="Ion mode by target count" /></td>
<td width="50%"><img src="figures/top_ion_mode_by_percent_target.svg" alt="Ion mode by percent target" /></td>
</tr>
</table>

## Source instrument

Distribution by recorded source instrument.

[CSV table](tables/contains_by_source_instrument.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_source_instrument_by_target_count.svg" alt="Source instrument by target count" /></td>
<td width="50%"><img src="figures/top_source_instrument_by_percent_target.svg" alt="Source instrument by percent target" /></td>
</tr>
</table>

## Library quality

Distribution by library quality metadata.

[CSV table](tables/contains_by_library_quality.csv)

<table>
<tr>
<th>Top groups by target count</th>
<th>Top groups by percent target</th>
</tr>
<tr>
<td width="50%"><img src="figures/top_library_quality_by_target_count.svg" alt="Library quality by target count" /></td>
<td width="50%"><img src="figures/top_library_quality_by_percent_target.svg" alt="Library quality by percent target" /></td>
</tr>
</table>
