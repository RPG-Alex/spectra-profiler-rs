# `Si` profile for `annotated_ms2`

This report summarizes how often the target element `Si` appears across metadata groups in `annotated_ms2`.


## Numeric summary

| Metric | Value |
|---|---:|
| Total spectra | 443905 |
| Positive count | 77 |
| Negative count | 443828 |
| Positive percentage | 0.0173% |

## Top enriched groups

These are the most target-enriched metadata groups with at least `30` total spectra.

| Metadata group | Value | Total | Positive | Positive % | % of positives |
|---|---|---:|---:|---:|---:|
| Organism | GNPS-COLLECTIONS-PESTICIDES-POSITIVE | 109 | 2 | 1.83% | 2.60% |
| NPC classes | Pyrroloquinoline alkaloids | 752 | 7 | 0.93% | 9.09% |
| NPC superclasses | Histidine alkaloids | 5249 | 48 | 0.91% | 62.34% |
| NPC classes | Imidazole alkaloids | 5249 | 48 | 0.91% | 62.34% |
| NPC classes | Hydrocarbons | 907 | 7 | 0.77% | 9.09% |
| Source instrument | FourierTransform | 1200 | 6 | 0.50% | 7.79% |
| NPC superclasses | Fatty acyls | 1885 | 7 | 0.37% | 9.09% |
| Organism | GNPS-NIST14-MATCHES | 2631 | 7 | 0.27% | 9.09% |

## Low-support warning summary

| Warning | Count |
|---|---:|
| `LOW_TARGET_SUPPORT` | 22 |
| `LOW_TOTAL_SUPPORT` | 204 |
| `NO_TARGET_POSITIVES` | 640 |
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
