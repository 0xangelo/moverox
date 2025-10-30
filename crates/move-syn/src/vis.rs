use crate::Item;

/// Visibility options of a Move [`Item`].
pub enum Visibility {
    /// `public`
    Public,
    /// `public(package)`
    Package,
    /// No visibility modifier
    Private,
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
