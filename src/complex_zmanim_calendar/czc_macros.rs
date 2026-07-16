/// Generates *zman* accessor methods that each compute a
/// [`ZmanOffset`](crate::zmanim_calculator::ZmanOffset) via a
/// closure and forward it to the matching calculation method.
///
/// The main entry point takes an offset-producing expression and a list of
/// `method_name => zman_type, doc` triples. For each triple it emits a `pub fn`
/// that evaluates the offset closure and calls `self.<zman_type>(&offset)`. The
/// `shaah_zmanis_mga` type is handled specially because it returns
/// `Option<SignedDuration>` rather than `Option<Zoned>`.
macro_rules! zmanim_for_offset {
    // Main entry point
    ($offset_fn:expr, [$($fn_name:ident => $zman_type:ident, $doc:expr),* $(,)?]) => {
        $(
            zmanim_for_offset!(@method_custom $fn_name, $offset_fn, $zman_type, $doc);
        )*
    };

    // shaah_zmanis_mga - separate variant due to different return type
    (@method_custom $fn_name:ident, $offset_fn:expr, shaah_zmanis_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<SignedDuration> {
            let offset = ($offset_fn)(self)?;
            self.shaah_zmanis_mga(&offset)
        }
    };

    // Generic variant for other zman types returning Zoned
    (@method_custom $fn_name:ident, $offset_fn:expr, $zman_type:ident, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<Zoned> {
            let offset = ($offset_fn)(self)?;
            self.$zman_type(&offset)
        }
    };
}
