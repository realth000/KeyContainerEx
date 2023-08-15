import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:keycontainerex/bridge_generated.dart';
import 'package:keycontainerex/ffi.dart';
import 'package:keycontainerex/router.dart';

class WelcomePage extends ConsumerWidget {
  const WelcomePage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final data = api.storageDefaultSavePath(storageFormat: StorageFormat.Kdbx4);

    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          'Welcome to KeyContainerEx',
          style: TextStyle(fontSize: 24),
        ),
        const SizedBox(
          width: 20,
          height: 20,
        ),
        SizedBox(
          width: 200,
          child: ElevatedButton(
            onPressed: () {
              context.go(ScreenPaths.home);
            },
            child: Text('Start'),
          ),
        ),
      ],
    );
  }
}
