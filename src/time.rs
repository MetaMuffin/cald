use crate::event::TimeComponent;

// Unix timestamp wrapper
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub struct Time(pub u64);

pub struct TimeComponents {
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

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Time(self.0 + rhs.0)
    }
}

impl Time {
    pub fn value(&self) -> u64 {
        self.0
    }

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
        let day_month = day_n - (month * 153 + 2) / 5 + 1;
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
        let hour = seconds_since_midnight / 3600;
        let minute = seconds_since_midnight % 3600 / 60;
        let second = seconds_since_midnight % 60;

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

    pub fn epsilon() -> Self {
        return Self(1);
    }
}

impl TimeComponents {
    pub fn value_of_component(&self, s: &TimeComponent) -> u32 {
        match s {
            TimeComponent::Years(_) => self.year,
            TimeComponent::MonthYear(_) => self.month as u32,
            TimeComponent::WeekYear(_) => self.day_week as u32,
            TimeComponent::DayMonth(_) => self.day_month as u32,
            TimeComponent::DayWeek(_) => self.day_week as u32,
            TimeComponent::HourDay(_) => self.hour as u32,
            TimeComponent::MinuteHour(_) => self.minute as u32,
            TimeComponent::SecondMinute(_) => self.second as u32,
        }
    }
    pub fn max_of_component(&self, s: &TimeComponent) -> Option<u32> {
        match s {
            TimeComponent::Years(_) => None,
            TimeComponent::MonthYear(_) => Some(12),
            TimeComponent::DayMonth(_) => Some(30), // TODO this should be 30, 31, 28 or 29 accordingly
            TimeComponent::WeekYear(_) => Some(52), // TODO this is wrong too
            TimeComponent::DayWeek(_) => Some(7),
            TimeComponent::HourDay(_) => Some(24),
            TimeComponent::MinuteHour(_) => Some(60),
            TimeComponent::SecondMinute(_) => Some(60),
        }
    }
}
