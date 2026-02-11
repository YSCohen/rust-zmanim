#[macro_export]
macro_rules! zmanim_for_offset {
    // Main entry point with method selection
    ($name:ident, $offset_fn:expr, [$($method:ident => $doc:expr),* $(,)?]) => {
        $(
            zmanim_for_offset!(@method_custom $name, $offset_fn, $method, $doc);
        )*
    };

    // Generate alos method
    (@method_custom $name:ident, $offset_fn:expr, alos, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<alos $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.alos(&offset)
            }
        }
    };

    // Generate tzais method
    (@method_custom $name:ident, $offset_fn:expr, tzais, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<tzais $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.tzais(&offset)
            }
        }
    };

    // Generate shaah_zmanis method
    (@method_custom $name:ident, $offset_fn:expr, shaah_zmanis, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<shaah_zmanis $name>](&self) -> Option<TimeDelta> {
                let offset = ($offset_fn)(self)?;
                self.shaah_zmanis(&offset)
            }
        }
    };

    // Generate sof_zman_shema_mga method
    (@method_custom $name:ident, $offset_fn:expr, sof_zman_shema_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<sof_zman_shema_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.sof_zman_shema_mga(&offset)
            }
        }
    };

    // Generate sof_zman_tefila_mga method
    (@method_custom $name:ident, $offset_fn:expr, sof_zman_tefila_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<sof_zman_tefila_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.sof_zman_tefila_mga(&offset)
            }
        }
    };

    // Generate sof_zman_biur_chametz_mga method
    (@method_custom $name:ident, $offset_fn:expr, sof_zman_biur_chametz_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<sof_zman_biur_chametz_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.sof_zman_biur_chametz_mga(&offset)
            }
        }
    };

    // Generate mincha_gedola_mga method
    (@method_custom $name:ident, $offset_fn:expr, mincha_gedola_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<mincha_gedola_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.mincha_gedola_mga(&offset)
            }
        }
    };

    // Generate samuch_lemincha_ketana_mga method
    (@method_custom $name:ident, $offset_fn:expr, samuch_lemincha_ketana_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<samuch_lemincha_ketana_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.samuch_lemincha_ketana_mga(&offset)
            }
        }
    };

    // Generate mincha_ketana_mga method
    (@method_custom $name:ident, $offset_fn:expr, mincha_ketana_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<mincha_ketana_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.mincha_ketana_mga(&offset)
            }
        }
    };

    // Generate plag_mga method
    (@method_custom $name:ident, $offset_fn:expr, plag_mga, $doc:expr) => {
        paste! {
            #[doc = $doc]
            pub fn [<plag_mga $name>](&self) -> Option<DateTime<Tz>> {
                let offset = ($offset_fn)(self)?;
                self.plag_mga(&offset)
            }
        }
    };
}
