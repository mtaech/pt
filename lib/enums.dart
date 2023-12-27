enum FileTypes {
  RW2("RW2"),
  CR2("CR2"),
  CR3("CR3"),
  NEF("NEF"),
  ARW("ARW"),
  ORF("ORF"),
  JPG("JPG"),
  PNG("PNG"),
  GIF("GIF"),
  TIFF("TIFF"),
  DNG("DNG");
  const FileTypes(this.label);
  final String label;
}
enum FileOperates {
  Copy("复制同名文件"),
  CopyReserve("复制非同名文件"),
  Delete("删除同名文件"),
  DeleteReserve("删除非同名文件"),
  Move("移动同名文件"),
  MoveReserve("移动非同名文件");
  const FileOperates(this.label);
  final String label;
}