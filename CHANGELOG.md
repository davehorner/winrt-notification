# Changelog


## [0.5.2](https://github.com/allenbenz/winrt-notification/compare/v0.5.1...v0.5.2) - 2025-03-12

### Added

- feat: add tracing support and update dependencies in Cargo.toml

- Added tracing-subscriber and tracing dependencies for enhanced logging
- Integrated tracing initialization in multiple example files (image.rs,
  reuse_arguments.rs, reuse_toast.rs, simple.rs, without_library.rs)
- Revised related documentation in lib.rs for clarity on tracing usage
- HORNERs Quality checks (fmt,fix,audit) [davehorner](https://github.com/davehorner) 



## 0.5.1
 - Re-export windows::runtime::Result
 - Add support for scenarios

## 0.5.0
 - Updated dependencies
 - Added support for gnu targets without msys

## 0.4.0
 - Updated dependencies

## 0.3.1
 - Allow the crate build with the gnu toolchain with msys [#4][i4]

## 0.3.0
 - Switched to the windows-rs crate, dropped winapi and winrt crate.

 ## 0.2.4
 - Made most enums Copy/Clonable.

## 0.2.3
 - Fixed issue where toasts weren't appearing after a windows update.

## 0.2.2
 - Fixed linking issue introduced with winapi 0.3.4 [#4][i4]

## 0.2.1
 - Implemented from_str on Sound enum.

## 0.2.0
 - Added Windows 8.1 support [#3][i3]
 - Incremented winrt dependency

## 0.1.5

- Re-export winrt::Error [#2][i2].
- Added workaround for AppId issue [#1][i1].

## 0.1.4

- Initial release

[i1]: https://github.com/allenbenz/winrt-notification/issues/1
[i2]: https://github.com/allenbenz/winrt-notification/issues/2
[i3]: https://github.com/allenbenz/winrt-notification/issues/3
[i4]: https://github.com/allenbenz/winrt-notification/issues/4
