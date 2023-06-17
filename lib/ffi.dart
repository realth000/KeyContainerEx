import 'bridge_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:io';

const _base = 'keycontainerex_backend';
final path = Platform.isWindows ? '$_base.dll' : 'lib$_base.so';
late final dylib = loadLibForFlutter(path);
late final api = KeycontainerexBackendImpl(dylib);
