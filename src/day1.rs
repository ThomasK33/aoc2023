use std::path::PathBuf;

pub(crate) fn day1(path: PathBuf, b: bool) -> anyhow::Result<()> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Err(anyhow::anyhow!("Failed to read file at path"));
    };

    let mut total: u64 = 0;

    for line in content.lines().filter(|line| !line.is_empty()) {
        log::info!("line: {}", line);

        // TODO(ThomasK33): Implement task B by iterating over the line
        // and replacing substrings going left to right with their digits.

        let digits: Vec<u8> = line
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .map(|c| c as u8)
            .map(|c| (c - 48u8))
            .collect();

        log::info!("digits {:?}", digits);

        let Some(first) = digits.first() else {
            return Err(anyhow::anyhow!("Failed to get first digit"));
        };
        let Some(last) = digits.last() else {
            return Err(anyhow::anyhow!("Failed to get last digit"));
        };

        log::info!("First and last digit {:?} {:?}", first, last);

        let value = first * 10 + last;
        total += value as u64;
    }

    log::info!("Total: {}", total);

    Ok(())
}
