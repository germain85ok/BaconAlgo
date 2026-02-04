# ============================
# BACONALGO - INSTALL SCRIPT
# ============================

Write-Host " Création du projet BaconAlgo..."

# Créer structure principale
mkdir BaconAlgo
cd BaconAlgo

mkdir backend, frontend, compute, ml, infra, docs

# Backend Rust
cd backend
cargo init --bin
cd ..

# Frontend Next.js
cd frontend
npx create-next-app@latest . --ts --no-tailwind --no-eslint --no-src-dir --app
cd ..

# Compute
mkdir compute/cpp
mkdir compute/cuda

# ML
mkdir ml/training
mkdir ml/models
mkdir ml/onnx
mkdir ml/features

# Infra
mkdir infra/docker
mkdir infra/supabase/migrations
mkdir infra/monitoring
mkdir infra/ci

# Docs
mkdir docs

Write-Host " Structure BaconAlgo créée!"
Write-Host " Backend: BaconAlgo/backend"
Write-Host " Frontend: BaconAlgo/frontend"
Write-Host " Compute: BaconAlgo/compute"
Write-Host " ML: BaconAlgo/ml"
Write-Host " Infra: BaconAlgo/infra"
Write-Host " Docs: BaconAlgo/docs"

Write-Host " Installation terminée!"