
import 'package:flutter/cupertino.dart';

import '../../avif/avif_image.dart';
import '../util/file_util.dart';

Widget buildAvifList() {
  return ListView.builder(
      itemBuilder: (context, index) {
        if(index == 0) {
          return const Text("this is avif list");
        } else {
          return AvifImage.asset("assets/$index.avif");
        }
      },
      itemCount: imageCount + 1);
}
