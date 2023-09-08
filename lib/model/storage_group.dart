import 'package:keycontainerex/generated/bridge_generated.dart';
import 'package:keycontainerex/utils/debug.dart';

extension StoraGroupExt on StorageGroup {
  void show({int indent = 0}) {
    // Recursively print this
    debug('StorageGroup: $title');
    debug('SubGroup: $subGroup');
    subPassword.forEach((element) {
      element.show();
    });
  }
}

extension StoragePasswordExt on StoragePassword {
  void show() {
    debug('title: $title');
    debug('username: $username');
    debug('password: $password');
  }
}
