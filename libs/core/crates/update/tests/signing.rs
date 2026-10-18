//! A text signed with `release-keys/update-signing.pem` by `openssl pkeyutl -sign -rawin` (what
//! `scripts/stage-release.sh` does) verifies against the key built into the apps.

#[test]
fn release_script_signatures_verify_with_the_built_in_key() {
    let sums = include_bytes!("fixtures/sums.txt");
    let sig = include_bytes!("fixtures/sums.txt.sig");
    assert!(mahay_update::verify_sums(sums, sig, None));
    let mut tampered = sums.to_vec();
    tampered[0] ^= 1;
    assert!(!mahay_update::verify_sums(&tampered, sig, None));
}
