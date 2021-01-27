# vanity

![Version: 0.2.0](https://img.shields.io/badge/Version-0.2.0-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 0.2.0](https://img.shields.io/badge/AppVersion-0.2.0-informational?style=flat-square)

A Helm chart for Kubernetes

## Values

| Key                                        | Type   | Default                             | Description |
| ------------------------------------------ | ------ | ----------------------------------- | ----------- |
| affinity                                   | object | `{}`                                |             |
| autoscaling.enabled                        | bool   | `false`                             |             |
| autoscaling.maxReplicas                    | int    | `100`                               |             |
| autoscaling.minReplicas                    | int    | `1`                                 |             |
| autoscaling.targetCPUUtilizationPercentage | int    | `80`                                |             |
| config.domain                              | string | `"go.ectobit.com"`                  |             |
| config.log-level                           | string | `"info"`                            |             |
| config.packages.clap                       | string | `"https://github.com/ectobit/clap"` |             |
| fullnameOverride                           | string | `""`                                |             |
| image.pullPolicy                           | string | `"IfNotPresent"`                    |             |
| image.repository                           | string | `"quay.io/ectobit/vanity"`          |             |
| image.tag                                  | string | `""`                                |             |
| imagePullSecrets                           | list   | `[]`                                |             |
| ingress.annotations                        | object | `{}`                                |             |
| ingress.enabled                            | bool   | `false`                             |             |
| ingress.hosts[0].host                      | string | `"chart-example.local"`             |             |
| ingress.hosts[0].paths                     | list   | `[]`                                |             |
| ingress.tls                                | list   | `[]`                                |             |
| nameOverride                               | string | `""`                                |             |
| nodeSelector                               | object | `{}`                                |             |
| podAnnotations                             | object | `{}`                                |             |
| podSecurityContext                         | object | `{}`                                |             |
| replicaCount                               | int    | `1`                                 |             |
| resources                                  | object | `{}`                                |             |
| securityContext                            | object | `{}`                                |             |
| service.port                               | int    | `80`                                |             |
| service.type                               | string | `"ClusterIP"`                       |             |
| serviceAccount.annotations                 | object | `{}`                                |             |
| serviceAccount.create                      | bool   | `true`                              |             |
| serviceAccount.name                        | string | `""`                                |             |
| tolerations                                | list   | `[]`                                |             |
