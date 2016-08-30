use super::DelayReason;

impl DelayReason {
    /// The code of the delay reason
    pub fn code(&self) -> &String {
        &self.code
    }

    /// The text associated with this delay reason
    pub fn text(&self) -> &String {
        &self.text
    }
}
