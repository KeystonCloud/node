#!/bin/sh
IPFS_DIR=/root/.ipfs

if [ -z "$KC__SATELLITE__PEER_ID" ]; then
  echo "[Node] ERREUR: La variable d'environnement KC__SATELLITE__PEER_ID n'est pas définie."
  echo "[Node] Veuillez lancer la satellite, copier son PeerID, et l'ajouter en viariable d'environnement."
  exit 1
fi

if [ -z "$KC__SATELLITE__PEER_HOST" ]; then
  echo "[Node] ERREUR: La variable d'environnement KC__SATELLITE__PEER_HOST n'est pas définie."
  echo "[Node] Veuillez lancer la satellite, copier son hostname/ip, et l'ajouter en viariable d'environnement."
  exit 1
fi

if [ ! -f $IPFS_DIR/config ]; then
  echo "[Node] Initialisation d'IPFS..."
  ipfs init

  ipfs config Addresses.API '"/ip4/0.0.0.0/tcp/5001"' --json
  ipfs config Addresses.Swarm '["/ip4/0.0.0.0/tcp/4001", "/ip4/0.0.0.0/udp/4001/quic"]' --json
fi

PEER_ADDR="/dns4/$KC__SATELLITE__PEER_HOST/tcp/$KC__SATELLITE__PEER_PORT/p2p/$KC__SATELLITE__PEER_ID"
echo "[Node] Ajout du satellite au Peering: $PEER_ADDR"
ipfs config --json Peering.Peers -- "[{\"ID\": \"$KC__SATELLITE__PEER_ID\", \"Addrs\": [\"$PEER_ADDR\"]}]"

echo "[Node] Lancement du daemon IPFS..."
ipfs daemon &

sleep 5

echo "[Node] Lancement du service KeystoneCloud Node..."
cargo watch -x run
