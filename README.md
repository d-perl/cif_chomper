# CIF_CHOMPER

[![CI](https://github.com/d-perl/cif_chomper/actions/workflows/ci.yaml/badge.svg)](https://github.com/d-perl/cif_chomper/actions/workflows/ci.yaml) [![codecov](https://codecov.io/gh/d-perl/cif_chomper/graph/badge.svg?token=Z5CVZZSE42)](https://codecov.io/gh/d-perl/cif_chomper)

A nom-based CIF parser with (forthcoming) automatic generation of structs from
the CIF dictionary definition.

## Development

To compile locally, you need the `cif_core` library to include the full specification for the CIF format. It is included as a submodule, and can be included when this repository is cloned with:

```console
git clone --recurse-submodules https://github.com/d-perl/cif_chomper
```

or after cloning with:

```console
git submodule update --init
```

You can find the CIF core library at https://github.com/COMCIFS/cif_core/.
