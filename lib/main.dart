
import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:provider/provider.dart';
import 'package:pt_next/models/operateModel.dart';
import 'package:pt_next/tools/db.dart';

import 'views/operate.dart';

void main() {
  initDb();
  runApp(ChangeNotifierProvider(
    create: (context) => OperateModel(),
    child: const MyApp(),
  ));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'PT',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.cyan),
        useMaterial3: true,
        fontFamily: "WenKaiScreen",
      ),
      home: const OperateView(title: '文件操作'),
      builder:FlutterSmartDialog.init()
    );
  }
}
