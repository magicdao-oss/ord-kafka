use super::*;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Rarity {
  Common,
  Uncommon,
  Rare,
  Epic,
  Legendary,
  Mythic,
  BlackUncommon,
  BlackRare,
  BlackEpic,
  BlackLegendary,
}

impl From<Rarity> for u8 {
  fn from(rarity: Rarity) -> Self {
    rarity as u8
  }
}

impl TryFrom<u8> for Rarity {
  type Error = u8;

  fn try_from(rarity: u8) -> Result<Self, u8> {
    match rarity {
      0 => Ok(Self::Common),
      1 => Ok(Self::Uncommon),
      2 => Ok(Self::Rare),
      3 => Ok(Self::Epic),
      4 => Ok(Self::Legendary),
      5 => Ok(Self::Mythic),
      n => Err(n),
    }
  }
}

impl Display for Rarity {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Common => "common",
        Self::Uncommon => "uncommon",
        Self::Rare => "rare",
        Self::Epic => "epic",
        Self::Legendary => "legendary",
        Self::Mythic => "mythic",
        Self::BlackUncommon => "black_uncommon",
        Self::BlackRare => "black_rare",
        Self::BlackEpic => "black_epic",
        Self::BlackLegendary => "black_legendary",
      }
    )
  }
}

impl From<Sat> for Rarity {
  fn from(sat: Sat) -> Self {
    let Degree {
      hour,
      minute,
      second,
      third,
    } = sat.degree();

    if hour == 0 && minute == 0 && second == 0 && third == 0 {
      Self::Mythic
    } else if minute == 0 && second == 0 && third == 0 {
      Self::Legendary
    } else if minute == 0 && third == 0 {
      Self::Epic
    } else if second == 0 && third == 0 {
      Self::Rare
    } else if third == 0 {
      Self::Uncommon
    } else if third == sat.epoch().subsidy() - 1 {
      if minute == SUBSIDY_HALVING_INTERVAL - 1 {
        if second == DIFFCHANGE_INTERVAL - 1 {
          Self::BlackLegendary
        } else {
          Self::BlackEpic
        }
      } else if second == DIFFCHANGE_INTERVAL - 1 {
        Self::BlackRare
      } else {
        Self::BlackUncommon
      }
    } else {
      Self::Common
    }
  }
}

impl FromStr for Rarity {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "common" => Ok(Self::Common),
      "uncommon" => Ok(Self::Uncommon),
      "rare" => Ok(Self::Rare),
      "epic" => Ok(Self::Epic),
      "legendary" => Ok(Self::Legendary),
      "mythic" => Ok(Self::Mythic),
      "black_uncommon" => Ok(Self::BlackUncommon),
      "black_rare" => Ok(Self::BlackRare),
      "black_epic" => Ok(Self::BlackEpic),
      "black_legendary" => Ok(Self::BlackLegendary),
      _ => Err(format!("invalid rarity `{s}`")),
    }
  }
}

impl Serialize for Rarity {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for Rarity {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    DeserializeFromStr::with(deserializer)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rarity() {
    assert_eq!(Sat(0).rarity(), Rarity::Mythic);
    assert_eq!(Sat(1).rarity(), Rarity::Common);

    assert_eq!(Sat(50 * COIN_VALUE - 1).rarity(), Rarity::BlackUncommon);
    assert_eq!(Sat(50 * COIN_VALUE).rarity(), Rarity::Uncommon);
    assert_eq!(Sat(50 * COIN_VALUE + 1).rarity(), Rarity::Common);

    assert_eq!(
      Sat(50 * COIN_VALUE * u64::from(DIFFCHANGE_INTERVAL) - 1).rarity(),
      Rarity::BlackRare
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * u64::from(DIFFCHANGE_INTERVAL)).rarity(),
      Rarity::Rare
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * u64::from(DIFFCHANGE_INTERVAL) + 1).rarity(),
      Rarity::Common
    );

    assert_eq!(
      Sat(50 * COIN_VALUE * u64::from(SUBSIDY_HALVING_INTERVAL) - 1).rarity(),
      Rarity::BlackEpic
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * u64::from(SUBSIDY_HALVING_INTERVAL)).rarity(),
      Rarity::Epic
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * u64::from(SUBSIDY_HALVING_INTERVAL) + 1).rarity(),
      Rarity::Common
    );

    assert_eq!(Sat(2067187500000000 - 1).rarity(), Rarity::BlackLegendary);
    assert_eq!(Sat(2067187500000000).rarity(), Rarity::Legendary);
    assert_eq!(Sat(2067187500000000 + 1).rarity(), Rarity::Common);
  }

  #[test]
  fn from_str_and_deserialize_ok() {
    #[track_caller]
    fn case(s: &str, expected: Rarity) {
      let actual = s.parse::<Rarity>().unwrap();
      assert_eq!(actual, expected);
      let round_trip = actual.to_string().parse::<Rarity>().unwrap();
      assert_eq!(round_trip, expected);
      let serialized = serde_json::to_string(&expected).unwrap();
      assert!(serde_json::from_str::<Rarity>(&serialized).is_ok());
    }

    case("common", Rarity::Common);
    case("uncommon", Rarity::Uncommon);
    case("rare", Rarity::Rare);
    case("epic", Rarity::Epic);
    case("legendary", Rarity::Legendary);
    case("mythic", Rarity::Mythic);
    case("black_uncommon", Rarity::BlackUncommon);
    case("black_rare", Rarity::BlackRare);
    case("black_epic", Rarity::BlackEpic);
    case("black_legendary", Rarity::BlackLegendary);
  }

  #[test]
  fn conversions_with_u8() {
    for &expected in &[
      Rarity::Common,
      Rarity::Uncommon,
      Rarity::Rare,
      Rarity::Epic,
      Rarity::Legendary,
      Rarity::Mythic,
    ] {
      let n: u8 = expected.into();
      let actual = Rarity::try_from(n).unwrap();
      assert_eq!(actual, expected);
    }

    assert_eq!(Rarity::try_from(6), Err(6));
  }

  #[test]
  fn error() {
    assert_eq!("foo".parse::<Rarity>().unwrap_err(), "invalid rarity `foo`");
  }
}
