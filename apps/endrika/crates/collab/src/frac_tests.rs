//! Fractional position keys.

use crate::frac::{FIRST_KEY, KeyError, between, n_between, validate};
use crate::sim::Rng;

#[test]
fn first_keys_count_up_and_down() {
    assert_eq!(between(None, None).unwrap(), FIRST_KEY);
    assert_eq!(between(Some("a0"), None).unwrap(), "a1");
    assert_eq!(between(Some("az"), None).unwrap(), "b00");
    assert_eq!(between(None, Some("a0")).unwrap(), "Zz");
    assert_eq!(between(None, Some("a1V")).unwrap(), "a1");
    assert_eq!(between(Some("a0"), Some("a1")).unwrap(), "a0V");
    assert_eq!(between(Some("a0"), Some("a2")).unwrap(), "a1");
    assert_eq!(between(Some("a0V"), Some("a1")).unwrap(), "a0l");
    assert_eq!(between(Some("a0"), Some("a0V")).unwrap(), "a0G");
    assert_eq!(between(Some("Zz"), Some("a0")).unwrap(), "ZzV");
}

#[test]
fn bad_keys_and_bounds_are_refused() {
    for bad in ["", "a", "a00", "a0 ", "b1", "!0", "a0é", "A00000000000000000000000000"] {
        assert!(validate(bad).is_err(), "{bad:?}");
        assert!(between(Some(bad), None).is_err());
    }
    assert!(validate(&format!("a0{}", "1".repeat(200))).is_err(), "too long");
    for good in ["a0", "a0V", "Zz", "b00", "zzzzzzzzzzzzzzzzzzzzzzzzzzz", "A000000000000000000000000001"] {
        assert_eq!(validate(good), Ok(()), "{good:?}");
    }
    assert_eq!(between(Some("a1"), Some("a1")), Err(KeyError::Order));
    assert_eq!(between(Some("a2"), Some("a1")), Err(KeyError::Order));
    // Past the largest integer, the fraction grows instead of failing.
    let top = "zzzzzzzzzzzzzzzzzzzzzzzzzzz";
    let after = between(Some(top), None).unwrap();
    assert!(after.as_str() > top);
}

#[test]
fn many_keys_are_ordered_and_short() {
    let keys = n_between(None, None, 1000).unwrap();
    assert!(keys.windows(2).all(|w| w[0] < w[1]));
    assert!(keys.iter().all(|k| k.len() <= 3), "counting up keeps keys short");
    let inside = n_between(Some("a0"), Some("a1"), 1000).unwrap();
    assert_eq!(inside.len(), 1000);
    assert!(inside.windows(2).all(|w| w[0] < w[1]));
    assert!(inside.first().unwrap().as_str() > "a0" && inside.last().unwrap().as_str() < "a1");
    assert!(inside.iter().all(|k| k.len() <= 8), "bisection keeps keys short");
    let below = n_between(None, Some("a0"), 50).unwrap();
    assert!(below.windows(2).all(|w| w[0] < w[1]) && below.last().unwrap().as_str() < "a0");
    assert!(n_between(Some("a0"), Some("a1"), 0).unwrap().is_empty());
}

#[test]
fn random_inserts_stay_ordered_valid_and_deterministic() {
    for seed in 0..20 {
        let mut rng = Rng::new(seed);
        let mut keys: Vec<String> = vec![between(None, None).unwrap()];
        for _ in 0..300 {
            let i = rng.below(keys.len() + 1);
            let lo = i.checked_sub(1).map(|j| keys[j].clone());
            let hi = keys.get(i).cloned();
            let k = between(lo.as_deref(), hi.as_deref()).unwrap();
            assert_eq!(validate(&k), Ok(()));
            assert_eq!(between(lo.as_deref(), hi.as_deref()).unwrap(), k, "deterministic");
            assert!(lo.as_deref().is_none_or(|l| l < k.as_str()) && hi.as_deref().is_none_or(|h| k.as_str() < h));
            keys.insert(i, k);
        }
        assert!(keys.windows(2).all(|w| w[0] < w[1]));
    }
}

#[test]
fn appending_and_prepending_forever_never_fails() {
    let mut k = FIRST_KEY.to_string();
    for _ in 0..5000 {
        k = between(Some(&k), None).unwrap();
    }
    assert!(k.len() <= 4);
    let mut k = FIRST_KEY.to_string();
    for _ in 0..5000 {
        k = between(None, Some(&k)).unwrap();
    }
    assert!(k.len() <= 4);
    // Squeezing next to one key: the fraction grows by about one digit per six inserts.
    let mut hi = "a1".to_string();
    for _ in 0..60 {
        hi = between(Some("a0"), Some(&hi)).unwrap();
    }
    assert!(hi.len() <= 16, "{hi}");
}

#[test]
fn junk_keys_never_panic() {
    let mut rng = Rng::new(7);
    for _ in 0..5000 {
        let a: String = (0..rng.below(12)).map(|_| char::from(rng.below(128) as u8)).collect();
        let b: String = (0..rng.below(12)).map(|_| char::from(32 + rng.below(95) as u8)).collect();
        let _ = validate(&a);
        let _ = between(Some(&a), Some(&b));
        let _ = between(None, Some(&b));
        let _ = between(Some(&a), None);
        let _ = n_between(Some(&a), Some(&b), 5);
    }
}
