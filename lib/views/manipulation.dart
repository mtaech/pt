import 'dart:developer' as developer;
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';

import '../enums.dart';

class ManipulationView extends StatefulWidget {
  const ManipulationView({super.key, required this.title});

  final String title;

  @override
  State<ManipulationView> createState() => _ManipulationView();
}

class _ManipulationView extends State<ManipulationView> {
  final TextEditingController mainDirController = TextEditingController();
  final TextEditingController mainTypeController = TextEditingController();
  final TextEditingController compareDirController = TextEditingController();
  final TextEditingController compareTypeController = TextEditingController();
  final TextEditingController targetDirController = TextEditingController();
  final TextEditingController manipulationTypeController =
      TextEditingController();

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
                        controller: mainDirController, labelText: "文件操作目录"),
                    const SizedBox(width: 20),
                    FileTypeWidget(
                        controller: mainTypeController, labelText: "文件操作类型")
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
                        controller: compareDirController, labelText: "文件对比目录"),
                    const SizedBox(width: 20),
                    FileTypeWidget(
                      controller: mainTypeController,
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
                        controller: targetDirController, labelText: "操作目标目录"),
                    const SizedBox(width: 20),
                    FileOperateWidget(
                        controller: manipulationTypeController,
                        labelText: "文件操作类型"),
                  ],
                ),
              ),
              Container(
                margin: const EdgeInsets.only(bottom: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: Stack(
                        children: <Widget>[
                          Positioned.fill(
                            child: Container(
                              decoration: const BoxDecoration(
                                gradient: LinearGradient(
                                  colors: <Color>[
                                    Color(0xFF1976D2),
                                    Color(0xFF1976D2),
                                    Color(0xFF1976D2),
                                  ],
                                ),
                              ),
                            ),
                          ),
                          TextButton(
                            style: TextButton.styleFrom(
                              foregroundColor: Colors.white,
                              padding: const EdgeInsets.all(16.0),
                            ),
                            onPressed: () {},
                            child: const Text('执行操作'),
                          ),
                        ],
                      ),
                    ),
                  ],
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
    developer.log("log type", name: type);
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
    return DropdownMenu<FileOperates>(
      label: Text(widget.labelText),
      initialSelection: FileOperates.Copy,
      dropdownMenuEntries: FileOperates.values
          .map<DropdownMenuEntry<FileOperates>>((FileOperates type) {
        return DropdownMenuEntry<FileOperates>(
          value: type,
          label: type.label,
        );
      }).toList(),
    );
  }
}
