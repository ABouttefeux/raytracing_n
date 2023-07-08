//! version test module

/// readme version number test.
#[test]
fn readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

/// html root url version test.
#[test]
fn html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
