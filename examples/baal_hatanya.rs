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
    let mg = czc
        .mincha_gedola_baal_hatanya_greater_than_30_minutes()
        .unwrap();
    let mk = czc.mincha_ketana_baal_hatanya().unwrap();
    let plag = czc.plag_baal_hatanya().unwrap();
    let shkia = czc.shkia().unwrap();
    let tzais = czc.tzais_baal_hatanya().unwrap();

    println!(
        "alos:    {alos}
hanetz:  {hanetz}
SZKS:    {szks}
SZT:     {szt}
chatzos: {chatzos}
MG:      {mg}
MK:      {mk}
shkia:   {shkia}
plag:    {plag}
tzais:   {tzais}"
    )
}
