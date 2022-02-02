### v1.0.1 (2022-01-29)

- **Fixes**:
    - Fix empty gaps returned by `Gaps` iterator for `RangeMap`, and incorrect gaps returned by `Gaps` iterator for `RangeInclusiveMap`.

### v1.0.0 (2022-01-28)

It's time. (No functional change.)


### v0.1.14 (2021-11-16)

- **Features**:
    - Expose nameable types for iterators: `Iterator`, `IntoIterator`, `Gaps` (for each collection type).
- **Changes**:
    - Document overflow behaviour required by implementors of `StepLite` and `StepFns`.


### v0.1.13 (2021-08-25)

- **Features**:
    - Add serde support.


### v0.1.12 (2021-08-23)

- **Features**:
    - Implement more traits for all map and set types: `IntoIter`, `FromIter`, and `Extend`.
- **Changes**:
    - Bump minimum supported Rust version to 1.46.


### v0.1.11 (2021-06-30) "EOFY edition"

- **Features**:
    - Support `no_std` environments.
- **Changes**:
    - Update all dev-dependencies to latest versions.


### v0.1.10 (2021-02-23)

- **Fixes**:
    - Fix performance regression introduced in v0.1.9, which made inserts extremely slow for large maps.


### v0.1.9 (2021-02-23)

- **Fixes**:
    - Fix coalescing of contiguous ranges. In some cases `RangeMap` and `RangeInclusiveMap` would leave two separate contiguous ranges with the same value instead of combining them into one.


### v0.1.8 (2020-11-22)

- **Features**:
    - Implement `Debug` for all map and set types.


### v0.1.7 (2020-09-07)

- **Features**:
    - Add `gaps` method to all map and set types for iterating over sub-ranges of a given outer range that are not covered by any stored range.


### v0.1.6 (2020-07-15)

- **Features**:
    - Add `RangeInclusiveMap` and `RangeInclusiveSet` types for storing closed ranges.
