# KeystonCloud Node

## Development Setup
To set up a development environment for KeystonCloud, we use this simple files structure:
```
keystone-cloud/
 ├── node/
 │    ├── Dockerfile.dev
 │    ├── start.sh
 │    ├── ...
 ├── satellite/
 ├── webapp/
 ├── docker-compose.yml
```

### Define Compose file
If you want to use docker compose for development, you can add into ``services`` part all needed services for node. This is a simple example of configuration:
```yaml
  node:
    build:
      context: ./node
      dockerfile: Dockerfile.dev
    restart: unless-stopped
    environment:
      KC__SATELLITE__PEER_HOST: satellite # Satellit peer host for start.sh
      KC__SATELLITE__PEER_ID: <your_satellite_peer_id>  # Satellite peer ID
      KC__NODE__OWNER_ID: <your_node_owner_id>  # Team ID to own this node
    volumes:
      - ./node:/app
      - ipfs-node-data:/root/.ipfs
      - node-data:/app/data
    deploy:
      replicas: 1
```

Add volumes storage for IPFS and node datas in ``volumes`` part of your docker compose file:
```yaml
  ipfs-node-data:
  node-data:
```

This stack will create the node service. The node service will be built using the `Dockerfile.dev` file located in the `node` folder and use starting script `start.sh`.
This starting script will run the application by using `cargo watch` to automatically reload the application when code changes are detected.
