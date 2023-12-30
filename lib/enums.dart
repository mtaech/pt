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
enum OperateTypes {
  Copy("复制同名文件","COPY"),
  CopyReserve("复制非同名文件","CopyReserve"),
  Delete("删除同名文件","Delete"),
  DeleteReserve("删除非同名文件","DeleteReserve"),
  Move("移动同名文件","Move"),
  MoveReserve("移动非同名文件","MoveReserve");
  const OperateTypes(this.label,this.value);
  final String label;
  final String value;
}