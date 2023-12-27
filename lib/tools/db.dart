import 'dart:io';

import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;
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
  var path = Directory.current.path;
  String dbPath = p.join(path, "pt.db");
  var file  = File(dbPath);
  if(file.existsSync()) {
    file.deleteSync();
  }
  var db = await getDb();
  var initSql = await rootBundle.loadString("assets/sql/init.sql");
  await db.execute(initSql);
}