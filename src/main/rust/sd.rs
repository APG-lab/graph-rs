
pub mod key_pair_usize
{
    use serde::{self, Deserialize, Serializer, Deserializer};
    use std::collections;

    pub fn deserialize<'de, D, V> (deserializer: D)
        -> Result<collections::HashMap<(usize, usize), V>, D::Error>
    where
        D: Deserializer<'de>,
        V: serde::Deserialize<'de>
    {
        collections::HashMap::<String,_>::deserialize (deserializer)
            .map (|mut v| Ok (v.drain ().try_fold (collections::HashMap::<(usize, usize), V>::new (), |mut acc, (mk, mv)| {
                let (a, b) = mk.strip_prefix ("(")
                    .ok_or (serde::de::Error::custom (format! ("Invalid key: {}", mk)))?
                    .strip_suffix (")")
                    .ok_or (serde::de::Error::custom (format! ("Invalid key: {}", mk)))?
                    .split_once (",").ok_or (serde::de::Error::custom (format! ("Invalid key: {}", mk)))?;
                let au = a.parse::<usize> ().map_err (serde::de::Error::custom)?;
                let bu = b.parse::<usize> ().map_err (serde::de::Error::custom)?;
                acc.insert ((au, bu), mv);
                Ok (acc)
            })?))?
    }

    pub fn serialize<S, V> (hs: &collections::HashMap<(usize, usize), V>, serializer: S)
        -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: serde::Serialize
    {
        serializer.collect_map(hs.iter().map(|(k, v)| (format! ("({},{})", k.0, k.1), v)))
    }
}

pub mod key_usize
{
    use serde::{self, Deserialize, Serializer, Deserializer};
    use std::collections;

    pub fn deserialize<'de, D, V> (deserializer: D)
        -> Result<collections::HashMap<usize, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: serde::Deserialize<'de>
    {
        collections::HashMap::<String,_>::deserialize (deserializer)
            .map (|mut v| Ok (v.drain ().try_fold (collections::HashMap::<usize, V>::new (), |mut acc, (mk, mv)| {
                acc.insert (mk.parse::<usize> ().map_err (serde::de::Error::custom)?, mv);
                Ok (acc)
            })?))?
    }

    pub fn serialize<S, V> (hs: &collections::HashMap<usize, V>, serializer: S)
        -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: serde::Serialize
    {
        serializer.collect_map(hs.iter().map(|(k, v)| (k.to_string (), v)))
    }
}



