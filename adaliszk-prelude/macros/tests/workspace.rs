// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_prelude_macros::workspace_crates;
use googletest::prelude::*;

#[test]
fn finds_workspace_members_via_macro()
{
    let crates: Vec<String> = workspace_crates!();

    assert_that!(crates, not(is_empty()));
    assert_that!(crates, each(not(contains_substring("-"))));
    assert_that!(crates, contains(eq("adaliszk_prelude_macros")));
}
