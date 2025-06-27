# Changelog

## [0.10.11](https://github.com/momentohq/momento-proxy/compare/v0.10.10...v0.10.11) (2025-06-27)


### ci

* release 0.10.11 ([#55](https://github.com/momentohq/momento-proxy/issues/55)) ([70b4a5c](https://github.com/momentohq/momento-proxy/commit/70b4a5c4e45cfb276e5f841befadf2bfb4c702d7))

## [0.10.10](https://github.com/momentohq/momento-proxy/compare/v0.10.9...v0.10.10) (2025-06-21)


### Bug Fixes

* use Ubuntu 22.04 runners because Ubuntu 24.04 built binaries do not run on AL2023 ([#52](https://github.com/momentohq/momento-proxy/issues/52)) ([64222fc](https://github.com/momentohq/momento-proxy/commit/64222fc2e93deed297627577d21a335e5ea40c9c))

## [0.10.9](https://github.com/momentohq/momento-proxy/compare/v0.10.8...v0.10.9) (2025-06-19)


### ci

* release 0.10.9 ([#49](https://github.com/momentohq/momento-proxy/issues/49)) ([bd9ceef](https://github.com/momentohq/momento-proxy/commit/bd9ceef6dcf8cb46b736a6701e2845a29449daef))

## [0.10.8](https://github.com/momentohq/momento-proxy/compare/v0.10.7...v0.10.8) (2025-06-19)


### ci

* Release 0.10.8 ([#46](https://github.com/momentohq/momento-proxy/issues/46)) ([8f99f64](https://github.com/momentohq/momento-proxy/commit/8f99f64432fadff6411523066fc8c837f94c82d5))

## [0.10.7](https://github.com/momentohq/momento-proxy/compare/v0.10.6...v0.10.7) (2025-06-12)


### Miscellaneous

* make buffer sizes configurable for incoming connections ([#43](https://github.com/momentohq/momento-proxy/issues/43)) ([7fc94be](https://github.com/momentohq/momento-proxy/commit/7fc94be34494ef1661b2463e77e469c355f2dc78))

## [0.10.6](https://github.com/momentohq/momento-proxy/compare/v0.10.5...v0.10.6) (2025-06-11)


### Bug Fixes

* **admin:** zero-initialize `rusage` instead of struct literal for musl compatibility ([#41](https://github.com/momentohq/momento-proxy/issues/41)) ([385ff88](https://github.com/momentohq/momento-proxy/commit/385ff88bedfc81c29f75c0e50b0d1d52309092f9))


## [0.10.5](https://github.com/momentohq/momento-proxy/compare/v0.10.4...v0.10.5) (2025-06-09)


### ci

* release as 0.10.5 ([#35](https://github.com/momentohq/momento-proxy/issues/35)) ([df7219b](https://github.com/momentohq/momento-proxy/commit/df7219bd7e2b9fe77f93a7613a7ee9ce5b0035ad))

## [0.10.4](https://github.com/momentohq/momento-proxy/compare/v0.10.3...v0.10.4) (2025-06-06)


### Bug Fixes

* do not use env var for config in dockerfile ([#28](https://github.com/momentohq/momento-proxy/issues/28)) ([f86dd44](https://github.com/momentohq/momento-proxy/commit/f86dd443fc48305cd48d8fcdbfefd38dbd847bdc))

## [0.10.3](https://github.com/momentohq/momento-proxy/compare/v0.10.2...v0.10.3) (2025-06-06)


### Miscellaneous

* clean up makefile ([#25](https://github.com/momentohq/momento-proxy/issues/25)) ([bfe7496](https://github.com/momentohq/momento-proxy/commit/bfe7496f980027407526e7565be1a3e2c8872e79))
* log version at startup ([#23](https://github.com/momentohq/momento-proxy/issues/23)) ([855bd1e](https://github.com/momentohq/momento-proxy/commit/855bd1e87ec03c55154c35d18e2f3848df7c71af))

## [0.10.2](https://github.com/momentohq/momento-proxy/compare/v0.10.1...v0.10.2) (2025-06-06)


### Miscellaneous

* clean up project metadata ([#20](https://github.com/momentohq/momento-proxy/issues/20)) ([b406dc4](https://github.com/momentohq/momento-proxy/commit/b406dc47f2f3ffe7583115f0210368258328b48c))

## [0.10.1](https://github.com/momentohq/momento-proxy/compare/v0.10.0...v0.10.1) (2025-06-05)


### ci

* use release please for release process ([#18](https://github.com/momentohq/momento-proxy/issues/18)) ([1b84f37](https://github.com/momentohq/momento-proxy/commit/1b84f37e6d992201537773c867da9fabbc866fd5))
