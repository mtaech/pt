import 'dart:ffi';

import 'package:flutter/cupertino.dart';

class OperateModel extends ChangeNotifier {
  TextEditingController mainDir = TextEditingController();
  TextEditingController mainType = TextEditingController();
  TextEditingController compareDir = TextEditingController();
  TextEditingController compareType = TextEditingController();
  TextEditingController targetDir = TextEditingController();
  TextEditingController operateTypeText = TextEditingController();
  String operateTypeVal = "";
}

class FileInfo {
  String name = "";
  String path = "";
  String ext = "";
  String size = "0";
  String cameraModel = "";
  String lenModel = "";
  String focalLength = "";
}
