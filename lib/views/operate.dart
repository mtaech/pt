import 'dart:developer';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:pt_next/models/operateModel.dart';
import 'package:pt_next/tools/common.dart';
import 'package:pt_next/tools/db.dart';

import '../tools/image.dart';
import '../enums.dart';

class OperateView extends StatefulWidget {
  const OperateView({super.key, required this.title});

  final String title;

  @override
  State<OperateView> createState() => _OperateView();
}

class _OperateView extends State<OperateView> {
  final OperateModel operateModel = OperateModel();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Container(
          width: MediaQuery.of(context).size.width,
          padding: const EdgeInsets.all(10),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Container(
                margin: const EdgeInsets.only(bottom: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    PathDirWidget(
                        controller: operateModel.mainDir, labelText: "文件操作目录"),
                    const SizedBox(width: 20),
                    FileTypeWidget(
                        controller: operateModel.mainType, labelText: "文件操作类型")
                  ],
                ),
              ),
              Container(
                margin: const EdgeInsets.only(bottom: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    PathDirWidget(
                        controller: operateModel.compareDir,
                        labelText: "文件对比目录"),
                    const SizedBox(width: 20),
                    FileTypeWidget(
                      controller: operateModel.compareType,
                      labelText: "文件对比类型",
                    ),
                  ],
                ),
              ),
              Container(
                margin: const EdgeInsets.only(bottom: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    PathDirWidget(
                        controller: operateModel.targetDir,
                        labelText: "操作目标目录"),
                    const SizedBox(width: 20),
                    FileOperateWidget(
                        controller: operateModel.operateTypeText,
                        labelText: "文件操作类型"),
                  ],
                ),
              ),
              Container(
                margin: const EdgeInsets.only(bottom: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  mainAxisSize: MainAxisSize.max,
                  children: [ExecButton(operateModel: operateModel)],
                ),
              )
            ],
          )),
    );
  }
}

class PathDirWidget extends StatefulWidget {
  const PathDirWidget(
      {super.key, required this.controller, required this.labelText});

  final TextEditingController controller;
  final String labelText;

  @override
  State<PathDirWidget> createState() => _PathDirField();
}

class _PathDirField extends State<PathDirWidget> {
  Future<void> chooseDir(String type) async {
    String? dirPath = await FilePicker.platform.getDirectoryPath();
    if (dirPath != null) {
      widget.controller.text = dirPath;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Expanded(
        child: TextField(
      controller: widget.controller,
      readOnly: true,
      onTap: () {
        chooseDir("mainDir");
      },
      decoration: InputDecoration(
        border: const OutlineInputBorder(),
        labelText: widget.labelText,
      ),
    ));
  }
}

class FileTypeWidget extends StatefulWidget {
  const FileTypeWidget(
      {super.key, required this.controller, required this.labelText});

  final TextEditingController controller;
  final String labelText;

  @override
  State<FileTypeWidget> createState() => _FileTypeSelectField();
}

class _FileTypeSelectField extends State<FileTypeWidget> {
  @override
  Widget build(BuildContext context) {
    return DropdownMenu<FileTypes>(
      label: Text(widget.labelText),
      controller: widget.controller,
      initialSelection: FileTypes.RW2,
      dropdownMenuEntries:
          FileTypes.values.map<DropdownMenuEntry<FileTypes>>((FileTypes type) {
        return DropdownMenuEntry<FileTypes>(
          value: type,
          label: type.label,
        );
      }).toList(),
    );
  }
}

class FileOperateWidget extends StatefulWidget {
  const FileOperateWidget(
      {super.key, required this.controller, required this.labelText});

  final TextEditingController controller;
  final String labelText;

  @override
  State<FileOperateWidget> createState() => _FileOperateSelectField();
}

class _FileOperateSelectField extends State<FileOperateWidget> {
  @override
  Widget build(BuildContext context) {
    return DropdownMenu<OperateTypes>(
      label: Text(widget.labelText),
      initialSelection: OperateTypes.Copy,
      controller: widget.controller,
      dropdownMenuEntries: OperateTypes.values
          .map<DropdownMenuEntry<OperateTypes>>((OperateTypes type) {
        return DropdownMenuEntry<OperateTypes>(
          value: type,
          label: type.label,
        );
      }).toList(),
    );
  }
}

class ExecButton extends StatefulWidget {
  final OperateModel operateModel;

  const ExecButton({super.key, required this.operateModel});

  @override
  State<StatefulWidget> createState() => _ExecButton();
}

class _ExecButton extends State<ExecButton> {


  @override
  Widget build(BuildContext context) {
    return FilledButton.icon(
      style: FilledButton.styleFrom(
        padding: const EdgeInsets.all(16.0),
      ),
      onPressed: () async {
        SmartDialog.showLoading(msg: "操作执行···");
        var result = validOperate(widget.operateModel, context);
        if (result) {
          insertOperateDate(widget.operateModel.mainDir.text,
              widget.operateModel.compareDir.text);
          execOperate(widget.operateModel);
        }
        await Future.delayed(const Duration(seconds: 1));
        SmartDialog.dismiss();
        buildSnackBar(context, "操作已执行完成");
      },
      icon: const Icon(Icons.done_all) ,
      label: const Text("执行操作"),
    );
  }
}

void execOperate(OperateModel operateModel) async {
  var operateText = operateModel.operateTypeText.text;
  OperateTypes operateType = getOperateType(operateText);
  List<FileInfo> fileList = [];
  if (isSameType(operateType)) {
    fileList = await findSame(
        operateModel.mainType.text, operateModel.compareType.text);
  } else {
    fileList = await findNotSame(
        operateModel.mainType.text, operateModel.compareType.text);
  }
  log("file list size ${fileList.length}");
  if (operateType == OperateTypes.Copy ||
      operateType == OperateTypes.CopyReserve) {
    copyFiles(fileList, operateModel.targetDir.text);
  } else if (operateType == OperateTypes.Delete ||
      operateType == OperateTypes.DeleteReserve) {
    deleteFiles(fileList);
  } else if (operateType == OperateTypes.Move ||
      operateType == OperateTypes.MoveReserve) {
    moveFiles(fileList, operateModel.targetDir.text);
  }
}
