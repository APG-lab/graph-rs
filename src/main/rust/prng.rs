
use crate::error;
use std::ops;

pub const P0: u64 = 0xa076_1d64_78bd_642f;
pub const P1: u64 = 0xe703_7ed1_a0b4_28db;

#[inline]
fn wymum (a: u64, b: u64)
    -> u64
{
    let r = u128::from(a) * u128::from(b);
    ((r >> 64) ^ r) as u64
}

// Pseudo random number generator
// https://github.com/eldruin/wyhash-rs
pub fn wyrng (seed: &mut u64)
    -> u64
{
    *seed = seed.wrapping_add (P0);
    wymum (*seed, *seed ^ P1)
}

pub fn wyrng_range (range: ops::Range<u64>, seed: &mut u64)
    -> u64
{
    let mut x: u64;
    loop
    {
        x = wyrng (seed);
        if x < (u64::MAX - u64::MAX % (range.end - range.start))
        {
            break
        }
    }
    x %= range.end - range.start;
    x + range.start
}

pub fn shuffle<T> (data: &mut [T], seed: &mut u64)
    -> Result<(), error::GraphError> 
    where
        T: Clone
{
    for i in (1..data.len ()).rev ()
    {
        let j = wyrng_range (0..u64::try_from (i+1)?, seed);
        data.swap (i, j.try_into ()?);
    }
    Ok (())
}

#[cfg(test)]
mod tests
{
    use log::debug;
    use std::collections;
    use std::sync;

    static INIT: sync::Once = sync::Once::new ();

    fn mean (data: &[i32])
        -> Option<f32>
    {
        let sum = data.iter ().sum::<i32> () as f32;
        let count = data.len ();

        match count {
            positive if positive > 0 => Some (sum / count as f32),
            _ => None,
        }
    }

    fn std_deviation (data: &[i32])
        -> Option<f32>
    {
        match (mean (data), data.len ())
        {
            (Some (data_mean), count) if count > 0 => {
                let variance = data.iter ().map (|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                }).sum::<f32> () / count as f32;

                Some (variance.sqrt ())
            },
            _ => None
        }
    }

    fn init ()
    {
        INIT.call_once (env_logger::init);
    }

    #[test]
    fn test_range ()
    {
        init ();

        assert_eq! (super::wyrng_range (0..2, &mut 42u64), 0u64);
        assert_eq! (super::wyrng_range (0..2, &mut 44u64), 1u64);
        assert_eq! (super::wyrng_range (1..3, &mut 42u64), 1u64);
        assert_eq! (super::wyrng_range (1..3, &mut 44u64), 2u64);
    }

    #[test]
    fn test_trials ()
    {
        init ();
        let mut seed = 42u64;

        for ts in vec![1,2,3,4,5,6,7,8,9,10,100,1000,10_000]
        {
            for ns in vec![2,3,4]
            {
                let trials = ts;
                let n = ns;
                let t = (0..trials).map (|_| { super::wyrng_range (0..n, &mut seed) }).collect::<Vec<_>> ();
                let tc = t.iter ().fold (collections::HashMap::<usize, usize>::new (), |mut acc, item| { *acc.entry (*item as usize).or_insert (0) += 1;acc });

                let mut tck = tc.keys ().collect::<Vec<_>> ();
                tck.sort ();
                for k in tck
                {
                    debug! ("{}\t{}\t{}\t{}\t{:.02}", n, trials, k, tc[k], (tc[k] * 10_000 / trials) as f32 / 100.0);
                }
            }
        }
    }

    #[test]
    fn test_shuffle ()
    {
        init ();
        let mut seed = 42u64;

        let n = 10_000;
        let mut v = (0..n).collect::<Vec<i32>> ();
        super::shuffle (&mut v, &mut seed).expect ("Failed to shuffle");

        let sd = std_deviation (&v).expect ("Failed to calculate sd");
        assert_eq! (sd.floor (), (n as f32/12f32.sqrt ()).floor ()); 
    }
}


