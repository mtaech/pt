
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

import '../enums.dart';

void buildSnackBar(BuildContext context,String content) {
  var snackBar = SnackBar(
    content: Text(content),
    behavior: SnackBarBehavior.floating,
    action: SnackBarAction(
      label: "了解",
      onPressed: () {
        // Code to execute.
      },
    ),
  );
  ScaffoldMessenger.of(context).showSnackBar(snackBar);
}

FileTypes getFileType(String text) {
  for (var type in FileTypes.values) {
    if (type.label == text) {
      return type;
    }
  }
  return FileTypes.RW2;
}

OperateTypes getOperateType(String text) {
  for (var type in OperateTypes.values) {
    if (type.label == text) {
      return type;
    }
  }
  return OperateTypes.Copy;
}

bool isSameType(OperateTypes operateType){
  var list = [OperateTypes.Copy,OperateTypes.Delete,OperateTypes.Move];
  return list.contains(operateType);
}



