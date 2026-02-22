#[macro_export]
macro_rules! zmanim_for_offset {
    // Main entry point
    ($offset_fn:expr, [$($fn_name:ident => $zman_type:ident, $doc:expr),* $(,)?]) => {
        $(
            zmanim_for_offset!(@method_custom $fn_name, $offset_fn, $zman_type, $doc);
        )*
    };

    // shaah_zmanis_mga - separate variant due to different return type (TimeDelta)
    (@method_custom $fn_name:ident, $offset_fn:expr, shaah_zmanis_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<TimeDelta> {
            let offset = ($offset_fn)(self)?;
            self.shaah_zmanis_mga(&offset)
        }
    };

    // Generic variant for other zman types returning DateTime<Tz>
    (@method_custom $fn_name:ident, $offset_fn:expr, $zman_type:ident, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.$zman_type(&offset)
        }
    };
}
