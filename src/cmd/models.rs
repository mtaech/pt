#[derive(Debug)]
pub struct FileInfo{
    pub name:String,
    pub path:String,
    pub ext:Option<String>
}

#[derive(Debug)]
pub struct SourceData{
    pub name:String,
    pub path:String,
    pub ext:Option<String>
}


#[derive(Debug)]
pub struct TargetData{
    pub name:String,
    pub path:String,
    pub ext:Option<String>
}