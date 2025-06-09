# Changelog

## [0.10.5](https://github.com/momentohq/momento-proxy/compare/v0.10.5...v0.10.5) (2025-06-09)


### ci

* release as 0.10.5 ([#35](https://github.com/momentohq/momento-proxy/issues/35)) ([df7219b](https://github.com/momentohq/momento-proxy/commit/df7219bd7e2b9fe77f93a7613a7ee9ce5b0035ad))
* use release please for release process ([#18](https://github.com/momentohq/momento-proxy/issues/18)) ([1b84f37](https://github.com/momentohq/momento-proxy/commit/1b84f37e6d992201537773c867da9fabbc866fd5))


### Features

* add metrics for mcache and momento hit/miss latency ([#14](https://github.com/momentohq/momento-proxy/issues/14)) ([d973da8](https://github.com/momentohq/momento-proxy/commit/d973da8069a3417fa3b71718aaf6f1a58abc0de7))
* add resp hash commands to momento proxy ([#12](https://github.com/momentohq/momento-proxy/issues/12)) ([f5e77da](https://github.com/momentohq/momento-proxy/commit/f5e77da10fee70f584571ee3b3b5d1711177115a))
* add total active connection count as proxy metric ([#13](https://github.com/momentohq/momento-proxy/issues/13)) ([7a3ebd1](https://github.com/momentohq/momento-proxy/commit/7a3ebd19480049ee5b266a89aa30e615d67bd406))
* adds badd command ([#486](https://github.com/momentohq/momento-proxy/issues/486)) ([0cd1f29](https://github.com/momentohq/momento-proxy/commit/0cd1f29458effd1dad4a6215a69856dab4567210))
* bump momento SDK 32.0 -&gt; 50.1 ([#142](https://github.com/momentohq/momento-proxy/issues/142)) ([09dd3f0](https://github.com/momentohq/momento-proxy/commit/09dd3f0d7c40663572dfb64d02f68601d54a7e32))
* bumps momento sdk to latest version for proxy ([#7](https://github.com/momentohq/momento-proxy/issues/7)) ([da3dc88](https://github.com/momentohq/momento-proxy/commit/da3dc885ebfb77d63b094c96f45e19462fed1305))
* Implement resp `LPOP` command ([#49](https://github.com/momentohq/momento-proxy/issues/49)) ([50acb75](https://github.com/momentohq/momento-proxy/commit/50acb757dd843af4c6a096efd1807ae1dadbb50b))
* Implement resp `LPUSH` command ([#53](https://github.com/momentohq/momento-proxy/issues/53)) ([572a30d](https://github.com/momentohq/momento-proxy/commit/572a30d8061edf814855b262d9a6faaba4d8021b))
* Implement resp `SISMEMBER` command ([#47](https://github.com/momentohq/momento-proxy/issues/47)) ([39c912a](https://github.com/momentohq/momento-proxy/commit/39c912ae92df14631bb2c1a917e551c0dc8c81a9))
* Implement resp `SMEMBERS` command ([#46](https://github.com/momentohq/momento-proxy/issues/46)) ([6c5b7c7](https://github.com/momentohq/momento-proxy/commit/6c5b7c71ec8f811b1c1da989193c214b92fc1628))
* Implement resp LLEN command ([#38](https://github.com/momentohq/momento-proxy/issues/38)) ([14c9f1a](https://github.com/momentohq/momento-proxy/commit/14c9f1a1e8506a1b4777d5f929ba4d3b2b3736b0))
* Implement resp LRANGE command ([#52](https://github.com/momentohq/momento-proxy/issues/52)) ([5d785c8](https://github.com/momentohq/momento-proxy/commit/5d785c84e985053f108377c6a299cc30ffe3bf3f))
* Implement resp LTRIM command ([#55](https://github.com/momentohq/momento-proxy/issues/55)) ([837eced](https://github.com/momentohq/momento-proxy/commit/837eced1a034c614012060ffb213f56178571b25))
* Implement resp RPOP command ([#50](https://github.com/momentohq/momento-proxy/issues/50)) ([f9f560f](https://github.com/momentohq/momento-proxy/commit/f9f560f85a444e853d9f43c68d5ddf6251281f80))
* Implement resp RPUSH command ([#54](https://github.com/momentohq/momento-proxy/issues/54)) ([fb75411](https://github.com/momentohq/momento-proxy/commit/fb754111dc69907da8fe5ba1876b4c23f6b94b03))
* implement resp SDIFF command ([#43](https://github.com/momentohq/momento-proxy/issues/43)) ([df5a687](https://github.com/momentohq/momento-proxy/commit/df5a687542e159ececec2cd3fd1261e9436c798e))
* Implement resp SINTER command ([#45](https://github.com/momentohq/momento-proxy/issues/45)) ([957f656](https://github.com/momentohq/momento-proxy/commit/957f656e265bb3e0d21a304c5dfe8ae6d4ecfd26))
* Implement resp SREM command ([#37](https://github.com/momentohq/momento-proxy/issues/37)) ([d4db314](https://github.com/momentohq/momento-proxy/commit/d4db3149a6959e2656a1f65f413b963b9ac8f4ac))
* Implement resp SUNION command ([#44](https://github.com/momentohq/momento-proxy/issues/44)) ([fa075ae](https://github.com/momentohq/momento-proxy/commit/fa075ae71cd1ca765def7d33f18a0ba8de3241d9))
* implement write-through caching for mcache ([#16](https://github.com/momentohq/momento-proxy/issues/16)) ([860493a](https://github.com/momentohq/momento-proxy/commit/860493ab7b61df3afa78ba48008fd0cda8e78b64))
* instrument resp proxy with goodmetrics ([#8](https://github.com/momentohq/momento-proxy/issues/8)) ([7685552](https://github.com/momentohq/momento-proxy/commit/76855523d468e7b53cdbd9d07086e5fb260ca61e))
* integrate goodmetrics and instrument memcached ([#4](https://github.com/momentohq/momento-proxy/issues/4)) ([8d5c04f](https://github.com/momentohq/momento-proxy/commit/8d5c04f1535b56787bb9bb43b7333585a3d1b2f1))
* **momento-proxy:** add delete support ([#102](https://github.com/momentohq/momento-proxy/issues/102)) ([09644b9](https://github.com/momentohq/momento-proxy/commit/09644b985b324a122e02ddef89c8e27178284573))
* support for the HINCRBY RESP command in Momento proxy ([#22](https://github.com/momentohq/momento-proxy/issues/22)) ([4f69031](https://github.com/momentohq/momento-proxy/commit/4f69031d20f30f248de0d18121e33111b6ccfce7))
* support sorted set methods ([#2](https://github.com/momentohq/momento-proxy/issues/2)) ([08bb2d9](https://github.com/momentohq/momento-proxy/commit/08bb2d9e8016df8a8617f4810d730e419345c4b3))
* update momento client and improve error messages ([#88](https://github.com/momentohq/momento-proxy/issues/88)) ([640a1f8](https://github.com/momentohq/momento-proxy/commit/640a1f8d1aba0b0c833e9dfa0bd953bf11b35555))


### Bug Fixes

* address clippy warnings ([#17](https://github.com/momentohq/momento-proxy/issues/17)) ([cc72bd8](https://github.com/momentohq/momento-proxy/commit/cc72bd857c39915aeda85faf7b102e121438ae7d))
* do not use env var for config in dockerfile ([#28](https://github.com/momentohq/momento-proxy/issues/28)) ([f86dd44](https://github.com/momentohq/momento-proxy/commit/f86dd443fc48305cd48d8fcdbfefd38dbd847bdc))
* **momento-proxy:** update momento client to latest version ([#98](https://github.com/momentohq/momento-proxy/issues/98)) ([af30567](https://github.com/momentohq/momento-proxy/commit/af30567760a808f4cae886708a3eb4cf9eeccfa3))
* treat memcached request ttl as seconds ([#137](https://github.com/momentohq/momento-proxy/issues/137)) ([5093549](https://github.com/momentohq/momento-proxy/commit/5093549f17f7b926a53350a01c0115fb147693ee))
* Update dependencies and add dockerfile ([5ae6130](https://github.com/momentohq/momento-proxy/commit/5ae61300eca2d780674c55bafbb45d5bfe71adcf))
* Update dependencies and add dockerfile ([fcb2ff2](https://github.com/momentohq/momento-proxy/commit/fcb2ff23118d4b05dad8f2a7ff6e4f4d9cd24bae))
* update the upload artifact action ([#6](https://github.com/momentohq/momento-proxy/issues/6)) ([00ec2ed](https://github.com/momentohq/momento-proxy/commit/00ec2ed0bf513df607e3b3a58d092168a0cf8164))
* Use std::time:Instant ([#10](https://github.com/momentohq/momento-proxy/issues/10)) ([6b398da](https://github.com/momentohq/momento-proxy/commit/6b398da377ccd351bb90ab3637379442c4e37720))
* use taiki-e cross toolchain action ([#33](https://github.com/momentohq/momento-proxy/issues/33)) ([a6e21a7](https://github.com/momentohq/momento-proxy/commit/a6e21a77f504970583cc35dda45dfdf8913559ef))


### Miscellaneous

* add a comment about reusing the Momento client ([#9](https://github.com/momentohq/momento-proxy/issues/9)) ([d0bb923](https://github.com/momentohq/momento-proxy/commit/d0bb923c3e3e6ebe98906bb369b605acecbd10c8))
* add github actions ([#5](https://github.com/momentohq/momento-proxy/issues/5)) ([3a3e127](https://github.com/momentohq/momento-proxy/commit/3a3e12777d9e74f1d6d3ea74476582dac9eb37e0))
* add missing phony targets ([#30](https://github.com/momentohq/momento-proxy/issues/30)) ([f371e5e](https://github.com/momentohq/momento-proxy/commit/f371e5efa5859d2772d5badbd619ecce6a08eece))
* add Redis example to momento-proxy docs ([#477](https://github.com/momentohq/momento-proxy/issues/477)) ([7ebcd39](https://github.com/momentohq/momento-proxy/commit/7ebcd396f7efb1903832567f4d41acdd9b5d9a37))
* clean up makefile ([#25](https://github.com/momentohq/momento-proxy/issues/25)) ([bfe7496](https://github.com/momentohq/momento-proxy/commit/bfe7496f980027407526e7565be1a3e2c8872e79))
* clean up project metadata ([#20](https://github.com/momentohq/momento-proxy/issues/20)) ([b406dc4](https://github.com/momentohq/momento-proxy/commit/b406dc47f2f3ffe7583115f0210368258328b48c))
* enable publishing to Docker hub ([#7](https://github.com/momentohq/momento-proxy/issues/7)) ([555e7ac](https://github.com/momentohq/momento-proxy/commit/555e7ac8bd17b3d3d932e71f22f8a51962b5a340))
* fix clippy warnings ([#48](https://github.com/momentohq/momento-proxy/issues/48)) ([961d909](https://github.com/momentohq/momento-proxy/commit/961d9091d7cbabf3c4a0c9f040c4d83c048a983b))
* fix clippy warnings ([#65](https://github.com/momentohq/momento-proxy/issues/65)) ([85bccf6](https://github.com/momentohq/momento-proxy/commit/85bccf638c75180da2e431bb1d69f51f7ee09e64))
* Implement the LINDEX command ([#36](https://github.com/momentohq/momento-proxy/issues/36)) ([03e94d8](https://github.com/momentohq/momento-proxy/commit/03e94d82d2fe150c874ac3676c1000df0e46fd60))
* log version at startup ([#23](https://github.com/momentohq/momento-proxy/issues/23)) ([855bd1e](https://github.com/momentohq/momento-proxy/commit/855bd1e87ec03c55154c35d18e2f3848df7c71af))
* **main:** release 0.10.1 ([#19](https://github.com/momentohq/momento-proxy/issues/19)) ([f972bde](https://github.com/momentohq/momento-proxy/commit/f972bde1b0f72ae68345280c144efada17f047ad))
* **main:** release 0.10.2 ([#21](https://github.com/momentohq/momento-proxy/issues/21)) ([25e7d8c](https://github.com/momentohq/momento-proxy/commit/25e7d8c4737fbb046903a592025ef1fc96117de6))
* **main:** release 0.10.3 ([#24](https://github.com/momentohq/momento-proxy/issues/24)) ([041c571](https://github.com/momentohq/momento-proxy/commit/041c571b119001573bf1e7a86a52663273254bd5))
* **main:** release 0.10.4 ([#29](https://github.com/momentohq/momento-proxy/issues/29)) ([db14f6c](https://github.com/momentohq/momento-proxy/commit/db14f6c7be7bf90709e8b7742c3f84c78a12875c))
* **main:** release 0.10.5 ([#31](https://github.com/momentohq/momento-proxy/issues/31)) ([d9341bf](https://github.com/momentohq/momento-proxy/commit/d9341bfb4cd3eb5379e68f354a60da44df9b3ec8))
* Modenize momento commands ([#57](https://github.com/momentohq/momento-proxy/issues/57)) ([22e8fe0](https://github.com/momentohq/momento-proxy/commit/22e8fe09d4803381d078dd06970c7d118e9c0908))
* project gitignore and recommended extensions ([#3](https://github.com/momentohq/momento-proxy/issues/3)) ([d7b952d](https://github.com/momentohq/momento-proxy/commit/d7b952d5524e68b7d526c79ab934a55049daefdb))
* refactor momento frontend to reduce code duplication ([#42](https://github.com/momentohq/momento-proxy/issues/42)) ([e79d83a](https://github.com/momentohq/momento-proxy/commit/e79d83ac01425c7c6cf2a48e38d3b76074dc58a3))
* update dependencies ([#19](https://github.com/momentohq/momento-proxy/issues/19)) ([d641c48](https://github.com/momentohq/momento-proxy/commit/d641c486541a9ca016307038235b1fe7dbfa3d71))
* update metriken library ([#100](https://github.com/momentohq/momento-proxy/issues/100)) ([bb55a43](https://github.com/momentohq/momento-proxy/commit/bb55a43559dd6489b1a6bfd5cd85e421a3b0b4f5))
* update momento crate requirement to the latest version ([#35](https://github.com/momentohq/momento-proxy/issues/35)) ([2f6ad17](https://github.com/momentohq/momento-proxy/commit/2f6ad17062fabd9d27eaeef3c9c913940b0ba857))
* update momento crate to v0.17.0 ([#18](https://github.com/momentohq/momento-proxy/issues/18)) ([3a587c9](https://github.com/momentohq/momento-proxy/commit/3a587c9b81cc176291d2f23d1151ca44aaa57e5b))
* Update momento to v0.21.0 ([#30](https://github.com/momentohq/momento-proxy/issues/30)) ([bbe1849](https://github.com/momentohq/momento-proxy/commit/bbe18493ae592255d292afccbe3db7bec94435ef))
* upgrade to latest pelikan for memcached text protocol fix ([#11](https://github.com/momentohq/momento-proxy/issues/11)) ([f6687ea](https://github.com/momentohq/momento-proxy/commit/f6687ea896c1a05af69c1ae38c03d5d0ec0e567f))

## [0.10.5](https://github.com/momentohq/momento-proxy/compare/v0.10.4...v0.10.5) (2025-06-09)


### Miscellaneous

* add missing phony targets ([#30](https://github.com/momentohq/momento-proxy/issues/30)) ([f371e5e](https://github.com/momentohq/momento-proxy/commit/f371e5efa5859d2772d5badbd619ecce6a08eece))

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
