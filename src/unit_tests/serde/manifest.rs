use crate::types::addon::{Manifest, ManifestBehaviorHints};
use crate::unit_tests::serde::default_tokens_ext::DefaultTokens;
use semver::Version;
use serde_test::{assert_de_tokens, assert_ser_tokens, Configure, Token};
use url::Url;

#[test]
fn manifest() {
    assert_ser_tokens(
        &vec![
            Manifest {
                id: "id".to_owned(),
                version: Version::new(0, 0, 1),
                name: "name".to_owned(),
                contact_email: Some("contact_email".to_owned()),
                description: Some("description".to_owned()),
                logo: Some(Url::parse("https://logo").unwrap()),
                background: Some(Url::parse("https://background").unwrap()),
                types: vec!["type".to_owned()],
                resources: vec![],
                id_prefixes: Some(vec!["id_prefix".to_owned()]),
                catalogs: vec![],
                addon_catalogs: vec![],
                behavior_hints: ManifestBehaviorHints::default(),
            },
            Manifest {
                id: "id".to_owned(),
                version: Version::new(0, 0, 1),
                name: "name".to_owned(),
                contact_email: None,
                description: None,
                logo: None,
                background: None,
                types: vec![],
                resources: vec![],
                id_prefixes: None,
                catalogs: vec![],
                addon_catalogs: vec![],
                behavior_hints: ManifestBehaviorHints::default(),
            },
        ]
        .readable(),
        &[
            vec![
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "Manifest",
                    len: 13,
                },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("version"),
                Token::Str("0.0.1"),
                Token::Str("name"),
                Token::Str("name"),
                Token::Str("contactEmail"),
                Token::Some,
                Token::Str("contact_email"),
                Token::Str("description"),
                Token::Some,
                Token::Str("description"),
                Token::Str("logo"),
                Token::Some,
                Token::Str("https://logo/"),
                Token::Str("background"),
                Token::Some,
                Token::Str("https://background/"),
                Token::Str("types"),
                Token::Seq { len: Some(1) },
                Token::Str("type"),
                Token::SeqEnd,
                Token::Str("resources"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("idPrefixes"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Str("id_prefix"),
                Token::SeqEnd,
                Token::Str("catalogs"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("addonCatalogs"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("behaviorHints"),
            ],
            ManifestBehaviorHints::default_tokens(),
            vec![
                Token::StructEnd,
                Token::Struct {
                    name: "Manifest",
                    len: 13,
                },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("version"),
                Token::Str("0.0.1"),
                Token::Str("name"),
                Token::Str("name"),
                Token::Str("contactEmail"),
                Token::None,
                Token::Str("description"),
                Token::None,
                Token::Str("logo"),
                Token::None,
                Token::Str("background"),
                Token::None,
                Token::Str("types"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("resources"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("idPrefixes"),
                Token::None,
                Token::Str("catalogs"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("addonCatalogs"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("behaviorHints"),
            ],
            ManifestBehaviorHints::default_tokens(),
            vec![Token::StructEnd, Token::SeqEnd],
        ]
        .concat(),
    );
    assert_de_tokens(
        &vec![
            Manifest {
                id: "id".to_owned(),
                version: Version::new(0, 0, 1),
                name: "name".to_owned(),
                contact_email: Some("contact_email".to_owned()),
                description: Some("description".to_owned()),
                logo: Some(Url::parse("https://logo").unwrap()),
                background: Some(Url::parse("https://background").unwrap()),
                types: vec!["type".to_owned()],
                resources: vec![],
                id_prefixes: Some(vec!["id_prefix".to_owned()]),
                catalogs: vec![],
                addon_catalogs: vec![],
                behavior_hints: ManifestBehaviorHints::default(),
            },
            Manifest {
                id: "id".to_owned(),
                version: Version::new(0, 0, 1),
                name: "name".to_owned(),
                contact_email: None,
                description: None,
                logo: None,
                background: None,
                types: vec![],
                resources: vec![],
                id_prefixes: None,
                catalogs: vec![],
                addon_catalogs: vec![],
                behavior_hints: ManifestBehaviorHints::default(),
            },
        ]
        .readable(),
        &[
            vec![
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "Manifest",
                    len: 13,
                },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("version"),
                Token::Str("0.0.1"),
                Token::Str("name"),
                Token::Str("name"),
                Token::Str("contactEmail"),
                Token::Some,
                Token::Str("contact_email"),
                Token::Str("description"),
                Token::Some,
                Token::Str("description"),
                Token::Str("logo"),
                Token::Str("https://logo/"),
                Token::Str("background"),
                Token::Str("https://background/"),
                Token::Str("types"),
                Token::Seq { len: Some(1) },
                Token::Str("type"),
                Token::SeqEnd,
                Token::Str("resources"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("idPrefixes"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Str("id_prefix"),
                Token::SeqEnd,
                Token::Str("catalogs"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("addonCatalogs"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("behaviorHints"),
            ],
            ManifestBehaviorHints::default_tokens(),
            vec![
                Token::StructEnd,
                Token::Struct {
                    name: "Manifest",
                    len: 5,
                },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("version"),
                Token::Str("0.0.1"),
                Token::Str("name"),
                Token::Str("name"),
                Token::Str("types"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("resources"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
            ],
        ]
        .concat(),
    );
}
