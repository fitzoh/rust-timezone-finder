# Rust Timezone Finder

A library that uses data from [Timezone Boundary Builder](https://github.com/evansiroky/timezone-boundary-builder) to
retrieve time zones at lat/long coordinates.

`SimpleTimezoneFinder` is a very simple/slow implementation that is used as a reference for tests.
It iterates over all time zones until a match is found.
`BucketedTimezoneFinder` precomputes the possible time zones for each lat/lon bucket leading to much faster lookups.


The library currently embeds a very large (130M) geojson file containing all time zone boundaries/ids.
