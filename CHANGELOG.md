# cavalier_contours changelog

All notable changes to the cavalier_contours crate will be documented in this file.

## Unreleased

### Added ⭐
* Added CHANGELOG.md for tracking changes and releases.
* Added `PolylineRef`, `PolylineRefMut`, and `PolylineCreation` traits for sharing methods across
different polyline data views (for example sub views/selections over polylines or direction
inversion).

### Changed 🔧
* All Polyline methods have moved to the appropriate trait (`PolylineRef`, `PolylineRefMut`, or
`PolylineCreation`).

### Removed 🔥
* `Polyline::visit_segments` (use `PolylineRef::iter_segments` instead).
