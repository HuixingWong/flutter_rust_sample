import 'dart:io';

import 'package:flutter/cupertino.dart';

import '../../avif/avif_image.dart';

Widget homeBody() {
  return ListView(
    children: [
      AvifImage.asset(
        "assets/vettel.avif",
        // height: 200,
        // fit: BoxFit.contain,
      ),
      Image.asset(
        "assets/vettel.gif",
        // height: 200,
        // fit: BoxFit.contain,
      )
      // AvifImage.network(
      //   "https://ezgif.com/images/format-demo/butterfly.avif",
      //   height: 200,
      //   fit: BoxFit.contain,
      // ),

      // encoderOutput,
      // encoderOutput2,
      // AvifImage.file(
      //   File("${Directory.systemTemp.path}/image.avif"),
      //   height: 200,
      //   fit: BoxFit.contain,
      // ),
    ],
  );
}
