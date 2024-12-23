# I Don't Want To (IDWT)

## Runtime Dependencies

- `/usr/sbin/iptables` for the `block-networking` module
- `/usr/bin/yq` for the `edit` command
- `/usr/bin/gpasswd` for the `revoke-admin` module

## Usage

### How to use in containers

Below is a table describing where the outputs from the `idwt` container should go.

| Output path                      | Recommended Destination          |
|----------------------------------|----------------------------------|
| /out/bin/idwt                    | /usr/bin/idwt                    |

Tags available:

- `git` (for the latest git commit)
- `{COMMIT_SHA}` (eg: 74224c0)
- `{MAJOR}` (eg: 1)
- `{MAJOR}.{MINOR}.{PATCH}` (eg: 1.2.3)
- `latest` (for the latest released version

Below is an example of copying /out/bin/idwt to /usr/bin/idwt with idwt version 1.2.3

```containerfile
COPY --from=ghcr.io/noahdotpy/idwt:1.2.3 /out/bin/idwt /usr/bin/idwt
```
