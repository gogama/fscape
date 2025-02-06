use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FilesystemRule {
    /// Path to root of monitored directory tree, also indicates the filesystem to monitor.
    /// 
    /// This does *not* necessarily need to be the root directory of the filesystem (*i.e.* the
    /// mounting directory). However, it must contain the path of a directory in the filesystem
    /// being monitored, and all deletion rules will be evaulated relative to this directory, so
    /// only files that are children of this directory can ever be deleted.
    pub root_directory_path: String,

    /// Threshold level of filesystem usage as a percentage of blocks used to total blocks above
    /// which the rule deletion groups will be evaluated.
    pub usage_threshold_pct: Option<u8>,

    /// Threshold level of filesystem usage as a number of megabytes (MB), where 1 MB is 1 million
    /// bytes.
    pub usage_threshold_mb: Option<u64>,

    /// How often to check the thresholds, e.g. once per minute.
    pub period: Duration,

    /// Deletion rules to run if either threshold level is breached.
    pub deletion_rules: Vec<DeletionRule>,
}

// A rule specifying how to identify and delete files within a monitored directory tree when
// filesystem usage thresholds are exceeded.
//
// Deletion rules are executed in the following way. First, the full set of candidates files is
// identified by evaluating the all the include and exclude patterns. The candidate set is
// constructed as the set of all files that match the include patterns minus any files that match
// the exclude patterns minus any files that are directories.
#[derive(Debug, Clone)]
pub struct DeletionRule {
    // File path patterns relative to the root directory of the monitored directory tree that are
    // *potentially* subject to deletion by this rule.
    //
    // This list must contain at least one entry.
    //
    // Each entry in this list must be a "concrete" relative path, or a relative path "pattern",
    // where the difference between a concrete path and a path pattern is that the pattern may
    // contain globs, including the globstar (`**`).
    pub include_patterns: Vec<String>,

    // File paths patterns relative to the root directory of the monitored directory tree that are
    // *never* subject to deletion by this rule.
    //
    // Each entry in this list must be a "concrete" relative path, or a relative path "pattern",
    // where the difference between a concrete path and a path pattern is that the pattern may
    // contain globs, including the globstar (`**`).
    pub exclude_patterns: Vec<String>,

    // TODO: Symbolic links.

    /// Attribute is an enum, e.g. Age.
    /// OrderedAttribute is an Attribute
    /// with ASC or DESC ordering attached.
    pub sort: Vec<OrderedAttribute>,

    /// Subset of the attributes from Sort.
    /// When you reach any of these limits
    /// in the sorted list of files, e.g.
    /// an age limit, deletion must stop.
    pub limits: HashMap<Attribute, Value>,
}

// Assuming these enums/structs are defined elsewhere
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    Age,
    Size,
    // Add other attributes as needed
}

#[derive(Debug, Clone)]
pub struct OrderedAttribute {
    pub attribute: Attribute,
    pub order: SortOrder,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

// Placeholder for Value; adjust based on actual use case
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(String),
    // Add more variants if needed
}