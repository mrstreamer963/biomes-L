## 1. Reactive State

- [x] 1.1 Add `hoveredCell: Ref<{ col: number; row: number } | null>` to `GridSnapshotState` interface in `useGridSnapshot.ts`
- [x] 1.2 Create reactive `hoveredCell` ref initialized to `null` in `createGridSnapshot()`
- [x] 1.3 Include `hoveredCell` in the returned `instance` object

## 2. MapCamera Pointer Extensions

- [x] 2.1 Add `onPointerMoveCb: ((screenX: number, screenY: number) => void) | null` field to `MapCamera`
- [x] 2.2 Add `onPointerLeaveCb: (() => void) | null` field to `MapCamera`
- [x] 2.3 Add public `onPointerMove(cb)` and `onPointerLeave(cb)` methods
- [x] 2.4 Call `onPointerMoveCb(e.clientX - rect.left, e.clientY - rect.top)` in `handlePointerMove` (branch: always, not just when panning)
- [x] 2.5 Call `onPointerLeaveCb?.()` in `handlePointerLeave`

## 3. GameCanvas Integration

- [x] 3.1 Create handler that converts screen coords → world coords → tile coords (col = floor(worldX / 32), row = floor(worldY / 32))
- [x] 3.2 Validate bounds: if col/row are outside [0, width-1] / [0, height-1], set `hoveredCell` to null
- [x] 3.3 Subscribe camera.onPointerMove and camera.onPointerLeave after camera creation
- [x] 3.4 Import `hoveredCell` from `useGridSnapshot()`

## 4. StatusPanel Display

- [x] 4.1 Extract `hoveredCell` from `useGridSnapshot()` in `StatusPanel.vue`
- [x] 4.2 Add "Cell:" display block in the template showing `hoveredCell.col, hoveredCell.row` when not null
- [x] 4.3 Show placeholder "—" when `hoveredCell` is null
