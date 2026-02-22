#[macro_export]
macro_rules! zmanim_for_offset {
    // Main entry point with explicit function names
    ($offset_fn:expr, [$($fn_name:ident => $zman_type:ident, $doc:expr),* $(,)?]) => {
        $(
            zmanim_for_offset!(@method_custom $fn_name, $offset_fn, $zman_type, $doc);
        )*
    };

    // alos
    (@method_custom $fn_name:ident, $offset_fn:expr, alos, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.alos(&offset)
        }
    };

    // tzeis
    (@method_custom $fn_name:ident, $offset_fn:expr, tzeis, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.tzeis(&offset)
        }
    };

    // shaah_zmanis_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, shaah_zmanis_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<TimeDelta> {
            let offset = ($offset_fn)(self)?;
            self.shaah_zmanis_mga(&offset)
        }
    };

    // sof_zman_shema_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, sof_zman_shema_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.sof_zman_shema_mga(&offset)
        }
    };

    // sof_zman_tefila_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, sof_zman_tefila_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.sof_zman_tefila_mga(&offset)
        }
    };

    // sof_zman_biur_chametz_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, sof_zman_biur_chametz_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.sof_zman_biur_chametz_mga(&offset)
        }
    };

    // mincha_gedola_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, mincha_gedola_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.mincha_gedola_mga(&offset)
        }
    };

    // samuch_lemincha_ketana_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, samuch_lemincha_ketana_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.samuch_lemincha_ketana_mga(&offset)
        }
    };

    // mincha_ketana_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, mincha_ketana_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.mincha_ketana_mga(&offset)
        }
    };

    // plag_mga
    (@method_custom $fn_name:ident, $offset_fn:expr, plag_mga, $doc:expr) => {
        #[must_use]
        #[doc = $doc]
        pub fn $fn_name(&self) -> Option<DateTime<Tz>> {
            let offset = ($offset_fn)(self)?;
            self.plag_mga(&offset)
        }
    };
}
