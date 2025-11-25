//! Lorem ipsum text content for the demo

pub const SHORT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

pub const MEDIUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

pub const LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Curabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam varius, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris.";

pub const PARAGRAPH_1: &str = "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo.";

pub const PARAGRAPH_2: &str = "Quisque sit amet est et sapien ullamcorper pharetra. Vestibulum erat wisi, condimentum sed, commodo vitae, ornare sit amet, wisi. Aenean fermentum, elit eget tincidunt condimentum, eros ipsum rutrum orci, sagittis tempus lacus enim ac dui. Donec non enim in turpis pulvinar facilisis.";

pub const PARAGRAPH_3: &str = "Morbi in sem quis dui placerat ornare. Pellentesque odio nisi, euismod in, pharetra a, ultricies in, diam. Sed arcu. Cras consequat. Praesent dapibus, neque id cursus faucibus, tortor neque egestas augue, eu vulputate magna eros eu erat.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorem_lengths_ordered() {
        // Test that SHORT < MEDIUM < LONG in length
        assert!(SHORT.len() < MEDIUM.len());
        assert!(MEDIUM.len() < LONG.len());
    }

    #[test]
    fn test_short_starts_with_lorem() {
        assert!(SHORT.starts_with("Lorem ipsum"));
    }

    #[test]
    fn test_paragraphs_have_content() {
        // Test that paragraphs have reasonable length (not just a few chars)
        assert!(PARAGRAPH_1.len() > 100);
        assert!(PARAGRAPH_2.len() > 100);
        assert!(PARAGRAPH_3.len() > 100);
    }
}
