//! Example of using [`ComplexZmanimCalendar`] to calculate *zmanim*. Calculates
//! *zmanim* at the *kosel* according to the *shita* of the *Baal HaTanya*

use jiff::{Zoned, tz::TimeZone};
use rust_zmanim::prelude::*;

fn main() {
    let jerusalem = TimeZone::get("Asia/Jerusalem").unwrap();
    let today = Zoned::now().date();

    // your location here
    let kosel = GeoLocation {
        latitude: 31.777,
        longitude: 35.234,
        elevation: 700.0,
        timezone: jerusalem,
    };

    let czc = ComplexZmanimCalendar {
        geo_location: kosel,
        date: today,
        use_elevation: UseElevation::No,
    };

    let alos = czc.alos_baal_hatanya().unwrap();
    let hanetz = czc.hanetz().unwrap();
    let szks = czc.sof_zman_shema_baal_hatanya().unwrap();
    let szt = czc.sof_zman_tefila_baal_hatanya().unwrap();
    let chatzos = czc.chatzos().unwrap();
    let mg = czc.mincha_gedola_baal_hatanya().unwrap();
    let mk = czc.mincha_ketana_baal_hatanya().unwrap();
    let plag = czc.plag_baal_hatanya().unwrap();
    let shkia = czc.shkia().unwrap();
    let tzeis = czc.tzeis_baal_hatanya().unwrap();
    let shaah = czc.shaah_zmanis_baal_hatanya().unwrap();

    println!(
        "alos:         {alos}
hanetz:       {hanetz}
SZKS:         {szks}
SZT:          {szt}
chatzos:      {chatzos}
MG:           {mg}
MK:           {mk}
shkia:        {shkia}
plag:         {plag}
tzeis:        {tzeis}
shaah zmanis: {shaah}"
    )
}
