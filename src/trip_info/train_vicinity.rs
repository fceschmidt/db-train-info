use super::TrainVicinity;

impl TrainVicinity {
    /// Get the next scheduled stop
    pub fn scheduled_next(&self) -> &String {
        &self.scheduledNext
    }

    /// Get the next actual stop
    pub fn actual_next(&self) -> &String {
        &self.actualNext
    }

    /// Get the last actual stop
    pub fn actual_last(&self) -> &String {
        &self.actualLast
    }

    /// Get the next stop at the point in time of departure from the last stop
    pub fn actual_last_started(&self) -> &String {
        &self.actualLastStarted
    }
}
