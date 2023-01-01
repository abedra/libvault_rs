# Examples

## Setup

Start the local docker container (from the project root):

```sh
cd docker
docker-compose up --build
```

Set `VAULT_ADDR` and `VAULT_TOKEN`:

*nix
```sh
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=vault-plaintext-root-token
```

powershell
```powershell
$env:VAULT_ADDR="http://localhost:8200"
$env:VAULT_TOKEN=vault-plaintext-root-token"
```

Now run the following commands to setup your local vault instance:

```sh
vault auth enable approle
vault policy write example policies/example.hcl
vault write auth/approle/role/client policies="example"
vault secrets enable -version=1 -path=legacy kv
vault kv put legacy/hello foo=bar
vault kv put secret/hello foo=bar
```

Next, get your approle credentials:

```sh
vault read auth/approle/role/client/role-id
vault write -f auth/approle/role/client/secret-id
```

Finally, set the `ROLE_ID` and `SECRET_ID` environment variables:

*nix
```sh
export ROLE_ID=<role id>
export SECRET_ID=<secret id>
```

powershell
```powershell
$env:ROLE_ID=<role id>
$env:SECRET_ID=<secret id>
```

## Running the examples

```sh
cargo run --example main
```
