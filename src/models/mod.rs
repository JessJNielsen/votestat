#[derive(Debug)]
pub struct District {
    pub name: String,
    pub link: String
}

#[derive(Debug)]
pub struct SuperDistrict {
    pub name: String,
    pub sub_districts: Vec<District>,
}
