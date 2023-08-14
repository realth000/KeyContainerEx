import 'package:flex_color_scheme/flex_color_scheme.dart';
import 'package:flutter/material.dart';
import 'package:keycontainerex/utils/platform.dart';

class AppTheme {
  /// Global theme for [ListTile].
  static final listTileTheme = ListTileThemeData(
    shape: RoundedRectangleBorder(
      // Larger enough to ensure material style.
      borderRadius: BorderRadius.circular(27),
    ),
    horizontalTitleGap: 10,
  );

  /// Global theme for [TextField].
  static const inputDecorationTheme = InputDecorationTheme(
    border: OutlineInputBorder(),
  );

  static final sliderTheme = SliderThemeData(
    overlayColor: Colors.transparent,
    // Disable slider overlay on desktop platforms.
    overlayShape: isMobile ? null : SliderComponentShape.noOverlay,
  );

  static final _flexLightBase = FlexThemeData.light(
    fontFamily: isWindows ? 'Microsoft YaHei' : null,
    scheme: FlexScheme.cyanM3,
    surfaceMode: FlexSurfaceMode.highScaffoldLowSurface,
    blendLevel: 7,
    subThemesData: const FlexSubThemesData(
      blendOnLevel: 10,
      blendOnColors: false,
      useM2StyleDividerInM3: true,
      inputDecoratorBorderType: FlexInputBorderType.underline,
      inputDecoratorUnfocusedBorderIsColored: false,
      navigationBarLabelBehavior: NavigationDestinationLabelBehavior.alwaysHide,
      navigationRailLabelType: NavigationRailLabelType.none,
    ),
    visualDensity: FlexColorScheme.comfortablePlatformDensity,
    useMaterial3: true,
    swapLegacyOnMaterial3: true,
  );

  static final _flexDarkBase = FlexThemeData.dark(
    fontFamily: isWindows ? 'Microsoft YaHei' : null,
    scheme: FlexScheme.cyanM3,
    surfaceMode: FlexSurfaceMode.highScaffoldLowSurface,
    blendLevel: 13,
    subThemesData: const FlexSubThemesData(
      blendOnLevel: 20,
      useM2StyleDividerInM3: true,
      inputDecoratorBorderType: FlexInputBorderType.underline,
      inputDecoratorUnfocusedBorderIsColored: false,
      navigationBarLabelBehavior: NavigationDestinationLabelBehavior.alwaysHide,
      navigationRailLabelType: NavigationRailLabelType.none,
    ),
    visualDensity: FlexColorScheme.comfortablePlatformDensity,
    useMaterial3: true,
    swapLegacyOnMaterial3: true,
  );

  static ThemeData flexLight = _flexLightBase.copyWith(
    listTileTheme: listTileTheme,
    inputDecorationTheme: inputDecorationTheme,
    sliderTheme: sliderTheme,
  );

  static ThemeData flexDark = _flexDarkBase.copyWith(
    listTileTheme: listTileTheme,
    inputDecorationTheme: inputDecorationTheme,
    sliderTheme: sliderTheme,
  );
}
