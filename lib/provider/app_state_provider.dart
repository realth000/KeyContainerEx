import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part '../generated/provider/app_state_provider.freezed.dart';
part '../generated/provider/app_state_provider.g.dart';

@freezed
class State with _$State {
  const factory State({
    required int screenIndex,
  }) = _State;
}

@Riverpod(keepAlive: true)
class AppState extends _$AppState {
  @override
  State build() {
    return State(
      screenIndex: 0,
    );
  }

  void setScreenIndex(int index) {
    state = state.copyWith(screenIndex: index);
  }
}
