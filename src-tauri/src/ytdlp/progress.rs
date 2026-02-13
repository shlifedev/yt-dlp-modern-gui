use super::types::ProgressInfo;
use once_cell::sync::Lazy;
use regex::Regex;

static PROGRESS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^download:([0-9.]+)%\|([^|]*)\|([^|]*)$").expect("Invalid regex"));

/// Parse a single progress line from yt-dlp stderr
/// Input format (from --progress-template): "download:XX.X%|2.5MiB/s|00:01:30" or similar
pub fn parse_progress_line(line: &str) -> Option<ProgressInfo> {
    let line = line.trim();

    if let Some(captures) = PROGRESS_RE.captures(line) {
        let percent_str = captures.get(1)?.as_str();
        let speed_str = captures.get(2)?.as_str().trim();
        let eta_str = captures.get(3)?.as_str().trim();

        let percent = percent_str.parse::<f32>().ok()?;

        let speed = if speed_str.is_empty() || speed_str == "N/A" || speed_str == "Unknown" {
            None
        } else {
            Some(speed_str.to_string())
        };

        let eta = if eta_str.is_empty() || eta_str == "N/A" || eta_str == "Unknown" {
            None
        } else {
            Some(eta_str.to_string())
        };

        return Some(ProgressInfo {
            percent,
            speed,
            eta,
        });
    }

    None
}

/// Build the --progress-template argument string
pub fn progress_template() -> String {
    "download:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_progress() {
        let line = "download:45.2%|2.5MiB/s|00:01:30";
        let info = parse_progress_line(line).unwrap();
        assert_eq!(info.percent, 45.2);
        assert_eq!(info.speed, Some("2.5MiB/s".to_string()));
        assert_eq!(info.eta, Some("00:01:30".to_string()));
    }

    #[test]
    fn test_parse_unknown_speed() {
        let line = "download:12.5%|Unknown|00:05:00";
        let info = parse_progress_line(line).unwrap();
        assert_eq!(info.percent, 12.5);
        assert_eq!(info.speed, None);
        assert_eq!(info.eta, Some("00:05:00".to_string()));
    }

    #[test]
    fn test_parse_na_values() {
        let line = "download:99.9%|N/A|N/A";
        let info = parse_progress_line(line).unwrap();
        assert_eq!(info.percent, 99.9);
        assert_eq!(info.speed, None);
        assert_eq!(info.eta, None);
    }

    #[test]
    fn test_parse_invalid_line() {
        let line = "Some other output from yt-dlp";
        assert!(parse_progress_line(line).is_none());
    }

    #[test]
    fn test_progress_template_format() {
        let template = progress_template();
        assert!(template.contains("progress._percent_str"));
        assert!(template.contains("progress._speed_str"));
        assert!(template.contains("progress._eta_str"));
    }
}
