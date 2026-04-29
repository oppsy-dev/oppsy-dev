use std::cmp::Ordering;

use osv_db::types::Event;

/// Compares two Maven version strings using a simplified Maven version ordering
/// algorithm.
///
/// Rules:
/// 1. The version is split into a numeric component sequence and an optional qualifier
///    (separated by the first `-`).
/// 2. Numeric components (separated by `.`) are compared as unsigned integers, with
///    missing components treated as `0` (so `"1.5"` equals `"1.5.0"`).
/// 3. Qualifiers follow Maven's ordering: `alpha` < `beta` < `milestone` < `rc` <
///    `snapshot` < `""` (release) < `sp`.
pub(crate) fn compare(
    a: &str,
    b: &str,
) -> Ordering {
    let (a_nums, a_qual) = parse(a);
    let (b_nums, b_qual) = parse(b);

    let len = a_nums.len().max(b_nums.len());
    for i in 0..len {
        let av = a_nums.get(i).copied().unwrap_or(0);
        let bv = b_nums.get(i).copied().unwrap_or(0);
        match av.cmp(&bv) {
            Ordering::Equal => {},
            ord => return ord,
        }
    }

    qualifier_order(&a_qual).cmp(&qualifier_order(&b_qual))
}

/// Returns `true` if `version` falls within the OSV event window described by `events`,
/// using Maven ecosystem version comparison.
///
/// Follows the OSV evaluation algorithm but replaces semver ordering with [`compare`].
pub(crate) fn ecosystem_range_contains(
    version: &str,
    events: &[Event],
) -> bool {
    let mut introduced = false;
    let mut fixed = false;
    let mut within_limits = true;

    for event in events {
        match event {
            Event::Introduced { introduced: v } => {
                if v == "0" || compare(version, v) != Ordering::Less {
                    introduced = true;
                }
            },
            Event::Fixed { fixed: v } => {
                if compare(version, v) != Ordering::Less {
                    fixed = true;
                }
            },
            Event::LastAffected { last_affected: v } => {
                if compare(version, v) == Ordering::Greater {
                    fixed = true;
                }
            },
            Event::Limit { limit: v } => {
                if compare(version, v) != Ordering::Less {
                    within_limits = false;
                }
            },
        }
    }

    within_limits && introduced && !fixed
}

/// Parses a Maven version string into numeric components and a lowercase qualifier.
///
/// `"2.14.1-SNAPSHOT"` → `([2, 14, 1], "snapshot")`
/// `"1.5"` → `([1, 5], "")`
fn parse(version: &str) -> (Vec<u64>, String) {
    let (numeric_str, qualifier) = version
        .split_once('-')
        .map(|(n, q)| (n, q.to_lowercase()))
        .unwrap_or((version, String::new()));

    let nums = numeric_str
        .split('.')
        .map(|s| s.parse::<u64>().unwrap_or(0))
        .collect();

    (nums, qualifier)
}

/// Maps a qualifier string to a numeric rank for ordering.
///
/// Lower rank means an earlier (pre-release) version.
fn qualifier_order(q: &str) -> i32 {
    if q.starts_with("alpha") {
        return -5;
    }
    if q.starts_with("beta") {
        return -4;
    }
    if q.starts_with("milestone") {
        return -3;
    }
    if q.starts_with("rc") || q.starts_with("cr") {
        return -2;
    }
    if q == "snapshot" {
        return -1;
    }
    if q.is_empty() || q == "final" || q == "release" || q == "ga" {
        return 0;
    }
    if q.starts_with("sp") {
        return 1;
    }
    // Unknown qualifiers are treated as pre-release.
    -1
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("1.5", "1.5.0" => Ordering::Equal; "1.5 equals 1.5.0")]
    #[test_case("1.9", "1.10.0" => Ordering::Less; "1.9 is less than 1.10.0")]
    #[test_case("1.10.0", "1.9" => Ordering::Greater; "1.10.0 is greater than 1.9")]
    #[test_case("2.14.1", "2.15.0" => Ordering::Less; "2.14.1 is less than 2.15.0")]
    #[test_case("2.13.0", "2.14.1" => Ordering::Less; "2.13.0 is less than 2.14.1")]
    #[test_case("2.0-beta9", "2.0.0" => Ordering::Less; "beta is pre-release")]
    #[test_case("1.0-snapshot", "1.0.0" => Ordering::Less; "snapshot is pre-release")]
    #[test_case("1.0-rc1", "1.0.0" => Ordering::Less; "rc is pre-release")]
    #[test_case("1.0-sp1", "1.0.0" => Ordering::Greater; "sp is a service pack, after release")]
    fn compare_test(
        a: &str,
        b: &str,
    ) -> Ordering {
        compare(a, b)
    }

    #[test_case("1.9", &[Event::Introduced { introduced: "1.5".into() }, Event::Fixed { fixed: "1.10.0".into() }] => true; "1.9 inside [1.5, 1.10.0)")]
    #[test_case("1.10.0", &[Event::Introduced { introduced: "1.5".into() }, Event::Fixed { fixed: "1.10.0".into() }] => false; "1.10.0 at fixed bound is excluded")]
    #[test_case("2.14.1", &[Event::Introduced { introduced: "2.13.0".into() }, Event::Fixed { fixed: "2.15.0".into() }] => true; "2.14.1 inside [2.13.0, 2.15.0)")]
    #[test_case("2.15.0", &[Event::Introduced { introduced: "2.13.0".into() }, Event::Fixed { fixed: "2.15.0".into() }] => false; "2.15.0 at fixed bound is excluded")]
    #[test_case("0.9", &[Event::Introduced { introduced: "0".into() }, Event::Fixed { fixed: "1.0.0".into() }] => true; "introduced zero matches all from beginning")]
    fn ecosystem_range_test(
        version: &str,
        events: &[Event],
    ) -> bool {
        ecosystem_range_contains(version, events)
    }
}
