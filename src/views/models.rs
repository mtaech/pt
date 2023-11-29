use std::fmt;
use std::fmt::{Display, Formatter};
use strum::EnumIter;

#[derive(Clone, Debug, PartialEq, Default, EnumIter)]
pub enum FileTypes {
    #[default]
    RW2,
    CR2,
    CR3,
    NEF,
    ARW,
    ORF,
    JPG,
    PNG,
    GIF,
    TIFF,
}

impl FileTypes {
    pub fn default() -> Self {
        FileTypes::RW2
    }
}
impl Display for FileTypes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileTypes::RW2 => write!(f, "RW2"),
            FileTypes::CR2 => write!(f, "CR2"),
            FileTypes::CR3 => write!(f, "CR3"),
            FileTypes::NEF => write!(f, "NEF"),
            FileTypes::ARW => write!(f, "ARW"),
            FileTypes::ORF => write!(f, "ORF"),
            FileTypes::JPG => write!(f, "JPG"),
            FileTypes::PNG => write!(f, "PNG"),
            FileTypes::GIF => write!(f, "GIF"),
            FileTypes::TIFF => write!(f, "TIFF"),
        }
    }
}

impl From<FileTypes> for String {
    fn from(file_types: FileTypes) -> Self {
        match file_types {
            FileTypes::RW2 => "RW2".into(),
            FileTypes::CR2 => "CR2".into(),
            FileTypes::CR3 => "CR3".into(),
            FileTypes::NEF => "NEF".into(),
            FileTypes::ARW => "ARW".into(),
            FileTypes::ORF => "ORF".into(),
            FileTypes::JPG => "JPG".into(),
            FileTypes::PNG => "PNG".into(),
            FileTypes::GIF => "GIF".into(),
            FileTypes::TIFF => "TIFF".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, EnumIter)]
pub enum FileOperate {
    #[default]
    Copy,
    CopyReserve,
    Delete,
    DeleteReserve,
    Move,
    MoveReserve,
}

impl Display for FileOperate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileOperate::Copy => write!(f, "复制同名文件"),
            FileOperate::CopyReserve => write!(f, "复制非同名文件"),
            FileOperate::Delete => write!(f, "删除同名文件"),
            FileOperate::DeleteReserve => write!(f, "删除非同名文件"),
            FileOperate::Move => write!(f, "移动同名文件"),
            FileOperate::MoveReserve => write!(f, "移动非同名文件"),
        }
    }
}

impl From<FileOperate> for String {
    fn from(file_operate: FileOperate) -> Self {
        match file_operate {
            FileOperate::Copy => "Copy".into(),
            FileOperate::CopyReserve => "CopyReserve".into(),
            FileOperate::Delete => "Delete".into(),
            FileOperate::DeleteReserve => "DeleteReserve".into(),
            FileOperate::Move => "Move".into(),
            FileOperate::MoveReserve => "MoveReserve".into(),
        }
    }
}
