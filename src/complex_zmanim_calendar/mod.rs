//! The [`ComplexZmanimCalendar`] struct is stateful and has many premade
//! *zmanim* calculations, both conveniences built on the
//! [`zmanim_calculator`](crate::zmanim_calculator) API.

#[macro_use]
mod czc_macros;

#[macro_use]
mod doc_macros;

mod czc_struct;

pub use czc_struct::*;
