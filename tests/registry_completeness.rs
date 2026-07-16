//! Guards [`ALL_ZMANIM`] against drift from the actual accessor methods.
//!
//! The registry macro already makes a renamed or removed method a compile
//! error. This test covers the other direction: a newly *added* accessor that
//! was not added to the list. It parses the `ComplexZmanimCalendar` source and
//! asserts that every zero-argument *zman* method appears in the registry with
//! the correct [`ZmanKind`], and vice versa.

// Yes, it's a bit of a hack, but it works, and the only other method I could
// think of was implementing the registry through macros in `czc_struct.rs`,
// which was too invasive for my liking

use rust_zmanim::complex_zmanim_calendar::{ALL_ZMANIM, ZmanKind};
use std::collections::BTreeMap;

const SRC: &str = include_str!("../src/complex_zmanim_calendar/czc_struct.rs");

/// Collapses all whitespace out of a string, for signature matching that is
/// immune to future line-wrapping / reformatting.
fn squeeze(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Parses `czc_struct.rs` and returns the expected registry as `name -> kind`.
///
/// Rule A (hand-written methods): every
/// `pub fn NAME(&self) -> Option<Zoned|SignedDuration>`
/// Rule B (macro-generated methods): every `NAME => zman_type,` line whose
/// `zman_type` is one of the generic `&ZmanOffset`-taking accessors (collected
/// as a byproduct of Rule A), classified as a duration iff `zman_type` is
/// `shaah_zmanis_mga`.
fn parse_expected() -> BTreeMap<String, ZmanKind> {
    let mut expected: BTreeMap<String, ZmanKind> = BTreeMap::new();
    // Names of accessors that take `(&self, offset: &ZmanOffset)` - the valid
    // right-hand sides of a `zmanim_for_offset!` table entry.
    let mut offset_methods: std::collections::BTreeSet<String> = Default::default();

    // Rule A: scan every `pub fn` signature.
    // loop through all offsets where "pub fn " occurs
    for (start, _) in SRC.match_indices("pub fn ") {
        // get the fn ident
        let after = &SRC[start + "pub fn ".len()..];
        let Some(paren) = after.find('(') else {
            continue;
        };
        let name = after[..paren].trim();
        if name.is_empty() || !is_ident(name) {
            continue;
        }

        // get the fn params
        let Some(close) = after.find(')') else {
            continue;
        };
        let params = squeeze(&after[paren + 1..close]);

        // get fn return type
        let rest = &after[close + 1..];
        let Some(arrow) = rest.find("->") else {
            continue;
        };
        let Some(brace) = rest.find('{') else {
            continue;
        };
        if brace < arrow {
            continue;
        }
        let ret = squeeze(&rest[arrow + 2..brace]);

        // determine whether/how it should be included in `expected`
        if params == "&self" {
            match ret.as_str() {
                "Option<Zoned>" => {
                    expected.insert(name.to_string(), ZmanKind::Time);
                }
                "Option<SignedDuration>" => {
                    expected.insert(name.to_string(), ZmanKind::Duration);
                }
                _ => {}
            }
        } else if params == "&self,offset:&ZmanOffset" {
            offset_methods.insert(name.to_string());
        }
    }

    // Rule B: scan `zmanim_for_offset!` table lines `NAME => zman_type,`.
    for line in SRC.lines() {
        let line = line.trim();
        let Some((lhs, rhs)) = line.split_once("=>") else {
            continue;
        };
        let lhs = lhs.trim();
        if !is_ident(lhs) {
            continue;
        }
        let zman_type = rhs.trim().trim_end_matches(',');
        let zman_type = zman_type.split(',').next().unwrap_or("").trim();

        // the `$zman_type` in `zmanim_for_offset!` is used as a method ident of
        // a generic zman getter: `self.$zman_type(&offset)`. so it must be in
        // `offset_methods` if it exists
        if !offset_methods.contains(zman_type) {
            continue;
        }
        let kind = if zman_type == "shaah_zmanis_mga" {
            ZmanKind::Duration
        } else {
            ZmanKind::Time
        };
        expected.insert(lhs.to_string(), kind);
    }

    expected
}

/// True if `s` is a plausible Rust snake_case identifier.
/// Not worth including `syn` dep just for one non-critical check
fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && s.starts_with(|c: char| c.is_ascii_lowercase() || c == '_')
}

#[test]
fn registry_matches_source() {
    let expected = parse_expected();

    // Guard against parser rot: if the parser silently stops matching, this
    // floor forces a failure rather than a false pass. 174 is the current
    // number, and only expected to increase, so good sanity check
    assert!(
        expected.len() >= 174,
        "source parser found only {} zmanim (expected >= 174); the parser likely broke",
        expected.len()
    );

    let actual: BTreeMap<String, ZmanKind> = ALL_ZMANIM
        .iter()
        .map(|e| (e.name.to_string(), e.kind))
        .collect();

    let mut missing = Vec::new(); // in source, not in registry
    let mut kind_mismatch = Vec::new();
    for (name, kind) in &expected {
        match actual.get(name) {
            None => missing.push((name.clone(), *kind)),
            Some(a) if a != kind => kind_mismatch.push((name.clone(), *kind, *a)),
            _ => {}
        }
    }
    let extra: Vec<_> = actual
        .keys()
        .filter(|n| !expected.contains_key(*n))
        .cloned()
        .collect();

    if missing.is_empty() && extra.is_empty() && kind_mismatch.is_empty() {
        return;
    }

    let mut msg = String::from("registry is out of sync with czc_struct.rs\n");
    if !missing.is_empty() {
        msg.push_str("\nmissing from ALL_ZMANIM (add these to registry.rs):\n");
        for (name, kind) in &missing {
            let group = if *kind == ZmanKind::Time {
                "times"
            } else {
                "durations"
            };
            msg.push_str(&format!("    {name},   // -> {group}\n"));
        }
    }
    if !extra.is_empty() {
        msg.push_str("\nin ALL_ZMANIM but not found in source (remove or fix):\n");
        for name in &extra {
            msg.push_str(&format!("    {name}\n"));
        }
    }
    if !kind_mismatch.is_empty() {
        msg.push_str("\nkind mismatch (source kind vs registry kind):\n");
        for (name, exp, act) in &kind_mismatch {
            msg.push_str(&format!("    {name}: source={exp:?} registry={act:?}\n"));
        }
    }
    panic!("{msg}");
}
