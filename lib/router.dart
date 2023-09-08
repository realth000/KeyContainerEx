import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:keycontainerex/screens/home.dart';
import 'package:keycontainerex/screens/settings.dart';
import 'package:keycontainerex/screens/tools.dart';
import 'package:keycontainerex/screens/welcome.dart';
import 'package:keycontainerex/widgets/app_scaffold.dart';

typedef AppBarActionsBuilder = List<Widget>? Function(
    BuildContext context, WidgetRef ref)?;

class ScreenPaths {
  static const String welcome = '/';
  static const String home = '/home';
  static const String tools = '/tools';
  static const String settings = '/settings';
}

final appRoute = GoRouter(routes: [
  AppRoute(
    path: ScreenPaths.welcome,
    builder: (state) => WelcomePage(),
  ),
  AppRoute(
    path: ScreenPaths.home,
    builder: HomePage.new,
  ),
  AppRoute(
    path: ScreenPaths.tools,
    builder: (state) => const ToolsPage(),
  ),
  AppRoute(
    path: ScreenPaths.settings,
    builder: (state) => const SettingsPage(),
  ),
]);

class AppRoute extends GoRoute {
  AppRoute({
    required super.path,
    required Widget Function(GoRouterState state) builder,
    AppBarActionsBuilder appBarActionsBuilder,
    List<GoRoute> routes = const [],
    String? appBarTitle,
    super.redirect,
  }) : super(
          name: path,
          routes: routes,
          pageBuilder: (context, state) => MaterialPage<void>(
            name: path,
            arguments: state.pathParameters,
            child: _buildScaffold(
              state,
              builder,
              appBarActionsBuilder,
              appBarTitle,
            ),
          ),
        );

  static Widget _buildScaffold(
    GoRouterState state,
    Widget Function(GoRouterState state) builder,
    AppBarActionsBuilder appBarActionsBuilder,
    String? appBarTitle,
  ) {
    if (state.path == ScreenPaths.welcome) {
      return builder(state);
    }
    if (state.extra != null && state.extra is Map<String, dynamic>) {
      final extra = state.extra as Map<String, dynamic>;
      return AppScaffold(
        appBarActionsBuilder: appBarActionsBuilder,
        body: SafeArea(
          child: builder(state),
        ),
        appBarTitle: extra['appBarTitle'] is String
            ? extra['appBarTitle'] as String
            : appBarTitle,
      );
    } else {
      return AppScaffold(
        appBarActionsBuilder: appBarActionsBuilder,
        body: SafeArea(
          child: builder(state),
        ),
        appBarTitle: appBarTitle,
      );
    }
  }
}
