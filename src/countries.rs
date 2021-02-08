//! Generated using https://github.com/edg-l/payhelper

use crate::errors::InvalidCountryError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// IS0-3166-1 country codes
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Country {
    /// ALBANIA
    AL,
    /// ALGERIA
    DZ,
    /// ANDORRA
    AD,
    /// ANGOLA
    AO,
    /// ANGUILLA
    AI,
    /// ANTIGUA & BARBUDA
    AG,
    /// ARGENTINA
    AR,
    /// ARMENIA
    AM,
    /// ARUBA
    AW,
    /// AUSTRALIA
    AU,
    /// AUSTRIA
    AT,
    /// AZERBAIJAN
    AZ,
    /// BAHAMAS
    BS,
    /// BAHRAIN
    BH,
    /// BARBADOS
    BB,
    /// BELARUS
    BY,
    /// BELGIUM
    BE,
    /// BELIZE
    BZ,
    /// BENIN
    BJ,
    /// BERMUDA
    BM,
    /// BHUTAN
    BT,
    /// BOLIVIA
    BO,
    /// BOSNIA & HERZEGOVINA
    BA,
    /// BOTSWANA
    BW,
    /// BRAZIL
    BR,
    /// BRITISH VIRGIN ISLANDS
    VG,
    /// BRUNEI
    BN,
    /// BULGARIA
    BG,
    /// BURKINA FASO
    BF,
    /// BURUNDI
    BI,
    /// CAMBODIA
    KH,
    /// CAMEROON
    CM,
    /// CANADA
    CA,
    /// CAPE VERDE
    CV,
    /// CAYMAN ISLANDS
    KY,
    /// CHAD
    TD,
    /// CHILE
    CL,
    /// CHINA
    C2,
    /// COLOMBIA
    CO,
    /// COMOROS
    KM,
    /// CONGO - BRAZZAVILLE
    CG,
    /// CONGO - KINSHASA
    CD,
    /// COOK ISLANDS
    CK,
    /// COSTA RICA
    CR,
    /// CÔTE D’IVOIRE
    CI,
    /// CROATIA
    HR,
    /// CYPRUS
    CY,
    /// CZECH REPUBLIC
    CZ,
    /// DENMARK
    DK,
    /// DJIBOUTI
    DJ,
    /// DOMINICA
    DM,
    /// DOMINICAN REPUBLIC
    DO,
    /// ECUADOR
    EC,
    /// EGYPT
    EG,
    /// EL SALVADOR
    SV,
    /// ERITREA
    ER,
    /// ESTONIA
    EE,
    /// ETHIOPIA
    ET,
    /// FALKLAND ISLANDS
    FK,
    /// FAROE ISLANDS
    FO,
    /// FIJI
    FJ,
    /// FINLAND
    FI,
    /// FRANCE
    FR,
    /// FRENCH GUIANA
    GF,
    /// FRENCH POLYNESIA
    PF,
    /// GABON
    GA,
    /// GAMBIA
    GM,
    /// GEORGIA
    GE,
    /// GERMANY
    DE,
    /// GIBRALTAR
    GI,
    /// GREECE
    GR,
    /// GREENLAND
    GL,
    /// GRENADA
    GD,
    /// GUADELOUPE
    GP,
    /// GUATEMALA
    GT,
    /// GUINEA
    GN,
    /// GUINEA-BISSAU
    GW,
    /// GUYANA
    GY,
    /// HONDURAS
    HN,
    /// HONG KONG SAR CHINA
    HK,
    /// HUNGARY
    HU,
    /// ICELAND
    IS,
    /// INDIA
    IN,
    /// INDONESIA
    ID,
    /// IRELAND
    IE,
    /// ISRAEL
    IL,
    /// ITALY
    IT,
    /// JAMAICA
    JM,
    /// JAPAN
    JP,
    /// JORDAN
    JO,
    /// KAZAKHSTAN
    KZ,
    /// KENYA
    KE,
    /// KIRIBATI
    KI,
    /// KUWAIT
    KW,
    /// KYRGYZSTAN
    KG,
    /// LAOS
    LA,
    /// LATVIA
    LV,
    /// LESOTHO
    LS,
    /// LIECHTENSTEIN
    LI,
    /// LITHUANIA
    LT,
    /// LUXEMBOURG
    LU,
    /// MACEDONIA
    MK,
    /// MADAGASCAR
    MG,
    /// MALAWI
    MW,
    /// MALAYSIA
    MY,
    /// MALDIVES
    MV,
    /// MALI
    ML,
    /// MALTA
    MT,
    /// MARSHALL ISLANDS
    MH,
    /// MARTINIQUE
    MQ,
    /// MAURITANIA
    MR,
    /// MAURITIUS
    MU,
    /// MAYOTTE
    YT,
    /// MEXICO
    MX,
    /// MICRONESIA
    FM,
    /// MOLDOVA
    MD,
    /// MONACO
    MC,
    /// MONGOLIA
    MN,
    /// MONTENEGRO
    ME,
    /// MONTSERRAT
    MS,
    /// MOROCCO
    MA,
    /// MOZAMBIQUE
    MZ,
    /// NAMIBIA
    NA,
    /// NAURU
    NR,
    /// NEPAL
    NP,
    /// NETHERLANDS
    NL,
    /// NEW CALEDONIA
    NC,
    /// NEW ZEALAND
    NZ,
    /// NICARAGUA
    NI,
    /// NIGER
    NE,
    /// NIGERIA
    NG,
    /// NIUE
    NU,
    /// NORFOLK ISLAND
    NF,
    /// NORWAY
    NO,
    /// OMAN
    OM,
    /// PALAU
    PW,
    /// PANAMA
    PA,
    /// PAPUA NEW GUINEA
    PG,
    /// PARAGUAY
    PY,
    /// PERU
    PE,
    /// PHILIPPINES
    PH,
    /// PITCAIRN ISLANDS
    PN,
    /// POLAND
    PL,
    /// PORTUGAL
    PT,
    /// QATAR
    QA,
    /// RÉUNION
    RE,
    /// ROMANIA
    RO,
    /// RUSSIA
    RU,
    /// RWANDA
    RW,
    /// SAMOA
    WS,
    /// SAN MARINO
    SM,
    /// SÃO TOMÉ & PRÍNCIPE
    ST,
    /// SAUDI ARABIA
    SA,
    /// SENEGAL
    SN,
    /// SERBIA
    RS,
    /// SEYCHELLES
    SC,
    /// SIERRA LEONE
    SL,
    /// SINGAPORE
    SG,
    /// SLOVAKIA
    SK,
    /// SLOVENIA
    SI,
    /// SOLOMON ISLANDS
    SB,
    /// SOMALIA
    SO,
    /// SOUTH AFRICA
    ZA,
    /// SOUTH KOREA
    KR,
    /// SPAIN
    ES,
    /// SRI LANKA
    LK,
    /// ST. HELENA
    SH,
    /// ST. KITTS & NEVIS
    KN,
    /// ST. LUCIA
    LC,
    /// ST. PIERRE & MIQUELON
    PM,
    /// ST. VINCENT & GRENADINES
    VC,
    /// SURINAME
    SR,
    /// SVALBARD & JAN MAYEN
    SJ,
    /// SWAZILAND
    SZ,
    /// SWEDEN
    SE,
    /// SWITZERLAND
    CH,
    /// TAIWAN
    TW,
    /// TAJIKISTAN
    TJ,
    /// TANZANIA
    TZ,
    /// THAILAND
    TH,
    /// TOGO
    TG,
    /// TONGA
    TO,
    /// TRINIDAD & TOBAGO
    TT,
    /// TUNISIA
    TN,
    /// TURKMENISTAN
    TM,
    /// TURKS & CAICOS ISLANDS
    TC,
    /// TUVALU
    TV,
    /// UGANDA
    UG,
    /// UKRAINE
    UA,
    /// UNITED ARAB EMIRATES
    AE,
    /// UNITED KINGDOM
    GB,
    /// UNITED STATES
    US,
    /// URUGUAY
    UY,
    /// VANUATU
    VU,
    /// VATICAN CITY
    VA,
    /// VENEZUELA
    VE,
    /// VIETNAM
    VN,
    /// WALLIS & FUTUNA
    WF,
    /// YEMEN
    YE,
    /// ZAMBIA
    ZM,
    /// ZIMBABWE
    ZW,
}

impl Default for Country {
    fn default() -> Self {
        Self::US
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl FromStr for Country {
    type Err = InvalidCountryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AL" => Ok(Self::AL),
            "DZ" => Ok(Self::DZ),
            "AD" => Ok(Self::AD),
            "AO" => Ok(Self::AO),
            "AI" => Ok(Self::AI),
            "AG" => Ok(Self::AG),
            "AR" => Ok(Self::AR),
            "AM" => Ok(Self::AM),
            "AW" => Ok(Self::AW),
            "AU" => Ok(Self::AU),
            "AT" => Ok(Self::AT),
            "AZ" => Ok(Self::AZ),
            "BS" => Ok(Self::BS),
            "BH" => Ok(Self::BH),
            "BB" => Ok(Self::BB),
            "BY" => Ok(Self::BY),
            "BE" => Ok(Self::BE),
            "BZ" => Ok(Self::BZ),
            "BJ" => Ok(Self::BJ),
            "BM" => Ok(Self::BM),
            "BT" => Ok(Self::BT),
            "BO" => Ok(Self::BO),
            "BA" => Ok(Self::BA),
            "BW" => Ok(Self::BW),
            "BR" => Ok(Self::BR),
            "VG" => Ok(Self::VG),
            "BN" => Ok(Self::BN),
            "BG" => Ok(Self::BG),
            "BF" => Ok(Self::BF),
            "BI" => Ok(Self::BI),
            "KH" => Ok(Self::KH),
            "CM" => Ok(Self::CM),
            "CA" => Ok(Self::CA),
            "CV" => Ok(Self::CV),
            "KY" => Ok(Self::KY),
            "TD" => Ok(Self::TD),
            "CL" => Ok(Self::CL),
            "C2" => Ok(Self::C2),
            "CO" => Ok(Self::CO),
            "KM" => Ok(Self::KM),
            "CG" => Ok(Self::CG),
            "CD" => Ok(Self::CD),
            "CK" => Ok(Self::CK),
            "CR" => Ok(Self::CR),
            "CI" => Ok(Self::CI),
            "HR" => Ok(Self::HR),
            "CY" => Ok(Self::CY),
            "CZ" => Ok(Self::CZ),
            "DK" => Ok(Self::DK),
            "DJ" => Ok(Self::DJ),
            "DM" => Ok(Self::DM),
            "DO" => Ok(Self::DO),
            "EC" => Ok(Self::EC),
            "EG" => Ok(Self::EG),
            "SV" => Ok(Self::SV),
            "ER" => Ok(Self::ER),
            "EE" => Ok(Self::EE),
            "ET" => Ok(Self::ET),
            "FK" => Ok(Self::FK),
            "FO" => Ok(Self::FO),
            "FJ" => Ok(Self::FJ),
            "FI" => Ok(Self::FI),
            "FR" => Ok(Self::FR),
            "GF" => Ok(Self::GF),
            "PF" => Ok(Self::PF),
            "GA" => Ok(Self::GA),
            "GM" => Ok(Self::GM),
            "GE" => Ok(Self::GE),
            "DE" => Ok(Self::DE),
            "GI" => Ok(Self::GI),
            "GR" => Ok(Self::GR),
            "GL" => Ok(Self::GL),
            "GD" => Ok(Self::GD),
            "GP" => Ok(Self::GP),
            "GT" => Ok(Self::GT),
            "GN" => Ok(Self::GN),
            "GW" => Ok(Self::GW),
            "GY" => Ok(Self::GY),
            "HN" => Ok(Self::HN),
            "HK" => Ok(Self::HK),
            "HU" => Ok(Self::HU),
            "IS" => Ok(Self::IS),
            "IN" => Ok(Self::IN),
            "ID" => Ok(Self::ID),
            "IE" => Ok(Self::IE),
            "IL" => Ok(Self::IL),
            "IT" => Ok(Self::IT),
            "JM" => Ok(Self::JM),
            "JP" => Ok(Self::JP),
            "JO" => Ok(Self::JO),
            "KZ" => Ok(Self::KZ),
            "KE" => Ok(Self::KE),
            "KI" => Ok(Self::KI),
            "KW" => Ok(Self::KW),
            "KG" => Ok(Self::KG),
            "LA" => Ok(Self::LA),
            "LV" => Ok(Self::LV),
            "LS" => Ok(Self::LS),
            "LI" => Ok(Self::LI),
            "LT" => Ok(Self::LT),
            "LU" => Ok(Self::LU),
            "MK" => Ok(Self::MK),
            "MG" => Ok(Self::MG),
            "MW" => Ok(Self::MW),
            "MY" => Ok(Self::MY),
            "MV" => Ok(Self::MV),
            "ML" => Ok(Self::ML),
            "MT" => Ok(Self::MT),
            "MH" => Ok(Self::MH),
            "MQ" => Ok(Self::MQ),
            "MR" => Ok(Self::MR),
            "MU" => Ok(Self::MU),
            "YT" => Ok(Self::YT),
            "MX" => Ok(Self::MX),
            "FM" => Ok(Self::FM),
            "MD" => Ok(Self::MD),
            "MC" => Ok(Self::MC),
            "MN" => Ok(Self::MN),
            "ME" => Ok(Self::ME),
            "MS" => Ok(Self::MS),
            "MA" => Ok(Self::MA),
            "MZ" => Ok(Self::MZ),
            "NA" => Ok(Self::NA),
            "NR" => Ok(Self::NR),
            "NP" => Ok(Self::NP),
            "NL" => Ok(Self::NL),
            "NC" => Ok(Self::NC),
            "NZ" => Ok(Self::NZ),
            "NI" => Ok(Self::NI),
            "NE" => Ok(Self::NE),
            "NG" => Ok(Self::NG),
            "NU" => Ok(Self::NU),
            "NF" => Ok(Self::NF),
            "NO" => Ok(Self::NO),
            "OM" => Ok(Self::OM),
            "PW" => Ok(Self::PW),
            "PA" => Ok(Self::PA),
            "PG" => Ok(Self::PG),
            "PY" => Ok(Self::PY),
            "PE" => Ok(Self::PE),
            "PH" => Ok(Self::PH),
            "PN" => Ok(Self::PN),
            "PL" => Ok(Self::PL),
            "PT" => Ok(Self::PT),
            "QA" => Ok(Self::QA),
            "RE" => Ok(Self::RE),
            "RO" => Ok(Self::RO),
            "RU" => Ok(Self::RU),
            "RW" => Ok(Self::RW),
            "WS" => Ok(Self::WS),
            "SM" => Ok(Self::SM),
            "ST" => Ok(Self::ST),
            "SA" => Ok(Self::SA),
            "SN" => Ok(Self::SN),
            "RS" => Ok(Self::RS),
            "SC" => Ok(Self::SC),
            "SL" => Ok(Self::SL),
            "SG" => Ok(Self::SG),
            "SK" => Ok(Self::SK),
            "SI" => Ok(Self::SI),
            "SB" => Ok(Self::SB),
            "SO" => Ok(Self::SO),
            "ZA" => Ok(Self::ZA),
            "KR" => Ok(Self::KR),
            "ES" => Ok(Self::ES),
            "LK" => Ok(Self::LK),
            "SH" => Ok(Self::SH),
            "KN" => Ok(Self::KN),
            "LC" => Ok(Self::LC),
            "PM" => Ok(Self::PM),
            "VC" => Ok(Self::VC),
            "SR" => Ok(Self::SR),
            "SJ" => Ok(Self::SJ),
            "SZ" => Ok(Self::SZ),
            "SE" => Ok(Self::SE),
            "CH" => Ok(Self::CH),
            "TW" => Ok(Self::TW),
            "TJ" => Ok(Self::TJ),
            "TZ" => Ok(Self::TZ),
            "TH" => Ok(Self::TH),
            "TG" => Ok(Self::TG),
            "TO" => Ok(Self::TO),
            "TT" => Ok(Self::TT),
            "TN" => Ok(Self::TN),
            "TM" => Ok(Self::TM),
            "TC" => Ok(Self::TC),
            "TV" => Ok(Self::TV),
            "UG" => Ok(Self::UG),
            "UA" => Ok(Self::UA),
            "AE" => Ok(Self::AE),
            "GB" => Ok(Self::GB),
            "US" => Ok(Self::US),
            "UY" => Ok(Self::UY),
            "VU" => Ok(Self::VU),
            "VA" => Ok(Self::VA),
            "VE" => Ok(Self::VE),
            "VN" => Ok(Self::VN),
            "WF" => Ok(Self::WF),
            "YE" => Ok(Self::YE),
            "ZM" => Ok(Self::ZM),
            "ZW" => Ok(Self::ZW),
            country => Err(InvalidCountryError(country.to_owned())),
        }
    }
}
