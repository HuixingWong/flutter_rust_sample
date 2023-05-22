import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../../avif/avif_encoder.dart';
import '../util/file_util.dart';
import '../util/log.dart';
import '../util/measure.dart';

Widget buildButton1() {
  return IconButton(
    icon: const Icon(Icons.repeat_outlined),
    tooltip: 'Encode Demo',
    onPressed: () async {
      measureAsyncBlockTime(() => rootBundle.load("assets/vettel.gif"));

      measureAsyncBlockTime(() => rootBundle.load("assets/keyboard.png"));

      measureAsyncBlockTime(() => rootBundle.load("assets/butterfly.avif"));

      final bytes = await rootBundle.load("assets/vettel.gif");

      measureAsyncBlockTime(() => encodeAvif(bytes.buffer.asUint8List()));
    },
  );
}

Widget buildButton2() {
  return IconButton(
    icon: const Icon(Icons.repeat_outlined),
    tooltip: 'Encode Demo 2',
    onPressed: () async {
      final start = DateTime.now().millisecond;
      final bytes = await rootBundle.load("assets/keyboard.png");
      final duration = DateTime.now().millisecond - start;
      print("fucking duration: $duration");
      final avifBytes = await encodeAvif(bytes.buffer.asUint8List());
      final path = Directory.systemTemp.path;
      final file = File("$path/image.avif");
      await file.create();
      print(await file.exists());
      file.writeAsBytes(avifBytes);
    },
  );
}
//flutter_rust_bridge_codegen --rust-input /Users/hx/Desktop/code/personal/flutter_avif/rust/src/api.rs \
//                             --dart-output /Users/hx/.pub-cache/hosted/pub.dartlang.org/flutter_avif_platform_interface-1.1.0/lib/bridge_generated.dart

Widget buildButton3({
   Function? start,
   Function? end,
}) {
  return Padding(
    padding: const EdgeInsets.all(16.0),
    child: IconButton(
      onPressed: () async {
        start?.call();
        convertFile().then((value) {
          end?.call();
          loging("convert file finished");
        });
      },
      icon: const Icon(Icons.file_download),
    ),
  );
}
