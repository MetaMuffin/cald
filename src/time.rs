use derive_more::{Add, Sub};

// Unix timestamp wrapper
#[derive(Add, Sub)]
struct Time(u64);

struct TimeComponents {
    year: u32,
    month: u8,
    week: u8,
    day_week: u8,
    day_month: u8,
    day_year: u16,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    // Thanks to the german wikipedia article on unix time.
    pub fn from_components(
        jahr: u32,
        monat: u32,
        tag: u32,
        stunde: u32,
        minute: u32,
        sekunde: u32,
    ) -> u64 {
        // Anzahl der Tage seit Jahresanfang ohne Tage des aktuellen Monats und ohne Schalttag
        let tage_seit_jahresanfang = &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        // Anzahl der Schaltjahre seit 1970 (ohne das evtl. laufende Schaltjahr)
        let schaltjahre =
            ((jahr - 1) - 1968) / 4 - ((jahr - 1) - 1900) / 100 + ((jahr - 1) - 1600) / 400;

        let mut tage_seit_1970 =
            (jahr - 1970) * 365 + schaltjahre + tage_seit_jahresanfang[(monat - 1) as usize] + tag
                - 1;

        // +Schalttag, wenn jahr Schaltjahr ist
        if (monat > 2) && (jahr % 4 == 0 && (jahr % 100 != 0 || jahr % 400 == 0)) {
            tage_seit_1970 += 1;
        }
        return sekunde as u64
            + 60 * (minute as u64 + 60 * (stunde as u64 + 24 * tage_seit_1970 as u64));
    }
    
    // Thanks to the german wikipedia article on unix time.
    pub fn get_components(&self) -> TimeComponents {
        let seconds_per_day = 86400; /*  24* 60 * 60 */
        let days_per_normal_year = 365; /* kein Schaltjahr */
        let days_of_four_years = 1461; /*   4*365 +   1 */
        let days_of_100_years = 36524; /* 100*365 +  25 - 1 */
        let days_of_400_years = 146097; /* 400*365 + 100 - 4 + 1 */
        let days_since_1970_01_01 = 719468; /* Tagnummer bezogen auf den 1. Maerz des Jahres "Null" */
        let mut day_n = days_since_1970_01_01 + self.0 as u32 / seconds_per_day;
        let seconds_since_midnight = self.0 as u32 % seconds_per_day;
        let mut temp;
        let mut year: u32;
        let mut month;
        let mut day_month;
        let mut minute;
        let mut hour;
        let mut second;

        // TODO
        let day_week = 0;
        let day_year = 0;
        let week = 0;

        /* Schaltjahrregel des Gregorianischen Kalenders:
        Jedes durch 100 teilbare Jahr ist kein Schaltjahr, es sei denn, es ist durch 400 teilbar. */
        temp = 4 * (day_n + days_of_100_years + 1) / days_of_400_years - 1;
        year = 100 * temp;
        day_n -= days_of_100_years * temp + temp / 4;
        /* Schaltjahrregel des Julianischen Kalenders:
        Jedes durch 4 teilbare Jahr ist ein Schaltjahr. */
        temp = 4 * (day_n + days_per_normal_year + 1) / days_of_four_years - 1;
        year += temp;
        day_n -= days_per_normal_year * temp + temp / 4;
        /* TagN enthaelt jetzt nur noch die Tage des errechneten Jahres bezogen auf den 1. Maerz. */
        month = (5 * day_n + 2) / 153;
        day_month = day_n - (month * 153 + 2) / 5 + 1;
        /*  153 = 31+30+31+30+31 Tage fuer die 5 Monate von Maerz bis Juli
        153 = 31+30+31+30+31 Tage fuer die 5 Monate von August bis Dezember
        31+28          Tage fuer Januar und Februar (siehe unten)
        +2: Justierung der Rundung
        +1: Der erste Tag im Monat ist 1 (und nicht 0).
        */
        month += 3; /* vom Jahr, das am 1. Maerz beginnt auf unser normales Jahr umrechnen: */
        if month > 12 {
            /* Monate 13 und 14 entsprechen 1 (Januar) und 2 (Februar) des naechsten Jahres */
            month -= 12;
            year += 1;
        }
        hour = seconds_since_midnight / 3600;
        minute = seconds_since_midnight % 3600 / 60;
        second = seconds_since_midnight % 60;

        TimeComponents {
            hour: hour as u8,
            minute: minute as u8,
            day_month: day_month as u8,
            day_week: day_week as u8,
            day_year: day_year as u16,
            month: month as u8,
            second: second as u8,
            week: week as u8,
            year: year as u32,
        }
    }
}
