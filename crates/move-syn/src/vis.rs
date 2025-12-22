use crate::Item;

/// Visibility options of a Move [`Item`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Visibility {
    /// `public`
    Public,
    /// `public(package)`
    Package,
    /// No visibility modifier
    Private,
}

impl Visibility {
    pub const fn is_public(&self) -> bool {
        matches!(self, Self::Public)
    }

    pub const fn is_package(&self) -> bool {
        matches!(self, Self::Package)
    }

    pub const fn is_private(&self) -> bool {
        matches!(self, Self::Private)
    }
}

impl Item {
    pub const fn visibility(&self) -> Visibility {
        let Some(vis) = &self.vis else {
            return Visibility::Private;
        };
        if vis.modifier.is_some() {
            Visibility::Package
        } else {
            Visibility::Public
        }
    }
}
