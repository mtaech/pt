import 'package:flutter/material.dart';
import 'package:pt_next/tools/db.dart';

import 'views/manipulation.dart';

void main()  {
  initDb();
  runApp(const MyApp());
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
      home: const ManipulationView(title: '文件操作'),
    );
  }
}

