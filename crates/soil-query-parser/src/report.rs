//! Report generation for parsing statistics

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Statistics for a single country
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryStats {
    pub profiles: usize,
    pub errors: usize,
}

/// Overall parsing report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseReport {
    pub summary: Summary,
    pub by_country: HashMap<String, CountryStats>,
    pub errors: Vec<ErrorRecord>,
    pub performance: Performance,
}

/// Summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub total_files: usize,
    pub total_profiles: usize,
    pub successful: usize,
    pub failed: usize,
    pub duration_seconds: f64,
}

/// Individual error record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecord {
    pub file: String,
    pub profile_id: Option<String>,
    pub error: String,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Performance {
    pub profiles_per_second: f64,
    pub database_size_mb: f64,
}

impl ParseReport {
    /// Create a new empty report
    pub fn new() -> Self {
        Self {
            summary: Summary {
                total_files: 0,
                total_profiles: 0,
                successful: 0,
                failed: 0,
                duration_seconds: 0.0,
            },
            by_country: HashMap::new(),
            errors: Vec::new(),
            performance: Performance {
                profiles_per_second: 0.0,
                database_size_mb: 0.0,
            },
        }
    }

    /// Record successful profile insertion
    pub fn record_success(&mut self, country_code: &str) {
        self.summary.total_profiles += 1;
        self.summary.successful += 1;

        self.by_country
            .entry(country_code.to_string())
            .or_insert(CountryStats {
                profiles: 0,
                errors: 0,
            })
            .profiles += 1;
    }

    /// Record an error
    pub fn record_error(&mut self, file: String, profile_id: Option<String>, error: String) {
        self.summary.failed += 1;

        self.errors.push(ErrorRecord {
            file: file.clone(),
            profile_id,
            error,
        });

        // Extract country code from filename (e.g., "US.SOL" -> "US")
        if let Some(country_code) = file.strip_suffix(".SOL") {
            self.by_country
                .entry(country_code.to_string())
                .or_insert(CountryStats {
                    profiles: 0,
                    errors: 0,
                })
                .errors += 1;
        }
    }

    /// Finalize the report with performance metrics
    pub fn finalize(&mut self, duration_seconds: f64, db_path: &Path) -> Result<()> {
        self.summary.duration_seconds = duration_seconds;

        // Calculate profiles per second
        if duration_seconds > 0.0 {
            self.performance.profiles_per_second =
                self.summary.total_profiles as f64 / duration_seconds;
        }

        // Get database size
        if db_path.exists() {
            let metadata = std::fs::metadata(db_path)?;
            self.performance.database_size_mb = metadata.len() as f64 / 1_048_576.0;
        }

        Ok(())
    }

    /// Save report to JSON file
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Print summary to console
    pub fn print_summary(&self) {
        println!("\n{}", "=".repeat(60));
        println!("PARSING COMPLETE");
        println!("{}", "=".repeat(60));
        println!("Files processed:    {}", self.summary.total_files);
        println!("Profiles parsed:    {}", self.summary.total_profiles);
        println!("Successful:         {}", self.summary.successful);
        println!("Failed:             {}", self.summary.failed);
        println!("Duration:           {:.2}s", self.summary.duration_seconds);
        println!(
            "Profiles/second:    {:.0}",
            self.performance.profiles_per_second
        );
        println!(
            "Database size:      {:.2} MB",
            self.performance.database_size_mb
        );

        if !self.errors.is_empty() {
            println!("\nErrors: {}", self.errors.len());
            for error in self.errors.iter().take(5) {
                println!("  - {}: {}", error.file, error.error);
            }
            if self.errors.len() > 5 {
                println!("  ... and {} more", self.errors.len() - 5);
            }
        }

        println!("{}", "=".repeat(60));
    }
}

impl Default for ParseReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_record_success() {
        let mut report = ParseReport::new();

        report.record_success("US");
        report.record_success("US");
        report.record_success("GI");

        assert_eq!(report.summary.total_profiles, 3);
        assert_eq!(report.summary.successful, 3);
        assert_eq!(report.by_country.get("US").unwrap().profiles, 2);
        assert_eq!(report.by_country.get("GI").unwrap().profiles, 1);
    }

    #[test]
    fn test_report_record_error() {
        let mut report = ParseReport::new();

        report.record_error(
            "US.SOL".to_string(),
            Some("US12345678".to_string()),
            "Invalid coordinates".to_string(),
        );

        assert_eq!(report.summary.failed, 1);
        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.by_country.get("US").unwrap().errors, 1);
    }

    #[test]
    fn test_report_finalize() -> Result<()> {
        let mut report = ParseReport::new();
        report.summary.total_profiles = 1000;

        let temp_db = std::env::temp_dir().join("test_report.db");
        std::fs::write(&temp_db, b"test data")?;

        report.finalize(10.0, &temp_db)?;

        assert_eq!(report.summary.duration_seconds, 10.0);
        assert_eq!(report.performance.profiles_per_second, 100.0);
        assert!(report.performance.database_size_mb > 0.0);

        std::fs::remove_file(&temp_db)?;

        Ok(())
    }
}
