//! Parser for .SOL format files

use crate::error::{Result, SoilError};
use crate::types::{Location, Metadata, SiteProperties, SiteWideProperties, SoilLayer, SoilProfile};

/// Sentinel value representing missing data in .SOL files
const MISSING_VALUE: f64 = -99.0;

/// Number of soil layers in a standard soil profile
const NUM_LAYERS: usize = 6;

/// Number of fields per soil layer line
const LAYER_FIELDS_COUNT: usize = 17;

/// Minimum number of lines required for a complete soil profile
const MIN_PROFILE_LINES: usize = 10;

/// Check if a value represents missing data
fn is_missing(value: f64) -> bool {
    (value - MISSING_VALUE).abs() < 0.01
}

/// Convert f64 to Option, treating -99 as None
fn optional_f64(value: f64) -> Option<f64> {
    if is_missing(value) {
        None
    } else {
        Some(value)
    }
}

impl SoilProfile {
    /// Parse a .SOL format string into a vector of SoilProfiles
    /// 
    /// A .SOL file may contain multiple soil profiles, each starting with a header line.
    pub fn from_sol_format(content: &str) -> Result<Vec<Self>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut profiles = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            // Skip empty lines
            if lines[i].trim().is_empty() {
                i += 1;
                continue;
            }

            // Check if this is a profile header line (starts with *)
            if lines[i].starts_with('*') {
                let profile = Self::parse_single_profile(&lines[i..])?;
                profiles.push(profile);
                
                // Skip past this profile (header + @SITE + @ SCOM + @  SLB header + 6 layers = 10 lines minimum)
                i += 10;
            } else {
                i += 1;
            }
        }

        if profiles.is_empty() {
            return Err(SoilError::ParseError("No valid soil profiles found".to_string()));
        }

        Ok(profiles)
    }

    /// Parse a single soil profile starting from the header line
    fn parse_single_profile(lines: &[&str]) -> Result<Self> {
        if lines.len() < MIN_PROFILE_LINES {
            return Err(SoilError::ParseError(format!(
                "Incomplete profile data: expected at least {} lines, got {}",
                MIN_PROFILE_LINES,
                lines.len()
            )));
        }

        // Parse header line: *ID    COUNTRY_ALPHA3    TextureName   DEPTH    SOURCE
        let (id, mut site, metadata) = Self::parse_header(lines[0])?;
        
        // Parse @SITE line (returns location and scs_family)
        let (location, scs_family) = Self::parse_site(lines[1], lines[2])?;
        site.scs_family = scs_family;
        
        // Parse @ SCOM line
        let properties = Self::parse_scom(lines[3], lines[4])?;
        
        // Parse @  SLB section (layers)
        let layers = Self::parse_layers(&lines[5..])?;

        Ok(SoilProfile {
            id,
            location,
            site,
            properties,
            layers,
            metadata,
        })
    }
    
    /// Parse the header line (first line starting with *)
    fn parse_header(line: &str) -> Result<(String, SiteProperties, Metadata)> {
        // Format: *ID    COUNTRY_ALPHA3    TextureName   DEPTH    SOURCE
        // Example: *GI02792815    GIB        Loam   200    ISRIC soilgrids + HC27
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < 5 {
            return Err(SoilError::ParseError(format!("Invalid header line: {}", line)));
        }

        let id = parts[0].trim_start_matches('*').to_string();
        let country_code_alpha3 = parts[1].to_string();
        let texture_name = parts[2].to_string();
        let max_depth_cm = parts[3].parse::<u32>()
            .map_err(|_| SoilError::InvalidValue {
                field: "max_depth_cm".to_string(),
                value: parts[3].to_string(),
            })?;
        
        // Rest is the source (may contain spaces)
        let source = parts[4..].join(" ");

        let site = SiteProperties {
            country_code_alpha3,
            scs_family: String::new(), // Will be filled from @SITE line
            texture: texture_name,
            max_depth_cm,
        };

        let metadata = Metadata {
            source,
            distance_km: None,
        };

        Ok((id, site, metadata))
    }

    /// Parse @SITE section (2 lines)
    fn parse_site(_header_line: &str, data_line: &str) -> Result<(Location, String)> {
        // Data line format: -99    CC    LAT    LON    SCS_FAMILY
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        
        if parts.len() < 5 {
            return Err(SoilError::ParseError(format!("Invalid @SITE line: {}", data_line)));
        }

        let country_code = parts[1].to_string();
        let lat = parts[2].parse::<f64>()
            .map_err(|_| SoilError::InvalidValue {
                field: "lat".to_string(),
                value: parts[2].to_string(),
            })?;
        let lon = parts[3].parse::<f64>()
            .map_err(|_| SoilError::InvalidValue {
                field: "lon".to_string(),
                value: parts[3].to_string(),
            })?;
        let scs_family = parts[4].to_string();

        let location = Location {
            lat,
            lon,
            country_code,
        };

        Ok((location, scs_family))
    }

    /// Parse @ SCOM section (2 lines)
    fn parse_scom(_header_line: &str, data_line: &str) -> Result<SiteWideProperties> {
        // Data line format: SCOM  SALB  SLU1  SLDR  SLRO  SLNF  SLPF  SMHB  SMPX  SMKE
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        
        if parts.len() < 10 {
            return Err(SoilError::ParseError(format!("Invalid @ SCOM line: {}", data_line)));
        }

        Ok(SiteWideProperties {
            scom: parts[0].to_string(),
            salb: parts[1].parse().map_err(|_| SoilError::InvalidValue {
                field: "salb".to_string(),
                value: parts[1].to_string(),
            })?,
            slu1: parts[2].parse().map_err(|_| SoilError::InvalidValue {
                field: "slu1".to_string(),
                value: parts[2].to_string(),
            })?,
            sldr: parts[3].parse().map_err(|_| SoilError::InvalidValue {
                field: "sldr".to_string(),
                value: parts[3].to_string(),
            })?,
            slro: parts[4].parse().map_err(|_| SoilError::InvalidValue {
                field: "slro".to_string(),
                value: parts[4].to_string(),
            })?,
            slnf: parts[5].parse().map_err(|_| SoilError::InvalidValue {
                field: "slnf".to_string(),
                value: parts[5].to_string(),
            })?,
            slpf: parts[6].parse().map_err(|_| SoilError::InvalidValue {
                field: "slpf".to_string(),
                value: parts[6].to_string(),
            })?,
            smhb: parts[7].to_string(),
            smpx: parts[8].to_string(),
            smke: parts[9].to_string(),
        })
    }

    /// Parse @  SLB section (layer data)
    fn parse_layers(lines: &[&str]) -> Result<Vec<SoilLayer>> {
        // Skip the header line (@  SLB ...)
        let data_lines: Vec<&str> = lines.iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.starts_with('@') && !line.starts_with('*'))
            .take(NUM_LAYERS)
            .copied()
            .collect();

        if data_lines.is_empty() {
            return Err(SoilError::ParseError("No layer data found".to_string()));
        }

        data_lines.iter().map(|line| Self::parse_layer(line)).collect()
    }

    /// Parse a single layer line
    fn parse_layer(line: &str) -> Result<SoilLayer> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < LAYER_FIELDS_COUNT {
            return Err(SoilError::ParseError(format!(
                "Invalid layer line: expected {} fields, got {}",
                LAYER_FIELDS_COUNT,
                parts.len()
            )));
        }

        // Pure helper function for parsing floats
        let parse_f64_at = |idx: usize, field_name: &str| -> Result<f64> {
            parts[idx].parse::<f64>()
                .map_err(|_| SoilError::InvalidValue {
                    field: field_name.to_string(),
                    value: parts[idx].to_string(),
                })
        };

        Ok(SoilLayer {
            slb: parts[0].parse().map_err(|_| SoilError::InvalidValue {
                field: "slb".to_string(),
                value: parts[0].to_string(),
            })?,
            slmh: parts[1].to_string(),
            slll: optional_f64(parse_f64_at(2, "slll")?),
            sdul: optional_f64(parse_f64_at(3, "sdul")?),
            ssat: optional_f64(parse_f64_at(4, "ssat")?),
            srgf: optional_f64(parse_f64_at(5, "srgf")?),
            ssks: optional_f64(parse_f64_at(6, "ssks")?),
            sbdm: optional_f64(parse_f64_at(7, "sbdm")?),
            sloc: optional_f64(parse_f64_at(8, "sloc")?),
            slcl: optional_f64(parse_f64_at(9, "slcl")?),
            slsi: optional_f64(parse_f64_at(10, "slsi")?),
            slcf: optional_f64(parse_f64_at(11, "slcf")?),
            slni: optional_f64(parse_f64_at(12, "slni")?),
            slhw: optional_f64(parse_f64_at(13, "slhw")?),
            slhb: optional_f64(parse_f64_at(14, "slhb")?),
            scec: optional_f64(parse_f64_at(15, "scec")?),
            sadc: optional_f64(parse_f64_at(16, "sadc")?),
        })
    }

    /// Convert this SoilProfile to .SOL format string
    pub fn to_sol_format(&self) -> String {
        let mut output = String::new();
        
        // Helper to format Option<f64> - None becomes "-99.0"
        let fmt_optional = |opt: Option<f64>| -> String {
            match opt {
                Some(val) => format!("{:5.2}", val),
                None => "-99.0".to_string(),
            }
        };
        
        // Helper for layer values with 3 decimal places
        let fmt_layer_value = |opt: Option<f64>| -> String {
            match opt {
                Some(val) => format!("{:5.3}", val),
                None => "-99.0".to_string(),
            }
        };
        
        // Header line: *ID    COUNTRY_ALPHA3    TextureName   DEPTH    SOURCE
        output.push_str(&format!(
            "*{:<14} {:<10} {:>10} {:>5}    {}\n",
            self.id,
            self.site.country_code_alpha3,
            self.site.texture,
            self.site.max_depth_cm,
            self.metadata.source
        ));
        
        // @SITE header
        output.push_str("@SITE        COUNTRY          LAT     LONG SCS Family\n");
        
        // @SITE data
        output.push_str(&format!(
            " -99              {:>2} {:>11.3} {:>9.3}     {}\n",
            self.location.country_code,
            self.location.lat,
            self.location.lon,
            self.site.scs_family
        ));
        
        // @ SCOM header
        output.push_str("@ SCOM  SALB  SLU1  SLDR  SLRO  SLNF  SLPF  SMHB  SMPX  SMKE\n");
        
        // @ SCOM data
        output.push_str(&format!(
            "    {} {:5.2} {:5.2} {:5.2} {:5.2} {:5.2} {:5.2} {} {} {}\n",
            self.properties.scom,
            self.properties.salb,
            self.properties.slu1,
            self.properties.sldr,
            self.properties.slro,
            self.properties.slnf,
            self.properties.slpf,
            self.properties.smhb,
            self.properties.smpx,
            self.properties.smke
        ));
        
        // @  SLB header
        output.push_str("@  SLB  SLMH  SLLL  SDUL  SSAT  SRGF  SSKS  SBDM  SLOC  SLCL  SLSI  SLCF  SLNI  SLHW  SLHB  SCEC  SADC\n");
        
        // Layer data - using 3 decimal places for most values, 2 for percentages
        for layer in &self.layers {
            output.push_str(&format!(
                " {:>5} {:<5} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n",
                layer.slb,
                layer.slmh,
                fmt_layer_value(layer.slll),  // 3 decimals
                fmt_layer_value(layer.sdul),  // 3 decimals
                fmt_layer_value(layer.ssat),  // 3 decimals
                fmt_optional(layer.srgf),     // 2 decimals
                fmt_optional(layer.ssks),     // 2 decimals
                fmt_optional(layer.sbdm),     // 2 decimals
                fmt_optional(layer.sloc),     // 2 decimals
                fmt_optional(layer.slcl),     // 2 decimals
                fmt_optional(layer.slsi),     // 2 decimals
                fmt_optional(layer.slcf),     // 2 decimals
                fmt_optional(layer.slni),     // 2 decimals
                fmt_optional(layer.slhw),     // 2 decimals
                fmt_optional(layer.slhb),     // 2 decimals
                fmt_optional(layer.scec),     // 2 decimals (actually 1 decimal in original)
                fmt_optional(layer.sadc),     // 2 decimals
            ));
        }
        
        output
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_missing_recognizes_sentinel_value() {
        assert!(is_missing(-99.0));
        assert!(is_missing(-99.001)); // Within tolerance
        assert!(!is_missing(0.0));
        assert!(!is_missing(5.76));
        assert!(!is_missing(-98.0));
    }

    #[test]
    fn test_optional_f64_converts_missing_to_none() {
        assert_eq!(optional_f64(-99.0), None);
        assert_eq!(optional_f64(5.76), Some(5.76));
        assert_eq!(optional_f64(0.0), Some(0.0));
        assert_eq!(optional_f64(-50.0), Some(-50.0));
    }

    #[test]
    fn test_parse_header_extracts_all_fields() {
        let line = "*GI02792815    GIB        Loam   200    ISRIC soilgrids + HC27";
        let result = SoilProfile::parse_header(line);
        
        assert!(result.is_ok());
        let (id, site, metadata) = result.unwrap();
        
        assert_eq!(id, "GI02792815");
        assert_eq!(site.country_code_alpha3, "GIB");
        assert_eq!(site.texture, "Loam");
        assert_eq!(site.max_depth_cm, 200);
        assert_eq!(metadata.source, "ISRIC soilgrids + HC27");
    }
}

