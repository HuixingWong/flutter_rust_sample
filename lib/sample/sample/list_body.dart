
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_template/sample/sample/sample_grid.dart';
import 'package:flutter_rust_bridge_template/sample/sample/top_buttons.dart';

import '../comic/avif_list.dart';
import '../comic/webp_list.dart';

class ImageListBody extends StatefulWidget {

  @override
  State<StatefulWidget> createState() {
    return _listBodyState();
  }

}

class _listBodyState extends State<ImageListBody> {

  var showType = 1;
  var isDouble = false;
  var showLoading = false;

  @override
  Widget build(BuildContext context) {
    return  Stack(
      children: [
        Scaffold(
          appBar: AppBar(
            title: const Text('Plugin example app'),
            actions: [
              buildButton3(start: () {
                setState(() {
                  showLoading = true;
                });
              }, end: () {
                setState(() {
                  showLoading = false;
                });
              }),
              IconButton(
                onPressed: () {
                  setState(() {
                    showType += 1;
                  });
                },
                icon: const Icon(Icons.next_plan),
              ),
              IconButton(
                onPressed: () {
                  setState(() {
                    isDouble = !isDouble;
                  });
                },
                icon: const Icon(Icons.double_arrow),
              ),
            ],
          ),
          body: isDouble
              ? buildGrid()
              : showType % 2 == 0
              ? buildWebpList()
              : buildAvifList(),
        ),
        showLoading
            ? Container(
          color: Colors.red,
          width: double.infinity,
          height: double.infinity,
          child: const Center(
            child: CircularProgressIndicator(),
          ),
        )
            : const SizedBox(
          width: 0,
          height: 0,
        ),
      ],
    );
  }

}