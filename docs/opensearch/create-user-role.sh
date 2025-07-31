ADMIN_CREDS="admin:admin"
OSEARCH_URL="http://localhost:9200"

# Create role template for users
PUT /_plugins/_security/api/roles/user_access
{
  "cluster_permissions": [
    "cluster:monitor/main",
    "cluster:monitor/state",
    "cluster:monitor/health",
    "cluster:admin/opensearch/ml/predict",
    "indices:admin/resolve/index",
    "indices:admin/mappings/fields/get*",
    "indices:data/read*",
    "indices:data/write/index"
  ],
  "index_permissions": [{
    "index_patterns": ["${user.name}_*"],
    "allowed_actions": [
      "indices:data/read*",
      "indices:data/write*",
      "indices:admin/create",
      "indices:admin/delete",
      "indices:admin/get",
      "indices:admin/mapping/put",
      "indices:admin/refresh*",
      "indices:admin/resolve/index",
      "indices:admin/search*",
      "indices:admin/validate/query",
      "indices:monitor/stats",
      "indices:monitor/settings/get",
      "indices:monitoring/get"
    ]
  }]
}

# Create user 'user1'
PUT /_plugins/_security/api/internalusers/user1
{
  "password": "SecurePass123!",
  "backend_roles": ["individual_role"]
}

# Assign user 'user1' to user_access role
PUT /_plugins/_security/api/rolesmapping/user_access
{
  "users": ["user1"]
}
