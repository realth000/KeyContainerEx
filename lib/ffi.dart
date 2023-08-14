import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:keycontainerex/bridge_generated.dart';

const _base = 'keycontainerex_backend';
final path = Platform.isWindows ? '$_base.dll' : 'lib$_base.so';
late final dylib = loadLibForFlutter(path);
late final api = KeycontainerexBackendImpl(dylib);
