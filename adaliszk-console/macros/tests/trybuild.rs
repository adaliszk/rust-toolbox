// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

#[test]
fn console_macro_rejects_invalid_usage()
{
    trybuild::TestCases::new().compile_fail("tests/examples/*.rs");
}
