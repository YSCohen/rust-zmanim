//! Example of using [`zmanim_calculator`] to create custom *zmanim*. Creates MGA
//! *zmanim* from *alos* 10&deg; to *tzais* 13&deg;, with elevation-adjusted
//! *shkia* and *hanetz*. THIS IS NOT A REAL *SHITA*, just an example

use chrono::Utc;
use chrono_tz::Asia::Jerusalem;
use rust_zmanim::prelude::*;

fn main() {
    let today = Utc::now().with_timezone(&Jerusalem);

    let kosel = GeoLocation {
        latitude: 31.777,
        longitude: 35.234,
        elevation: 700.0,
        timezone: Jerusalem,
    };

    let alos_10_deg =
        zmanim_calculator::alos(&today, &kosel, false, &ZmanOffset::Degrees(10.0)).unwrap();
    let tzais_13_deg =
        zmanim_calculator::tzais(&today, &kosel, false, &ZmanOffset::Degrees(13.0)).unwrap();
    let hanetz = zmanim_calculator::hanetz(&today, &kosel, true).unwrap();
    let szks = zmanim_calculator::sof_zman_shema(&alos_10_deg, &tzais_13_deg);
    let szt = zmanim_calculator::sof_zman_tefila(&alos_10_deg, &tzais_13_deg);
    let chatzos = zmanim_calculator::chatzos(&today, &kosel).unwrap();
    let mg = zmanim_calculator::mincha_gedola(&alos_10_deg, &tzais_13_deg);
    let mk = zmanim_calculator::mincha_ketana(&alos_10_deg, &tzais_13_deg);
    let plag = zmanim_calculator::plag_hamincha(&alos_10_deg, &tzais_13_deg);
    let shkia = zmanim_calculator::shkia(&today, &kosel, true).unwrap();
    let shaah = zmanim_calculator::shaah_zmanis(&alos_10_deg, &tzais_13_deg);

    // shaah is a TimeDelta, so we need to pretty-print it manually
    let sz_hours = shaah.num_hours();
    let sz_minutes = shaah.num_minutes() % 60;
    let sz_seconds = shaah.num_seconds() % 60;
    let sz_nanos = shaah.subsec_nanos();

    println!(
        "alos:         {alos_10_deg}
hanetz:       {hanetz}
SZKS:         {szks}
SZT:          {szt}
chatzos:      {chatzos}
MG:           {mg}
MK:           {mk}
shkia:        {shkia}
plag:         {plag}
tzais:        {tzais_13_deg}
shaah zmanis: {sz_hours:02}:{sz_minutes:02}:{sz_seconds:02}.{sz_nanos:02}"
    )
}
