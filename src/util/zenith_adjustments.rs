//! Basic calculations used for sun-time algorithms (though currently only
//! [the NOAA algorithm](super::noaa_calculator) is available).

use jiff::civil::Date;

/// 90&deg; below the vertical. Used as a basis for most calculations since the
/// location of the sun is 90&deg; below the vertical at sunrise and sunset.
const GEOMETRIC_ZENITH: f64 = 90.0;

/// 34 arcminutes of refraction
const REFRACTION: f64 = 34.0 / 60.0;

/// The commonly used average earth radius in KM (the IUGG mean radius
/// `R1 = (2a + b) / 3` of the WGS-84 ellipsoid)
const EARTH_RADIUS: f64 = 6_371.008_8;

/// The number of days in each month before it, for a common (non-leap) year.
/// Used to map a month and day to a day of the year.
const DAYS_BEFORE_MONTH: [i16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// The Sun's apparent angular semi-diameter (the "solar radius") in degrees for
/// each day of the year, precomputed from the
/// [VSOP87](https://en.wikipedia.org/wiki/VSOP_(planets)) Earth-Sun distance
/// for reference year 2050.
///
/// The Sun's apparent size varies through the year as the Earth-Sun distance
/// changes between the perihelion (~Jan 3, about 16.27&prime; or 0.2710&deg;)
/// and the aphelion (~Jul 5, about 15.73&prime; or 0.2622&deg;), with a mean of
/// the conventional 16&prime;. The table has 365 entries indexed by day-of-year
/// (1-365); 2050 is a common year, so February 29 reuses the February 28 value
/// (see [`apparent_solar_radius`]).
const SOLAR_RADIUS_BY_DAY_OF_YEAR: [f64; 365] = [
    0.27108024, 0.27108486, 0.27108790, 0.27108930, 0.27108899, 0.27108695, 0.27108316, 0.27107762,
    0.27107033, 0.27106133, 0.27105062, 0.27103826, 0.27102427, 0.27100873, 0.27099168, 0.27097320,
    0.27095337, 0.27093228, 0.27091002, 0.27088667, 0.27086231, 0.27083701, 0.27081079, 0.27078369,
    0.27075569, 0.27072676, 0.27069684, 0.27066588, 0.27063378, 0.27060048, 0.27056589, 0.27052995,
    0.27049261, 0.27045383, 0.27041359, 0.27037186, 0.27032864, 0.27028396, 0.27023782, 0.27019025,
    0.27014129, 0.27009098, 0.27003938, 0.26998658, 0.26993264, 0.26987767, 0.26982177, 0.26976506,
    0.26970763, 0.26964958, 0.26959099, 0.26953191, 0.26947239, 0.26941242, 0.26935200, 0.26929108,
    0.26922962, 0.26916755, 0.26910482, 0.26904136, 0.26897712, 0.26891206, 0.26884614, 0.26877935,
    0.26871165, 0.26864306, 0.26857358, 0.26850321, 0.26843197, 0.26835989, 0.26828703, 0.26821343,
    0.26813918, 0.26806437, 0.26798910, 0.26791348, 0.26783763, 0.26776167, 0.26768569, 0.26760979,
    0.26753404, 0.26745846, 0.26738308, 0.26730790, 0.26723289, 0.26715800, 0.26708320, 0.26700842,
    0.26693363, 0.26685877, 0.26678380, 0.26670870, 0.26663342, 0.26655796, 0.26648229, 0.26640640,
    0.26633030, 0.26625399, 0.26617748, 0.26610082, 0.26602406, 0.26594728, 0.26587055, 0.26579398,
    0.26571769, 0.26564180, 0.26556641, 0.26549164, 0.26541756, 0.26534425, 0.26527174, 0.26520006,
    0.26512920, 0.26505915, 0.26498987, 0.26492132, 0.26485345, 0.26478622, 0.26471959, 0.26465351,
    0.26458794, 0.26452285, 0.26445820, 0.26439396, 0.26433010, 0.26426661, 0.26420348, 0.26414070,
    0.26407832, 0.26401636, 0.26395489, 0.26389400, 0.26383378, 0.26377435, 0.26371580, 0.26365825,
    0.26360179, 0.26354651, 0.26349247, 0.26343971, 0.26338825, 0.26333807, 0.26328918, 0.26324153,
    0.26319510, 0.26314983, 0.26310568, 0.26306261, 0.26302057, 0.26297951, 0.26293938, 0.26290014,
    0.26286173, 0.26282411, 0.26278725, 0.26275111, 0.26271570, 0.26268102, 0.26264710, 0.26261399,
    0.26258177, 0.26255053, 0.26252037, 0.26249137, 0.26246366, 0.26243731, 0.26241239, 0.26238897,
    0.26236707, 0.26234671, 0.26232790, 0.26231061, 0.26229481, 0.26228048, 0.26226756, 0.26225602,
    0.26224581, 0.26223687, 0.26222914, 0.26222257, 0.26221708, 0.26221262, 0.26220912, 0.26220653,
    0.26220480, 0.26220392, 0.26220388, 0.26220470, 0.26220642, 0.26220910, 0.26221282, 0.26221768,
    0.26222375, 0.26223114, 0.26223991, 0.26225014, 0.26226187, 0.26227512, 0.26228992, 0.26230626,
    0.26232413, 0.26234349, 0.26236433, 0.26238659, 0.26241024, 0.26243521, 0.26246145, 0.26248890,
    0.26251746, 0.26254708, 0.26257766, 0.26260913, 0.26264141, 0.26267446, 0.26270823, 0.26274270,
    0.26277789, 0.26281382, 0.26285054, 0.26288813, 0.26292666, 0.26296622, 0.26300686, 0.26304868,
    0.26309171, 0.26313601, 0.26318159, 0.26322848, 0.26327666, 0.26332612, 0.26337685, 0.26342881,
    0.26348197, 0.26353627, 0.26359167, 0.26364809, 0.26370545, 0.26376368, 0.26382267, 0.26388233,
    0.26394256, 0.26400328, 0.26406442, 0.26412593, 0.26418777, 0.26424995, 0.26431249, 0.26437542,
    0.26443880, 0.26450270, 0.26456718, 0.26463231, 0.26469815, 0.26476474, 0.26483213, 0.26490034,
    0.26496937, 0.26503922, 0.26510990, 0.26518138, 0.26525364, 0.26532664, 0.26540034, 0.26547467,
    0.26554957, 0.26562495, 0.26570072, 0.26577676, 0.26585296, 0.26592922, 0.26600544, 0.26608154,
    0.26615745, 0.26623314, 0.26630859, 0.26638382, 0.26645884, 0.26653372, 0.26660850, 0.26668323,
    0.26675798, 0.26683280, 0.26690773, 0.26698280, 0.26705803, 0.26713345, 0.26720905, 0.26728485,
    0.26736083, 0.26743698, 0.26751326, 0.26758964, 0.26766606, 0.26774245, 0.26781872, 0.26789476,
    0.26797045, 0.26804569, 0.26812035, 0.26819433, 0.26826755, 0.26833992, 0.26841141, 0.26848200,
    0.26855170, 0.26862051, 0.26868849, 0.26875567, 0.26882211, 0.26888786, 0.26895296, 0.26901746,
    0.26908139, 0.26914478, 0.26920767, 0.26927006, 0.26933199, 0.26939345, 0.26945445, 0.26951496,
    0.26957497, 0.26963442, 0.26969325, 0.26975136, 0.26980865, 0.26986502, 0.26992034, 0.26997450,
    0.27002740, 0.27007895, 0.27012907, 0.27017773, 0.27022491, 0.27027060, 0.27031483, 0.27035764,
    0.27039906, 0.27043914, 0.27047794, 0.27051549, 0.27055186, 0.27058709, 0.27062122, 0.27065430,
    0.27068636, 0.27071746, 0.27074761, 0.27077684, 0.27080516, 0.27083256, 0.27085899, 0.27088442,
    0.27090875, 0.27093191, 0.27095379, 0.27097427, 0.27099326, 0.27101067, 0.27102640, 0.27104041,
    0.27105266, 0.27106312, 0.27107182, 0.27107876, 0.27108399,
];

/// Function to return the adjustment to the zenith required to account for the
/// elevation.
///
/// Since a person at a higher elevation can see farther below the horizon, the
/// calculation for sunrise / sunset is calculated below the horizon used at sea
/// level. The algorithm used is:
/// ```text
/// (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1_000.0))).acos().to_degrees()
/// ```
///
/// The source of this algorithm is *Calendrical Calculations* by Edward M.
/// Reingold and Nachum Dershowitz. An alternate algorithm that produces similar
/// (but not completely accurate) results found in *Ma'aglay Tzedek* by Moishe
/// Kosower and other sources is:
/// ```text
/// 0.0347 * elevation.sqrt();
/// ```
#[must_use]
pub fn elevation_adjustment(elevation: f64) -> f64 {
    (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1_000.0)))
        .acos()
        .to_degrees()
}

/// Returns the Sun's apparent angular semi-diameter (the "solar radius") for
/// the given date, in degrees.
///
/// The Sun's apparent size changes over the year as the Earth-Sun distance
/// varies between the perihelion (~Jan 3, when the Sun is largest) and the
/// aphelion (~Jul 5, when it is smallest). The value is read from a precomputed
/// table (`SOLAR_RADIUS_BY_DAY_OF_YEAR`) keyed by calendar day; the date's
/// month and day are mapped onto the common (non-leap) reference year 2050, so
/// a February 29 input is resolved to February 28 and assigned that day's
/// value.
#[must_use]
pub fn apparent_solar_radius(date: Date) -> f64 {
    let month = date.month();
    // Map onto the common reference year 2050, resolving February 29 to
    // February 28.
    let day = if month == 2 && date.day() == 29 {
        28
    } else {
        date.day()
    };
    let day_of_year = DAYS_BEFORE_MONTH[(month - 1) as usize] + i16::from(day);
    SOLAR_RADIUS_BY_DAY_OF_YEAR[(day_of_year - 1) as usize]
}

/// Adjusts the zenith of astronomical sunrise and sunset to account for solar
/// refraction, solar radius, and elevation.
///
/// The value for the Sun's zenith and true rise/set zenith is the angle that
/// the center of the Sun makes to a line perpendicular to the Earth's surface.
/// If the Sun were a point and the Earth were without an atmosphere, true
/// sunset and sunrise would correspond to a 90&deg; zenith. Because the Sun is
/// not a point, and because the atmosphere refracts light, this 90&deg; zenith
/// does not, in fact, correspond to true sunset or sunrise, instead the center
/// of the Sun's disk must lie just below the horizon for the upper edge to be
/// obscured. This means that a zenith of just above 90&deg; must be used. The
/// Sun subtends an angle of about 16 minutes of arc (the exact
/// [apparent solar radius](apparent_solar_radius) varies through the year) and
/// atmospheric refraction accounts for 34 minutes or so, giving a total of
/// about 50 arcminutes. The total value is therefore roughly 90+(5/6) or
/// 90.8333333&deg; for true sunrise/sunset. Since a person at an elevation can
/// see below the horizon of a person at sea level, this will also adjust the
/// zenith to account for elevation if available. Note that this will only
/// adjust the value if the zenith is exactly 90 degrees. For values below and
/// above this no correction is done, as calculations above or below 90&deg; are
/// usually calculated as an offset to 90&deg;.
#[must_use]
pub fn adjusted_zenith(zenith: f64, elevation: f64, date: Date) -> f64 {
    if zenith != GEOMETRIC_ZENITH {
        return zenith;
    }
    zenith + apparent_solar_radius(date) + REFRACTION + elevation_adjustment(elevation)
}
