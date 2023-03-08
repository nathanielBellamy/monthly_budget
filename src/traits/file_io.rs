pub trait FileIO<T> {
    fn path(&self) -> String;

    fn path_in(&self) -> String {
        format!("{}/{}", self.path(), "init")
    }

    fn path_out(&self) -> String {
        format!("{}/{}", self.path(), "reports")
    }

    fn path_events(&self) -> String {
        format!("{}/{}", self.path(), "events")
    }
}
