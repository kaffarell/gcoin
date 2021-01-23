pub struct Data {
    pub data_type: String,

}


impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\ndata_type: {}\n)", self.data_type)
    }
}