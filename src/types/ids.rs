use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// Reports whether `s` can be safely embedded as a single URL path segment.
///
/// Threads resource IDs are opaque, but the API only issues IDs made of RFC
/// 3986 unreserved characters. Restricting IDs to that set prevents a
/// caller-supplied ID from adding a path, query, or fragment to an
/// authenticated request (e.g. `"123/../me"` or `"123?fields=..."`).
fn valid_resource_id(s: &str) -> bool {
    if s.is_empty() || s == "." || s == ".." {
        return false;
    }
    s.bytes()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'.' | b'_' | b'~'))
}

macro_rules! id_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            /// Returns `true` if the ID is non-empty and safe to use as a
            /// URL path segment.
            pub fn is_valid(&self) -> bool {
                valid_resource_id(&self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_owned())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = str;
            fn deref(&self) -> &str {
                &self.0
            }
        }
    };
}

id_newtype!(
    /// Unique identifier for a post.
    PostId
);

id_newtype!(
    /// Unique identifier for a user.
    UserId
);

id_newtype!(
    /// Unique identifier for a media container.
    ContainerId
);

id_newtype!(
    /// Unique identifier for a location.
    LocationId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_id_valid() {
        let id = PostId::from("12345");
        assert!(id.is_valid());
        assert_eq!(id.to_string(), "12345");
    }

    #[test]
    fn test_post_id_invalid() {
        let id = PostId::from("");
        assert!(!id.is_valid());
    }

    #[test]
    fn test_id_rejects_path_injection() {
        for bad in [
            ".",
            "..",
            "123/../me",
            "123/replies",
            "123?fields=id",
            "123#frag",
            "123%2F",
            "12 3",
            "123\n",
            "ü123",
        ] {
            assert!(!PostId::from(bad).is_valid(), "{bad:?} must be rejected");
            assert!(!UserId::from(bad).is_valid(), "{bad:?} must be rejected");
            assert!(
                !ContainerId::from(bad).is_valid(),
                "{bad:?} must be rejected"
            );
            assert!(
                !LocationId::from(bad).is_valid(),
                "{bad:?} must be rejected"
            );
        }
    }

    #[test]
    fn test_id_accepts_unreserved_chars() {
        for good in ["17841400000000000", "abc-DEF_1.2~3", "me"] {
            assert!(PostId::from(good).is_valid(), "{good:?} must be accepted");
        }
    }

    #[test]
    fn test_user_id_from_string() {
        let id = UserId::from(String::from("user-1"));
        assert_eq!(id.as_ref(), "user-1");
        assert_eq!(&*id, "user-1");
    }

    #[test]
    fn test_id_serde_roundtrip() {
        let id = ContainerId::from("c-99");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, r#""c-99""#);
        let back: ContainerId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn test_location_id_deref() {
        let id = LocationId::from("loc-42");
        // Deref to str allows str methods directly
        assert!(id.starts_with("loc-"));
    }
}
