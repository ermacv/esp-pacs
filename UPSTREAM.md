# Upstream tracking

This fork (`oer/main` at `github.com/ermacv/esp-pacs`) carries the PAC fixes
open-esp-radio-rs needs on top of `github.com/esp-rs/esp-pacs` `main`. It is
never rebased: every revision a consumer pinned stays reachable. The
`ermacv/esp-hal` fork (`oer/main`) takes its ESP32-C5 and ESP32-S31 PACs from
here; every other chip still comes from upstream.

Fixes are made in the SVD patches (`<chip>/svd/patches/`), never in generated
code: run `cargo xtask patch <chip>` and `cargo xtask generate <chip>` with a
nightly `rustfmt` (`RUSTUP_TOOLCHAIN=nightly`), and commit the patched SVD
inputs with the regenerated crate.

## Base

- Upstream: `c29264b4753724b272599181c103a7008e8f49c4` ("fill the
  unnecessary reg gap (interrupt status regs) (#515)"), the revision
  `esp-hal` pinned.

## Why the fork differs

Each fix was checked against ESP-IDF `4d59230`.

- **ESP32-S31 GPIO matrix reset values.** The GPIO patch spelled the key
  `reset-value`, which svdtools ignores, so `FUNCn_IN_SEL_CFG` and
  `FUNCn_OUT_SEL_CFG` reset to 0 instead of `0x80` and `0x100`.
- **ESP32-S31 `LP_ANA.FIB_ENABLE`.** The register's one 32-bit field
  `ANA_FIB_ENA` gave its bits no meaning. Bit 1, which `brownout_ll.h`
  names `BROWNOUT_DETECTOR_LL_FIB_ENABLE`, is published as `BOD_MODE1_FIB`:
  clearing it gives the brownout detector's mode-1 control to software, so
  `esp-hal` configures and checks the brownout detector through typed
  accessors instead of a mask. The other bits keep no reviewed meaning and
  are not published.
- **ESP32-S31 `USB_UTMI`.** The UTMI PHY control block at `0x20380000`
  (`soc/usb_utmi_struct.h`) is published, so `esp-hal` configures it through
  typed accessors instead of a raw pointer.
- **Interrupt sources.** Every source of ESP-IDF's `soc/interrupts.h` has a
  vector: 18 were missing on the ESP32-S31 (`SYS_ICM`, the peripheral
  timeout and PMS sources, `MSPI_FLASH`, `MSPI_PSRAM`, the mailboxes and the
  LP sources) and 6 on the ESP32-C5 (the three peripheral timeouts, `SLC0`,
  `SLC1`, `KM`).
- **ESP32-C5 GPIO input matrix.** `FUNCn_IN_SEL_CFG` has 117 entries:
  input signal 116 exists, and ESP-IDF reserves only 117-255.
- **ESP32-C5 `LP_APM0`.** The base SVD put it at `LP_APM`'s address with
  `LP_APM`'s layout. It is at `0x60099800` with one master and `INT_EN`,
  `CLOCK_GATE` and `DATE` at `0xD8`, `0xDC` and `0x7FC` (`lp_apm0_reg.h`).
  `LP_APM` keeps the definition upstream already adds.
- **ESP32-C5 `TEE` and `LP_TEE` clock gates.** `CLOCK_GATE` is at `0xFF8`
  and `0xF8`, not `0x80` and `0x4` (`0x4` is `LP_TEE_EFUSE_CTRL`).

## Upstream changes not taken

None.
