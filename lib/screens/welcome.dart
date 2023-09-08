import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:go_router/go_router.dart';
import 'package:keycontainerex/constants.dart';
import 'package:keycontainerex/ffi.dart';
import 'package:keycontainerex/generated/bridge_generated.dart';
import 'package:keycontainerex/router.dart';
import 'package:keycontainerex/utils/debug.dart';

class WelcomePage extends ConsumerWidget {
  WelcomePage({super.key});

  final formKey = GlobalKey<FormState>();
  final masterKeyController = TextEditingController();

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

        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Center(
              child: SizedBox(
                width: 500,
                height: 400,
                child: _LoginForm(
                  formKey: formKey,
                  init: init,
                  masterKeyController: masterKeyController,
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

class _LoginForm extends ConsumerStatefulWidget {
  const _LoginForm({
    required this.formKey,
    required this.init,
    required this.masterKeyController,
  });

  final GlobalKey<FormState> formKey;
  final bool init;
  final TextEditingController masterKeyController;

  @override
  ConsumerState<_LoginForm> createState() => _LoginFormState();
}

class _LoginFormState extends ConsumerState<_LoginForm> {
  String? showPasswordNotPassMsg;

  @override
  Widget build(BuildContext context) {
    return Form(
      key: widget.formKey,
      autovalidateMode: AutovalidateMode.onUserInteraction,
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            Text(
              widget.init ? 'Welcome back' : 'Welcome to KeyContainerEx',
              style: const TextStyle(
                fontSize: 24,
              ),
            ),
            const SizedBox(
              width: 20,
              height: 20,
            ),
            if (!widget.init)
              const Text(
                'Setup Database:',
                style: TextStyle(
                  fontSize: 16,
                ),
              ),
            if (!widget.init)
              const SizedBox(
                width: 20,
                height: 20,
              ),
            TextFormField(
              autofocus: true,
              controller: widget.masterKeyController,
              obscureText: true,
              decoration: InputDecoration(
                labelText: 'Master Key',
                errorText: showPasswordNotPassMsg,
              ),
              validator: (v) =>
                  v!.trim().isNotEmpty ? null : 'Master key should no be empty',
            ),
            const SizedBox(
              width: 20,
              height: 20,
            ),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: () async {
                      if (widget.formKey.currentState == null ||
                          !(widget.formKey.currentState!).validate()) {
                        return;
                      }

                      // Clear error message.
                      // Here should set message to null, not ''.
                      setState(() {
                        showPasswordNotPassMsg = null;
                      });

                      if (!widget.init) {
                        await api.storageInit(
                          storageFormat: StorageFormat.Kdbx4,
                          path: '',
                          masterKey: widget.masterKeyController.text,
                          force: true,
                        );
                      } else {
                        try {
                          final storageGroups = await api.storageShow(
                            storageFormat: StorageFormat.Kdbx4,
                            path: '',
                            masterKey: widget.masterKeyController.text,
                          );
                          if (context.mounted) {
                            context.go(
                              ScreenPaths.home,
                              extra: <String, dynamic>{
                                'storageGroup': storageGroups
                              },
                            );
                          }
                        } on FfiException catch (e) {
                          if (e.code == errRustResultError) {
                            setState(() {
                              showPasswordNotPassMsg = e.message;
                            });
                          }
                          debug('login failed: ${e.code}, ${e.message}');
                        }
                      }
                    },
                    child: Text(widget.init ? 'Unlock' : 'Submit'),
                  ),
                ),
              ],
            )
          ],
        ),
      ),
    );
  }
}
