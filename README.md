# ics-dm-cli

ics-dm-cli is a cli tool to manage your ics-dm yocto images.

# Features
ics-dm-cli provides commands to inject various configurations into a flash image (wic) build with [meta-ics-dm](). Currently the following configurations options are supported:
- inject wifi configuration via wpa_supplicant into all ics-
- inject [auto enrollment]() configuration
- inject an iotedge gateway identity configuration
- inject an iotedge leaf identity configuration

# Installation
**Ensure ~/bin/ exists and is your $PATH**

```sh
docker run --rm --entrypoint cat icsdm.azurecr.io/ics-dm-cli-backend:latest /install/ics-dm-cli > ~/bin/ics-dm-cli && chmod +x ~/bin/ics-dm-cli
```

# Inject wifi configuration
Adapt either `conf/wpa_supplicant.conf.simple.template` or `conf/wpa_supplicant.conf.template`.
Use `wpa_passphrase` to generate your `psk`. Depending on your host system you may have to install `wpa_supplicant` to be able to use `wpa_passhrase`.

```sh
ics-dm-cli wifi set -c $(pwd)/conf/wpa_supplicant.conf -i /full/path/to/your/wic/image.wic
```

# Prepare devices for a transparent gateway with leaf scenario

This Example is based on the article [Configure an IoT Edge device to act as a transparent gateway](https://docs.microsoft.com/en-us/azure/iot-edge/how-to-create-transparent-gateway?view=iotedge-2020-11). We assume that you use a X.509 CA certificate setup.
## IotEdge Gateway configuration

### Certificates

Follow the article [Create demo certificates to test IoT Edge device features](https://docs.microsoft.com/en-us/azure/iot-edge/how-to-create-test-certificates?view=iotedge-2020-11=).
For the iotedge gateway device, you need the following files:
  - `azure-iot-test-only.root.ca.cert.pem`
  - `iot-edge-ca-<name>-full-chain.cert.pem`
  - `iot-edge-ca-<name>.key.pem`

### Config file
Use the following template as iotedge gateway config file: https://raw.githubusercontent.com/Azure/iotedge/master/edgelet/contrib/config/linux/template.toml.<br>
Uncomment the following lines and adapt the hostname.<br>
Don't adapt file paths!<br>
If you use a device image with enrollment demo, don't configure the provisioning! The configuration gets adapted at enrollment.
```sh
# hostname = "my-device"

# trust_bundle_cert = "file:///var/secrets/trust-bundle.pem"


# [edge_ca]
# cert = "file:///var/secrets/edge-ca.pem"                # file URI
#
# pk = "file:///var/secrets/edge-ca.key.pem"              # file URI, or...
```

### Inject configuration
```sh
ics-dm-cli identity  set-iotedge-gateway-config -c $(pwd)/conf/iotedge_config.toml -i /full/path/to/your/wic/iotedge_image.wic  -r /full/path/to/your/azure-iot-test-only.root.ca.cert.pem -d /full/path/to/your/iot-edge-device-ca-<name>-full-chain.cert.pem -k /full/path/to/your/iot-edge-device-ca-<name>.key.pem
```

## Leaf configuration

### Restriction
We provision iot applications as modules. Currently our leaf device configuration is restricted to SaS authentication. See https://azure.github.io/iot-identity-service/develop-an-agent.html#connecting-your-agent-to-iot-hub for details.

### Certificates
For the leaf device with SaS provisioning you only need the root ca certificate:
  - `azure-iot-test-only.root.ca.cert.pem`

### Config file
Use the following template as leaf config file: https://raw.githubusercontent.com/Azure/iot-identity-service/main/aziotctl/config/unix/template.toml.<br>
Adapt hostname.
Uncomment and adapt the following lines.
```sh
# hostname = "my-device"

# local_gateway_hostname = "my-parent-device"

## Manual provisioning with symmetric key
# [provisioning]
# source = "manual"
# iothub_hostname = "example.azure-devices.net"
# device_id = "my-device"
#
# [provisioning.authentication]
# method = "sas"
#
# device_id_pk = { value = "YXppb3QtaWRlbnRpdHktc2VydmljZXxhemlvdC1pZGU=" }     # inline key (base64), or...
# device_id_pk = { uri = "file:///var/secrets/device-id.key" }                  # file URI, or...
# device_id_pk = { uri = "pkcs11:slot-id=0;object=device%20id?pin-value=1234" } # PKCS#11 URI
```

### Inject configuration
```sh
ics-dm-cli identity set-iotedge-leaf-sas-config -c $(pwd)/conf/iotedge_config.toml -i /full/path/to/your/wic/leaf_image.wic  -r /full/path/to/your/azure-iot-test-only.root.ca.cert.pem
```

# Troubleshooting

## No credential store support
`ics-dm-cli` needs to pull a docker image `icsdm.azurecr.io/ics-dm-cli-backend` as backend for some cli
commands. If you use a docker environment with credential store you have to
pull the image prior to calling `ics-dm-cli` manually. (Note this is not necessary if you installed ´ics-dm-cli´ via: [Installation](#installation))
```sh
docker pull icsdm.azurecr.io/ics-dm-cli-backend:$(ics-dm-cli --version | awk '{print $2}')
```

# License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

Copyright (c) 2021 conplement AG
