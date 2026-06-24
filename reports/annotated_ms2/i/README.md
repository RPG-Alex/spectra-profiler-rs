# `I` profile for `annotated_ms2`

This report summarizes how often the target element `I` appears across metadata groups in `annotated_ms2`.


## Numeric summary

| Metric | Value |
|---|---:|
| Total spectra | 443905 |
| Positive count | 1338 |
| Negative count | 442567 |
| Positive percentage | 0.3014% |

## Top enriched groups

These are the most target-enriched metadata groups with at least `30` total spectra.

| Metadata group | Value | Total | Positive | Positive % | % of positives |
|---|---|---:|---:|---:|---:|
| NPC classes | Phenoxazine alkaloids | 140 | 39 | 27.86% | 2.91% |
| NPC classes | Simple amide alkaloids | 3329 | 190 | 5.71% | 14.20% |
| NPC classes | Oligomeric stibenes | 164 | 8 | 4.88% | 0.60% |
| NPC classes | Aporphine alkaloids | 1122 | 48 | 4.28% | 3.59% |
| NPC superclasses | Peptide alkaloids | 4875 | 195 | 4.00% | 14.57% |
| NPC classes | Pyrrolizidine alkaloids | 1913 | 51 | 2.67% | 3.81% |
| Organism | GNPS-NIH-SMALLMOLECULEPHARMACOLOGICALLYACTIVE | 906 | 23 | 2.54% | 1.72% |
| NPC classes | Chromones | 2778 | 66 | 2.38% | 4.93% |

## Low-support warning summary

| Warning | Count |
|---|---:|
| `LOW_TARGET_SUPPORT` | 40 |
| `LOW_TOTAL_SUPPORT` | 204 |
| `NO_TARGET_POSITIVES` | 566 |
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
