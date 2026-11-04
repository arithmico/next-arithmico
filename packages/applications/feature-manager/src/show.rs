use std::collections::{BTreeMap, BTreeSet};

use crate::verify::verify;

pub type NormalizeOutput = BTreeMap<String, BTreeSet<String>>;

pub fn normalize() -> anyhow::Result<NormalizeOutput> {
    let features = verify()?;
    let mut output = BTreeMap::<String, BTreeSet<String>>::new();
    for (feature, dependencies) in features.evaluator.features.iter() {
        let entry = output.entry(feature.clone()).or_default();
        entry.extend(dependencies.iter().map(|v| v.to_string()));
    }
    for (feature, dependencies) in features.engine.features.iter() {
        let entry = output.entry(feature.clone()).or_default();
        entry.extend(
            dependencies
                .iter()
                .map(|v| v.strip_prefix("evaluator/").unwrap_or(v).to_string()),
        );
    }
    for (feature, dependencies) in features.engine_worker.features.iter() {
        let entry = output.entry(feature.clone()).or_default();
        entry.extend(
            dependencies
                .iter()
                .map(|v| {
                    v.strip_prefix("evaluator/")
                        .or_else(|| v.strip_prefix("engine/"))
                        .unwrap_or(v)
                        .to_string()
                })
                .filter(|v| v != feature),
        );
    }

    Ok(output)
}
