import 'dart:io';

import 'package:flutter/services.dart';

import '../../avif/avif_encoder.dart';
import 'log.dart';

final String baseFilePath = Directory.systemTemp.path;

final pngPath = "$baseFilePath/png";

final webpPath = "$baseFilePath/webp";

final avifPath = "$baseFilePath/avif";

const imageCount = 16;

Future<void> saveUintToFile(
  Uint8List buffer,
  String filePath,
) async {
  await File(filePath).writeAsBytes(buffer);
}

Future<void> saveUintToAvif(
    Uint8List buffer,
    String fileName,
    ) async {
  print(File("${getAvifDir()}/$fileName"));
  await File("${getAvifDir()}/$fileName").writeAsBytes(buffer);
}

String getAvifDir() {
  final dir = Directory(avifPath);
  if (!dir.existsSync()) {
    dir.createSync();
  }
  return avifPath;
}


Future<void> convertFile() async{
  for(int i = 1; i <= 16; i++){
    final bytes = await rootBundle.load("assets/$i.webp");
    loging("bytessize: ${bytes.lengthInBytes}");
    final avifBytes = await encodeAvif(bytes.buffer.asUint8List());
    loging("avifBytesSize: ${avifBytes.lengthInBytes}");
    await saveUintToAvif(avifBytes, "$i.avif");
    loging("saved $i.avif finish");
  }
}