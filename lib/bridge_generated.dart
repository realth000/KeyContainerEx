// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.81.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:ffi' as ffi;

abstract class KeycontainerexBackend {
  Future<String> storageDefaultSavePath(
      {required StorageFormat storageFormat, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kStorageDefaultSavePathConstMeta;

  Future<void> storageInit(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      required bool force,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kStorageInitConstMeta;

  Future<StorageGroup> storageShow(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kStorageShowConstMeta;

  Future<void> storageAddGroup(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      required String group,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kStorageAddGroupConstMeta;

  Future<void> storageAddPassword(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      required String group,
      required String title,
      required String username,
      required String password,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kStorageAddPasswordConstMeta;
}

enum StorageFormat {
  Kdbx4,
  Json,
}

class StorageGroup {
  final String title;
  final List<StorageGroup> subGroup;
  final List<StoragePassword> subPassword;

  const StorageGroup({
    required this.title,
    required this.subGroup,
    required this.subPassword,
  });
}

class StoragePassword {
  final String title;
  final String username;
  final String password;

  const StoragePassword({
    required this.title,
    required this.username,
    required this.password,
  });
}

class KeycontainerexBackendImpl implements KeycontainerexBackend {
  final KeycontainerexBackendPlatform _platform;
  factory KeycontainerexBackendImpl(ExternalLibrary dylib) =>
      KeycontainerexBackendImpl.raw(KeycontainerexBackendPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory KeycontainerexBackendImpl.wasm(FutureOr<WasmModule> module) =>
      KeycontainerexBackendImpl(module as ExternalLibrary);
  KeycontainerexBackendImpl.raw(this._platform);
  Future<String> storageDefaultSavePath(
      {required StorageFormat storageFormat, dynamic hint}) {
    var arg0 = api2wire_storage_format(storageFormat);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_storage_default_save_path(port_, arg0),
      parseSuccessData: _wire2api_String,
      constMeta: kStorageDefaultSavePathConstMeta,
      argValues: [storageFormat],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kStorageDefaultSavePathConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "storage_default_save_path",
        argNames: ["storageFormat"],
      );

  Future<void> storageInit(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      required bool force,
      dynamic hint}) {
    var arg0 = api2wire_storage_format(storageFormat);
    var arg1 = _platform.api2wire_String(path);
    var arg2 = _platform.api2wire_String(masterKey);
    var arg3 = force;
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_storage_init(port_, arg0, arg1, arg2, arg3),
      parseSuccessData: _wire2api_unit,
      constMeta: kStorageInitConstMeta,
      argValues: [storageFormat, path, masterKey, force],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kStorageInitConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "storage_init",
        argNames: ["storageFormat", "path", "masterKey", "force"],
      );

  Future<StorageGroup> storageShow(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      dynamic hint}) {
    var arg0 = api2wire_storage_format(storageFormat);
    var arg1 = _platform.api2wire_String(path);
    var arg2 = _platform.api2wire_String(masterKey);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_storage_show(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_storage_group,
      constMeta: kStorageShowConstMeta,
      argValues: [storageFormat, path, masterKey],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kStorageShowConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "storage_show",
        argNames: ["storageFormat", "path", "masterKey"],
      );

  Future<void> storageAddGroup(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      required String group,
      dynamic hint}) {
    var arg0 = api2wire_storage_format(storageFormat);
    var arg1 = _platform.api2wire_String(path);
    var arg2 = _platform.api2wire_String(masterKey);
    var arg3 = _platform.api2wire_String(group);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_storage_add_group(port_, arg0, arg1, arg2, arg3),
      parseSuccessData: _wire2api_unit,
      constMeta: kStorageAddGroupConstMeta,
      argValues: [storageFormat, path, masterKey, group],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kStorageAddGroupConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "storage_add_group",
        argNames: ["storageFormat", "path", "masterKey", "group"],
      );

  Future<void> storageAddPassword(
      {required StorageFormat storageFormat,
      required String path,
      required String masterKey,
      required String group,
      required String title,
      required String username,
      required String password,
      dynamic hint}) {
    var arg0 = api2wire_storage_format(storageFormat);
    var arg1 = _platform.api2wire_String(path);
    var arg2 = _platform.api2wire_String(masterKey);
    var arg3 = _platform.api2wire_String(group);
    var arg4 = _platform.api2wire_String(title);
    var arg5 = _platform.api2wire_String(username);
    var arg6 = _platform.api2wire_String(password);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_storage_add_password(
          port_, arg0, arg1, arg2, arg3, arg4, arg5, arg6),
      parseSuccessData: _wire2api_unit,
      constMeta: kStorageAddPasswordConstMeta,
      argValues: [
        storageFormat,
        path,
        masterKey,
        group,
        title,
        username,
        password
      ],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kStorageAddPasswordConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "storage_add_password",
        argNames: [
          "storageFormat",
          "path",
          "masterKey",
          "group",
          "title",
          "username",
          "password"
        ],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  List<StorageGroup> _wire2api_list_storage_group(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_storage_group).toList();
  }

  List<StoragePassword> _wire2api_list_storage_password(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_storage_password).toList();
  }

  StorageGroup _wire2api_storage_group(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return StorageGroup(
      title: _wire2api_String(arr[0]),
      subGroup: _wire2api_list_storage_group(arr[1]),
      subPassword: _wire2api_list_storage_password(arr[2]),
    );
  }

  StoragePassword _wire2api_storage_password(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return StoragePassword(
      title: _wire2api_String(arr[0]),
      username: _wire2api_String(arr[1]),
      password: _wire2api_String(arr[2]),
    );
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }

  void _wire2api_unit(dynamic raw) {
    return;
  }
}

// Section: api2wire

@protected
bool api2wire_bool(bool raw) {
  return raw;
}

@protected
int api2wire_i32(int raw) {
  return raw;
}

@protected
int api2wire_storage_format(StorageFormat raw) {
  return api2wire_i32(raw.index);
}

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: finalizer

class KeycontainerexBackendPlatform
    extends FlutterRustBridgeBase<KeycontainerexBackendWire> {
  KeycontainerexBackendPlatform(ffi.DynamicLibrary dylib)
      : super(KeycontainerexBackendWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

// Section: api_fill_to_wire
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class KeycontainerexBackendWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  KeycontainerexBackendWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  KeycontainerexBackendWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    int ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject =
      _store_dart_post_cobjectPtr.asFunction<void Function(int)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_storage_default_save_path(
    int port_,
    int storage_format,
  ) {
    return _wire_storage_default_save_path(
      port_,
      storage_format,
    );
  }

  late final _wire_storage_default_save_pathPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_storage_default_save_path');
  late final _wire_storage_default_save_path =
      _wire_storage_default_save_pathPtr.asFunction<void Function(int, int)>();

  void wire_storage_init(
    int port_,
    int storage_format,
    ffi.Pointer<wire_uint_8_list> path,
    ffi.Pointer<wire_uint_8_list> master_key,
    ffi.Pointer<bool> force,
  ) {
    return _wire_storage_init(
      port_,
      storage_format,
      path,
      master_key,
      force,
    );
  }

  late final _wire_storage_initPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Int32,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<bool>)>>('wire_storage_init');
  late final _wire_storage_init = _wire_storage_initPtr.asFunction<
      void Function(int, int, ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>, ffi.Pointer<bool>)>();

  void wire_storage_show(
    int port_,
    int storage_format,
    ffi.Pointer<wire_uint_8_list> path,
    ffi.Pointer<wire_uint_8_list> master_key,
  ) {
    return _wire_storage_show(
      port_,
      storage_format,
      path,
      master_key,
    );
  }

  late final _wire_storage_showPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Int32, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_storage_show');
  late final _wire_storage_show = _wire_storage_showPtr.asFunction<
      void Function(int, int, ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>)>();

  void wire_storage_add_group(
    int port_,
    int storage_format,
    ffi.Pointer<wire_uint_8_list> path,
    ffi.Pointer<wire_uint_8_list> master_key,
    ffi.Pointer<wire_uint_8_list> group,
  ) {
    return _wire_storage_add_group(
      port_,
      storage_format,
      path,
      master_key,
      group,
    );
  }

  late final _wire_storage_add_groupPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Int32,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_storage_add_group');
  late final _wire_storage_add_group = _wire_storage_add_groupPtr.asFunction<
      void Function(int, int, ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  void wire_storage_add_password(
    int port_,
    int storage_format,
    ffi.Pointer<wire_uint_8_list> path,
    ffi.Pointer<wire_uint_8_list> master_key,
    ffi.Pointer<wire_uint_8_list> group,
    ffi.Pointer<wire_uint_8_list> title,
    ffi.Pointer<wire_uint_8_list> username,
    ffi.Pointer<wire_uint_8_list> password,
  ) {
    return _wire_storage_add_password(
      port_,
      storage_format,
      path,
      master_key,
      group,
      title,
      username,
      password,
    );
  }

  late final _wire_storage_add_passwordPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Int32,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_storage_add_password');
  late final _wire_storage_add_password =
      _wire_storage_add_passwordPtr.asFunction<
          void Function(
              int,
              int,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef bool = ffi.NativeFunction<ffi.Int Function(ffi.Pointer<ffi.Int>)>;
