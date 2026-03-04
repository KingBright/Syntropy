use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use std::time::Duration;
use tracing::info;

pub struct HeadlessBrowserSandbox {
    browser: Browser,
}

impl HeadlessBrowserSandbox {
    pub fn new() -> Result<Self> {
        info!("Launching headless chrome...");
        let options = LaunchOptions::default_builder()
            .headless(true)
            .sandbox(true) // Sandbox enabled for security
            .build()?;

        let browser = Browser::new(options)?;
        Ok(Self { browser })
    }

    /// Navigates to a URL and extracts the text content
    pub fn fetch_text_content(&self, url: &str) -> Result<String> {
        info!("Navigating to URL: {}", url);
        let tab = self.browser.new_tab()?;

        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;

        // Wait briefly for dynamic content
        std::thread::sleep(Duration::from_millis(500));

        let elem = tab.wait_for_element("body")?;
        let text = elem.get_inner_text()?;

        Ok(text)
    }

    /// Takes a screenshot of the visible screen
    pub fn take_screenshot(&self, url: &str) -> Result<Vec<u8>> {
        let tab = self.browser.new_tab()?;
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;

        let jpeg_data = tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Jpeg,
            None,
            None,
            true,
        )?;

        Ok(jpeg_data)
    }
}
