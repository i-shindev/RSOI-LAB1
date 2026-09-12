#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldUpdate<T> {
    Keep,
    Clear,
    Set(T),
}

impl<T> Default for FieldUpdate<T> {
    fn default() -> Self {
        FieldUpdate::Keep
    }
}

impl<T> FieldUpdate<T> {
    pub fn apply(self, field: &mut Option<T>) {
        match self {
            FieldUpdate::Keep => {}
            FieldUpdate::Clear => *field = None,
            FieldUpdate::Set(value) => *field = Some(value),
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            FieldUpdate::Set(value) => Some(value),
            FieldUpdate::Keep | FieldUpdate::Clear => None,
        }
    }
}
