use mascot_rs::prelude::MascotGenericFormatMetadata;

pub fn metadata_value(metadata: &MascotGenericFormatMetadata, key: &str) -> String {
    metadata
        .arbitrary_metadata_value(key)
        .unwrap_or_default()
        .to_string()
}
