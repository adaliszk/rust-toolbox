// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

#[test]
fn resolvable_derive_rejects_invalid_usage()
{
    trybuild::TestCases::new().compile_fail("tests/examples/*.rs");
}
