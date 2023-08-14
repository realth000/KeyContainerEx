import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:keycontainerex/router.dart';
import 'package:keycontainerex/theme.dart';
import 'package:responsive_framework/breakpoint.dart';
import 'package:responsive_framework/responsive_breakpoints.dart';

void main() {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) => ProviderScope(
        child: ResponsiveBreakpoints.builder(
          child: MaterialApp.router(
            title: 'KeyContainerEx',
            theme: AppTheme.flexLight,
            darkTheme: AppTheme.flexDark,
            routerConfig: appRoute,
            builder: (context, child) => Scaffold(body: child),
          ),
          breakpoints: [
            const Breakpoint(start: 0, end: 450, name: MOBILE),
            const Breakpoint(start: 451, end: 800, name: TABLET),
            const Breakpoint(start: 801, end: 1920, name: DESKTOP),
            const Breakpoint(start: 900, end: 900, name: 'EXPAND_SIDE_PANEL'),
            const Breakpoint(start: 1921, end: double.infinity, name: '4k'),
          ],
        ),
      );
}
