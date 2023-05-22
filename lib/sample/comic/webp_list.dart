import 'package:flutter/cupertino.dart';

import '../util/file_util.dart';

Widget buildWebpList() {
  return ListView.builder(
      itemBuilder: (context, index) {
        if(index == 0) {
          return const Text("this webp list");
        } else {
          return Image.asset("assets/$index.webp", key: ValueKey(index),);
        }
      },
      itemCount: imageCount + 1);
}
