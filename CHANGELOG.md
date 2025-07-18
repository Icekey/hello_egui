# hello_egui changelog

## 0.9.0

- update all crates for egui 0.32 support
- many improvements to egui_flex:
  - Fix shrinking items not growing other items in cross size
  - Add `FlexItem` disabled/enabled control methods
  - Simplify egui_flex based on new egui atoms and intrinsic size
  - Fix frame_time debug assert with bevy_egui
- improvements to egui_dnd:
  - Fix current layer transform not applied to dragged item
  - Make `dnd` functions work on any DragDropItem
- improvements to egui_animation:
  - Make animation time editable in egui_animation::Collapse
- improvements to egui_virtual_list:
  - Ensure stable ids in virtual list and properly handle resized rows
- improvements to egui_infinite_scroll:
  - Don't stop on empty list, only stop on empty cursor
- improvements to egui_router:
  - Add history and history_len to egui_router
- Add hello_egui_utils_dev to remove optional eframe dependency from hello_egui

## 0.8.0

- update all crates for egui 0.31 support (thanks to @bircni)

## 0.7.0

- many improvements to egui_flex
- update all crates for egui 0.30 support

## 0.6.0

- update all crates for egui 0.29 support
- introducing [egui_flex](https://crates.io/crates/egui_flex), a subset of flexbox for egui
- introducing [egui_material_icons](https://crates.io/crates/egui_material_icons), material icons for egui

## 0.5.0

- update all crates for egui 0.28 support
- introduce [egui_router](https://crates.io/crates/egui_router), a spa router for egui
- some smaller changes, view individual crate changelogs for more details

## 0.4.2

- fix missing docs

## 0.4.1

- add `egui_form` [crate](https://crates.io/crates/egui_form)

## 0.4.0

- update all crates for egui 0.27 support
- changes to egui_inbox [(changelog)](./crates/egui_inbox/CHANGELOG.md)

## 0.3.0

- update all crates for egui 0.26 support

## 0.2.0

- Removed all crates from the default features, and instead added a new feature `all` that includes all crates
- Updated egui to 0.25, dropping support for older versions
- Added new crates:
    - egui_thumbhash
    - egui_infinite_scroll
    - egui_suspense
