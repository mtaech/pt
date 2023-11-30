use diesel::{Insertable, Queryable, Selectable};
#[derive(Debug)]
pub struct FileInfo{
    pub name:String,
    pub path:String,
    pub ext:Option<String>
}

#[derive(Debug,Queryable, Selectable,Insertable)]
#[diesel(table_name = source_data)]
pub struct SourceData{
    pub name:String,
    pub path:String,
    pub ext:Option<String>
}


#[derive(Debug,Queryable, Selectable)]
#[diesel(table_name = target_data)]
pub struct TargetData{
    pub name:String,
    pub path:String,
    pub ext:Option<String>
}