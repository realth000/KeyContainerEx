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
    return FutureBuilder(
      future:
          api.storageCheckInit(storageFormat: StorageFormat.Kdbx4, path: ''),
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return Text('failed to check init: ${snapshot.error}');
        } else if (snapshot.connectionState != ConnectionState.done) {
          return const CircularProgressIndicator();
        }
        final init = snapshot.data!;
        if (init) {
          return _InitPage();
        } else {
          return _InitPage();
        }
      },
    );
  }
}

class _InitPage extends ConsumerWidget {
  final formKey = GlobalKey<FormState>();
  final masterKeyController = TextEditingController();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Center(
          child: SizedBox(
            width: 500,
            height: 400,
            child: Form(
              key: formKey,
              autovalidateMode: AutovalidateMode.onUserInteraction,
              child: Padding(
                padding: const EdgeInsets.all(20.0),
                child: Column(
                  children: [
                    const Text(
                      'Welcome to KeyContainerEx',
                      style: TextStyle(
                        fontSize: 24,
                      ),
                    ),
                    const SizedBox(
                      width: 20,
                      height: 20,
                    ),
                    const Text(
                      'Setup Database:',
                      style: TextStyle(
                        fontSize: 16,
                      ),
                    ),
                    const SizedBox(
                      width: 20,
                      height: 20,
                    ),
                    TextFormField(
                      autofocus: true,
                      controller: masterKeyController,
                      obscureText: true,
                      decoration: const InputDecoration(
                        labelText: 'Master Key',
                      ),
                      validator: (v) => v!.trim().isNotEmpty
                          ? null
                          : 'Master key should no be empty',
                    ),
                    const SizedBox(
                      width: 20,
                      height: 20,
                    ),
                    Row(
                      children: [
                        Expanded(
                          child: ElevatedButton(
                              onPressed: () {
                                if (formKey.currentState == null ||
                                    !(formKey.currentState!).validate()) {
                                  return;
                                }
                                api.storageInit(
                                  storageFormat: StorageFormat.Kdbx4,
                                  path: '',
                                  masterKey: masterKeyController.text,
                                  force: true,
                                );
                                context.go(ScreenPaths.home);
                              },
                              child: const Text('Submit')),
                        ),
                      ],
                    )
                  ],
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
