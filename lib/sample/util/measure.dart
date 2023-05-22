void measureBlockTime(Function block) {
  final stopwatch = Stopwatch()..start();
  block();
  stopwatch.stop();
  print('Block executed in ${stopwatch.elapsedMilliseconds}ms');
}

Future<void> measureAsyncBlockTime(Future<void> Function() block, { String tag = "fucking:::::: "}) async {
  final stopwatch = Stopwatch()..start();
  await block();
  stopwatch.stop();
  print('$tag ${stopwatch.elapsedMilliseconds}ms');
}