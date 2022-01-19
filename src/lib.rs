use deunicode::deunicode;

pub trait EqUnicodeInsensitive<T: ?Sized> {
    fn eq_unicode_insensitive(&self, rhs: &T) -> bool;
}

impl<T> EqUnicodeInsensitive<T> for str
where
    T: ?Sized + AsRef<str>,
{
    /// Compares two strings ignoring letter case and permitting ascii equivalents
    /// for unicode characters
    ///
    /// ```
    /// use poematic::EqUnicodeInsensitive;
    /// assert!("Zęby".eq_unicode_insensitive("żeby"));
    /// ```
    fn eq_unicode_insensitive(&self, rhs: &T) -> bool {
        deunicode(self.to_lowercase().as_ref())
            == deunicode(rhs.as_ref().to_lowercase().as_ref())
    }
}
