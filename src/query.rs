//! Map query searches.

use crate::PAGE_SIZE;
use serde::Deserialize;
use validator::Validate;

fn default_page_size() -> i64 {
    PAGE_SIZE
}

/// Struct used to deserialize query strings.
#[derive(Debug, Clone, Deserialize, Validate, PartialEq, Eq)]
pub struct QuerySearch {
    pub q: Option<String>,
    pub sort: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0))]
    pub offset: i64,
    #[serde(default = "default_page_size")]
    #[validate(range(min = 1))]
    pub page_size: i64,
    pub include_total: Option<bool>,
}

impl QuerySearch {
    /// Parse sort argument "col1,col2,-col3..." into a vector of strings,
    /// and if the column name starts with "-", it's translated to a DESC
    /// keyword, e.g. "-name" --> "name DESC".
    ///
    /// ```
    /// use backset::query::QuerySearch;
    /// let q = QuerySearch { q: None, offset: 0, page_size: 10, sort: None, include_total: None };
    /// assert_eq!(q.parse_sort(&["a", "b"]), Vec::<String>::new());
    /// let q = QuerySearch { q: None, offset: 0, page_size: 10, sort: Some(String::from("a,-b")), include_total: None };
    /// assert_eq!(q.parse_sort(&["a", "b"]), &[String::from("a"), String::from("b DESC")]);
    /// let q = QuerySearch { q: None, offset: 0, page_size: 10, sort: Some(String::from("name,-b,c")), include_total: None };
    /// assert_eq!(q.parse_sort(&vec!["name", "c"]), &[String::from("name"), String::from("c")]);
    /// ```
    pub fn parse_sort(&self, allowed_fields: &[&str]) -> Vec<String> {
        self.sort
            .as_deref()
            .unwrap_or("")
            .split(',')
            .filter(|s| allowed_fields.contains(&s.strip_prefix('-').unwrap_or(s)))
            .map(|f| {
                f.strip_prefix('-')
                    .map(|d| format!("{d} DESC"))
                    .unwrap_or(f.to_string())
            })
            .collect()
    }

    /// Parse sort argument "col1,col2,-col3..." into a compatible SQL ORDER BY expression,
    /// e.g. "name,-age" --> "name, age DESC", to be concatenated in a SQL SELECT query.
    ///
    /// ```
    /// use backset::query::QuerySearch;
    /// let q = QuerySearch { q: None, offset: 0, page_size: 10, sort: None, include_total: None };
    /// assert_eq!(q.sort_as_order_by_args(&["a", "b"], "a"), "a");
    /// let q = QuerySearch { q: None, offset: 0, page_size: 10, sort: Some(String::from("a,-b")), include_total: None };
    /// assert_eq!(q.sort_as_order_by_args(&["a", "b"], "a"), "a, b DESC");
    /// let q = QuerySearch { q: None, offset: 0, page_size: 10, sort: Some(String::from("name,-b,c")), include_total: None };
    /// assert_eq!(q.sort_as_order_by_args(&["a", "h"], "c"), "c");
    /// ```
    pub fn sort_as_order_by_args(&self, allowed_fields: &[&str], default: &str) -> String {
        let sorting = self.parse_sort(allowed_fields);
        match sorting.len() {
            0 => String::from(default),
            _ => sorting.join(", "),
        }
    }
}