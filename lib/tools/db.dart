import 'dart:developer';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;
import 'package:pt_next/models/operateModel.dart';
import 'package:sqflite_common_ffi/sqflite_ffi.dart';

Future<Database> getDb() async {
  sqfliteFfiInit();

  var databaseFactory = databaseFactoryFfi;
  var path = Directory.current.path;

  String dbPath = p.join(path, "pt.db");
  return await databaseFactory.openDatabase(
    dbPath,
  );
}

Future<void> initDb() async {
  var db = await getDb();
  var initSql = await rootBundle.loadString("assets/sql/init.sql");
  await db.execute(initSql);
}

Future<void> closeDb(Database? db) async {
  if (db != null) {
    await db.close();
  }
}

void insertDate(Transaction trx,String tableName, String dirPath) async {
  await trx.execute("delete from $tableName");
  var dir = Directory(dirPath);
  if (dir.existsSync()) {
    var fileList = dir.listSync();
    var batch = trx.batch();
    for (FileSystemEntity fileEntry in fileList) {
      if (FileSystemEntity.isFileSync(fileEntry.path)) {
        var fileName = p.basenameWithoutExtension(fileEntry.path);
        var ext = p.extension(fileEntry.path).replaceFirst(".", "");
        batch.insert(tableName, <String, Object?>{
          'name': fileName,
          'ext': ext.toLowerCase(),
          'path': fileEntry.path
        });
      }
    }
    batch.commit();
  }
}

FileInfo toFileInfo(Map<String, Object?> map) {
  var fileInfo = FileInfo();
  fileInfo.name = getVal( map["name"]) ;
  fileInfo.path = getVal(map["path"]);
  fileInfo.ext = getVal(map["ext"]);
  fileInfo.size = getVal( map["size"]);
  fileInfo.cameraModel = getVal(map["camera_model"]);
  fileInfo.lenModel = getVal(map["len_model"]);
  fileInfo.focalLength = getVal(map["focal_length"]);
  return fileInfo;
}
String getVal(Object? value){
  if(value == null) {
    return "";
  }
  return value as String;
}

Future<List<FileInfo>> findSame(String mainType, String compareType) async {
  var db = await getDb();
  return await db.transaction((txn)  async{
    var sql = '''select source.name,source.path,source.ext
              from source_data source
              where source.name in (select compare.name from compare_data compare
               where compare.ext = '${compareType.toLowerCase()}')
              and source.ext = '${mainType.toLowerCase()}' ;''';
    log("same sql: $sql");
    var list = await txn.rawQuery(sql);
    List<FileInfo> fileList =
    list.map((map) => toFileInfo(map)).toList();
    return fileList;
  });
}

Future<List<FileInfo>> findNotSame(String mainType, String compareType) async {
  var db = await getDb();
  return await db.transaction((txn)  async {
   var sql = '''select source.name,source.path,source.ext
            from source_data source
            where source.name not in (select compare.name from compare_data compare
             where compare.ext = '${compareType.toLowerCase()}')
            and source.ext = '${mainType.toLowerCase()}' ''';
   log("not same sql: $sql");
   var list = await txn.rawQuery(sql);
   List<FileInfo> fileList =
   list.map((map) => {toFileInfo(map)}).cast<FileInfo>().toList();
   return fileList;
 });
}
