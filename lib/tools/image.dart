import 'dart:io';

import 'package:flutter/material.dart';
import 'package:pt_next/enums.dart';
import 'package:pt_next/models/operateModel.dart';
import 'package:sqflite_common/sqlite_api.dart';

import 'common.dart';
import 'db.dart';

bool validOperate(OperateModel model, BuildContext context) {
  var mainDir = model.mainDir.text;
  if (mainDir.isEmpty) {
    buildSnackBar(context,"文件操作目录不能为空");
    return false;
  }
  var compareDir = model.compareDir.text;
  if (compareDir.isEmpty) {
    buildSnackBar(context,"文件对比目录不能为空");
    return false;
  }
  var operateTypeText = model.operateTypeText.text;
  var operateType = getOperateType(operateTypeText);
  if(operateType != OperateTypes.Delete && operateType != OperateTypes.DeleteReserve) {
    var targetDir = model.targetDir.text;
    if (targetDir.isEmpty) {
      buildSnackBar(context,"操作目标目录不能为空");
      return false;
    }
  }
  return true;
}

void insertOperateDate(mainDir, compareDir) async {
  Database db = await getDb();
  await db.transaction((txn) async {
      insertDate(txn,"source_data", mainDir);
      insertDate(txn,"compare_data", compareDir);
  });
}

void copyFiles(List<FileInfo> fileList,String targetDir){
  if(fileList.isNotEmpty) {
    for(var info in fileList){
      File file = File(info.path);
      file.copySync("$targetDir${Platform.pathSeparator}${info.name}.${info.ext}");
    }
  }
}

void deleteFiles(List<FileInfo> fileList){
  if(fileList.isNotEmpty) {
    for(var info in fileList){
      File file = File(info.path);
      file.deleteSync();
    }
  }
}
void moveFiles(List<FileInfo> fileList,String targetDir){
  if(fileList.isNotEmpty) {
    for(var info in fileList){
      File file = File(info.path);
      file.copySync("$targetDir${Platform.pathSeparator}${info.name}.${info.ext}");
      file.deleteSync();
    }
  }
}