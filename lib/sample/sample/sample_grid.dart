import 'dart:io';

import 'package:flutter/cupertino.dart';

import '../../avif/avif_image.dart';
import '../util/file_util.dart';

final images = [
  "https://raw.githubusercontent.com/link-u/avif-sample-images/master/kimono.avif",
  "https://ezgif.com/images/format-demo/butterfly.avif",
  "https://aomediacodec.github.io/av1-avif/testFiles/Link-U/fox.profile0.8bpc.yuv420.avif",
  "https://raw.githubusercontent.com/link-u/avif-sample-images/master/hato.profile0.8bpc.yuv420.avif",
  // "https://raw.githubusercontent.com/AOMediaCodec/av1-avif/master/testFiles/Netflix/avif/hdr_cosmos01650_cicp9-16-9_yuv420_limited_qp10.avif",
  // "https://github.com/AOMediaCodec/av1-avif/blob/master/testFiles/Netflix/avif/hdr_cosmos01000_cicp9-16-0_lossless.avif",
  // "https://github.com/AOMediaCodec/av1-avif/blob/master/testFiles/Netflix/avif/hdr_cosmos12920_cicp9-16-9_yuv444_full_qp40.avif",
  // "https://github.com/AOMediaCodec/av1-avif/blob/master/testFiles/Netflix/avif/sdr_cosmos12920_cicp1-13-6_yuv444_full_qp40.avif",
];

final comicImages =
    List.generate(imageCount, (index) => File("$avifPath/${index + 1}.avif"));

final imageUrlList = List.generate(1000, (index) {
  final i = index % images.length;
  return images[i];
}).toList();

Widget buildGrid() {
  return GridView.builder(
    itemCount: comicImages.length,
    gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
      crossAxisCount: 3,
      crossAxisSpacing: 10,
      mainAxisSpacing: 10,
    ),
    itemBuilder: (context, index) {
      return AvifImage.file(
        comicImages[index],
        height: 200,
        fit: BoxFit.contain,
      );
    },
  );
}
