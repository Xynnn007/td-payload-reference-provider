# td-payload-reference-provider
A simple tool to calculate td-payload's reference value via bzImage

## Usage

Test with the example bzImage
```
cargo run -- -k tests/bzImage --kernel-size 268435456
```

The `kernel-size` parameter here means `KERNEL_SIZE` defined in guest firmware, s.t. [TD-SHIM](https://github.com/confidential-containers/td-shim)

Will get the result
```
5b7aa6572f649714ff00b6a2b9170516a068fd1a0ba72aa8de27574131d454e6396d3bfa1727d9baf421618a942977fa
```

which is from https://github.com/confidential-containers/attestation-service/pull/33/files#diff-1a4e5ad4c3b043c019c00bc3b3072fd6e1e5b03a5ce8c498e1c0acaf697d9d3fR265

> :warning: hex number is not support, please use dec