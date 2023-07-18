# Aptos Python SDK Changelog

All notable changes to the Aptos Python SDK will be captured in this file. This changelog is written by hand for now.

**Note:** Until we cut a 1.0.0 release, the Aptos Python SDK does not follow semantic versioning.

## 0.7.0
- **[Breaking Change]**: Removed the `hex` function from `AccountAddress`. Instead of `addr.hex()` use `str(addr)`.
- **[Breaking Change]**: The string representation of `AccountAddress` now conforms to [AIP-40](https://github.com/aptos-foundation/AIPs/blob/main/aips/aip-40.md).

## 0.6.2
- Added custom header "x-aptos-client" to both sync/async RestClient

## 0.6.1
- Updated package manifest.

## 0.6.0
- Add token client.
- Add support for generating account addresses.
- Add support for http2
- Add async client

