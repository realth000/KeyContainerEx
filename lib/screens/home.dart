import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:keycontainerex/generated/bridge_generated.dart';
import 'package:keycontainerex/model/storage_group.dart';

class HomePage extends ConsumerStatefulWidget {
  HomePage(GoRouterState state, {super.key}) {
    final stateMap = state.extra as Map<String, dynamic>;
    storageGroup = stateMap['storageGroup'] as StorageGroup;
  }

  late final StorageGroup storageGroup;

  @override
  ConsumerState<HomePage> createState() => _HomePageState();
}

class _HomePageState extends ConsumerState<HomePage> {
  @override
  Widget build(BuildContext context) {
    widget.storageGroup.show();
    return Text('Home page');
  }
}
