
# This is intended to be copied into application repos and added to
# Dockerfiles.  It should have no dependencies besides Ruby and the vault
# gem.
#
# Expects a Secretfile with the format:
#
#     # Comments are OK.
#     # Format: ENV_VAR_NAME VAULT_PATH:VAULT_KEY
#     GOCD_AGENT_KEY secret/gocd/agent_key:agent_key
#
# If you pass in the environment variables VAULT_ENV, DATABASE_URL and
# DATABASE_ROLE, then this script will modify DATABASE_ROLE (and any
# related variables) to use temporary, role-specific database credentials
# from vault.

