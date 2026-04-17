// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use semver::Version;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum VersionError {
    MinExceedsExact { min: Version, exact: Version },
    MaxPrecedesExact { max: Version, exact: Version },
    MinExceedsMax { min: Version, max: Version },
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::MinExceedsExact { min, exact } => {
                write!(f, "min version {min} is greater than exact version {exact}")
            }
            VersionError::MaxPrecedesExact { max, exact } => {
                write!(f, "max version {max} is earlier than exact version {exact}")
            }
            VersionError::MinExceedsMax { min, max } => {
                write!(f, "min version {min} is greater than max version {max}")
            }
        }
    }
}

fn supported_qt_versions() -> Vec<Version> {
    vec![
        Version::new(6, 2, 0),
        Version::new(6, 3, 0),
        Version::new(6, 4, 0),
        Version::new(6, 5, 0),
        Version::new(6, 6, 0),
        Version::new(6, 7, 0),
        Version::new(6, 8, 0),
        Version::new(6, 9, 0),
        Version::new(6, 10, 0),
        Version::new(6, 11, 0),
        Version::new(6, 12, 0),
    ]
}

fn default_version() -> Option<Version> {
    if cfg!(feature = "qt_version_default") {
        Some(Version::new(6, 10, 2))
    } else {
        None
    }
}

fn exact_version() -> Option<Version> {
    #[cfg(feature = "qt_version_exact_6_12_0")]
    {
        return Some(Version::new(6, 12, 0));
    }
    #[cfg(feature = "qt_version_exact_6_11_0")]
    {
        return Some(Version::new(6, 11, 0));
    }
    #[cfg(feature = "qt_version_exact_6_10_2")]
    {
        return Some(Version::new(6, 10, 2));
    }
    #[cfg(feature = "qt_version_exact_6_10_1")]
    {
        return Some(Version::new(6, 10, 1));
    }
    #[cfg(feature = "qt_version_exact_6_10_0")]
    {
        return Some(Version::new(6, 10, 0));
    }
    #[cfg(feature = "qt_version_exact_6_9_3")]
    {
        return Some(Version::new(6, 9, 3));
    }
    #[cfg(feature = "qt_version_exact_6_9_2")]
    {
        return Some(Version::new(6, 9, 2));
    }
    #[cfg(feature = "qt_version_exact_6_9_1")]
    {
        return Some(Version::new(6, 9, 1));
    }
    #[cfg(feature = "qt_version_exact_6_9_0")]
    {
        return Some(Version::new(6, 9, 0));
    }
    #[cfg(feature = "qt_version_exact_6_8_6")]
    {
        return Some(Version::new(6, 8, 6));
    }
    #[cfg(feature = "qt_version_exact_6_8_5")]
    {
        return Some(Version::new(6, 8, 5));
    }
    #[cfg(feature = "qt_version_exact_6_8_4")]
    {
        return Some(Version::new(6, 8, 4));
    }
    #[cfg(feature = "qt_version_exact_6_8_3")]
    {
        return Some(Version::new(6, 8, 3));
    }
    #[cfg(feature = "qt_version_exact_6_8_2")]
    {
        return Some(Version::new(6, 8, 2));
    }
    #[cfg(feature = "qt_version_exact_6_8_1")]
    {
        return Some(Version::new(6, 8, 1));
    }
    #[cfg(feature = "qt_version_exact_6_8_0")]
    {
        return Some(Version::new(6, 8, 0));
    }
    #[cfg(feature = "qt_version_exact_6_7_0")]
    {
        return Some(Version::new(6, 7, 0));
    }
    #[cfg(feature = "qt_version_exact_6_6_0")]
    {
        return Some(Version::new(6, 6, 0));
    }
    #[cfg(feature = "qt_version_exact_6_5_0")]
    {
        return Some(Version::new(6, 5, 0));
    }
    #[cfg(feature = "qt_version_exact_6_4_0")]
    {
        return Some(Version::new(6, 4, 0));
    }
    #[cfg(feature = "qt_version_exact_6_3_0")]
    {
        return Some(Version::new(6, 3, 0));
    }
    #[cfg(feature = "qt_version_exact_6_2_0")]
    {
        return Some(Version::new(6, 2, 0));
    }
    None
}

fn min_version() -> Option<Version> {
    #[cfg(feature = "qt_version_at_least_6_12")]
    {
        return Some(Version::new(6, 12, 0));
    }
    #[cfg(feature = "qt_version_at_least_6_11")]
    {
        return Some(Version::new(6, 11, 0));
    }
    #[cfg(feature = "qt_version_at_least_6_10")]
    {
        return Some(Version::new(6, 10, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_9")]
    {
        return Some(Version::new(6, 9, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_8")]
    {
        return Some(Version::new(6, 8, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_7")]
    {
        return Some(Version::new(6, 7, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_6")]
    {
        return Some(Version::new(6, 6, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_5")]
    {
        return Some(Version::new(6, 5, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_4")]
    {
        return Some(Version::new(6, 4, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_3")]
    {
        return Some(Version::new(6, 3, 0));
    }

    #[cfg(feature = "qt_version_at_least_6_2")]
    {
        return Some(Version::new(6, 2, 0));
    }

    None
}

fn max_version() -> Option<Version> {
    #[cfg(feature = "qt_version_at_most_6_2")]
    {
        return Some(Version::new(6, 2, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_3")]
    {
        return Some(Version::new(6, 3, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_4")]
    {
        return Some(Version::new(6, 4, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_5")]
    {
        return Some(Version::new(6, 5, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_6")]
    {
        return Some(Version::new(6, 6, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_7")]
    {
        return Some(Version::new(6, 7, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_8")]
    {
        return Some(Version::new(6, 8, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_9")]
    {
        return Some(Version::new(6, 9, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_10")]
    {
        return Some(Version::new(6, 10, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_11")]
    {
        return Some(Version::new(6, 11, 0));
    }
    #[cfg(feature = "qt_version_at_most_6_12")]
    {
        return Some(Version::new(6, 12, 0));
    }

    None
}

fn resolve_versions(
    supported: &[Version],
    exact: Option<Version>,
    min: Option<Version>,
    max: Option<Version>,
) -> Result<Vec<Version>, VersionError> {
    if let Some(exact_v) = exact {
        if let Some(min_v) = min {
            if min_v > exact_v {
                return Err(VersionError::MinExceedsExact {
                    min: min_v,
                    exact: exact_v,
                });
            }
        }
        if let Some(max_v) = max {
            if max_v < exact_v {
                return Err(VersionError::MaxPrecedesExact {
                    max: max_v,
                    exact: exact_v,
                });
            }
        }
        return Ok(vec![exact_v]);
    }

    if let (Some(min_v), Some(max_v)) = (min.as_ref(), max.as_ref()) {
        if min_v > max_v {
            return Err(VersionError::MinExceedsMax {
                min: min_v.clone(),
                max: max_v.clone(),
            });
        }
    }
    Ok(supported
        .iter()
        .filter(|&v| {
            let ok_min = min.as_ref().is_none_or(|m| *v >= *m);
            let ok_max = max.as_ref().is_none_or(|m| *v <= *m);
            ok_min && ok_max
        })
        .cloned()
        .collect())
}

pub fn qt_versions() -> Vec<Version> {
    let supported = supported_qt_versions();
    let exact = exact_version().or(default_version());
    let min = min_version();
    let max = max_version();

    resolve_versions(&supported, exact, min, max)
        .expect("invalid Qt version feature flag combination")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_filter() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(
            &supported,
            None,
            Some(Version::new(6, 7, 0)),
            Some(Version::new(6, 9, 0)),
        );

        assert_eq!(
            result,
            Ok(vec![
                Version::new(6, 7, 0),
                Version::new(6, 8, 0),
                Version::new(6, 9, 0),
            ])
        );
    }

    #[test]
    fn test_exact() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(
            &supported,
            Some(Version::new(6, 6, 0)),
            Some(Version::new(6, 5, 0)),
            Some(Version::new(6, 7, 0)),
        );

        assert_eq!(result, Ok(vec![Version::new(6, 6, 0)]));
    }

    #[test]
    fn test_exact_smaller_than_range() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(
            &supported,
            Some(Version::new(6, 3, 0)),
            Some(Version::new(6, 5, 0)),
            Some(Version::new(6, 7, 0)),
        );

        assert_eq!(
            result,
            Err(VersionError::MinExceedsExact {
                min: Version::new(6, 5, 0),
                exact: Version::new(6, 3, 0),
            })
        );
    }

    #[test]
    fn test_exact_bigger_than_range() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(
            &supported,
            Some(Version::new(6, 9, 0)),
            Some(Version::new(6, 5, 0)),
            Some(Version::new(6, 7, 0)),
        );

        assert_eq!(
            result,
            Err(VersionError::MaxPrecedesExact {
                max: Version::new(6, 7, 0),
                exact: Version::new(6, 9, 0),
            })
        );
    }

    #[test]
    fn test_empty_range() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(
            &supported,
            None,
            Some(Version::new(6, 7, 0)),
            Some(Version::new(6, 6, 0)),
        );

        assert_eq!(
            result,
            Err(VersionError::MinExceedsMax {
                min: Version::new(6, 7, 0),
                max: Version::new(6, 6, 0),
            })
        );
    }

    #[test]
    fn test_no_minimum() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(&supported, None, None, Some(Version::new(6, 9, 0)));

        assert_eq!(
            result,
            Ok(vec![
                Version::new(6, 5, 0),
                Version::new(6, 6, 0),
                Version::new(6, 7, 0),
                Version::new(6, 8, 0),
                Version::new(6, 9, 0),
            ])
        );
    }

    #[test]
    fn test_no_maximum() {
        let supported = vec![
            Version::new(6, 5, 0),
            Version::new(6, 6, 0),
            Version::new(6, 7, 0),
            Version::new(6, 8, 0),
            Version::new(6, 9, 0),
            Version::new(6, 10, 0),
        ];

        let result = resolve_versions(&supported, None, Some(Version::new(6, 7, 0)), None);

        assert_eq!(
            result,
            Ok(vec![
                Version::new(6, 7, 0),
                Version::new(6, 8, 0),
                Version::new(6, 9, 0),
                Version::new(6, 10, 0),
            ])
        );
    }
}
