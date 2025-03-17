# Étape 1 : Construire l'application Rust
FROM rust:1.85 AS builder

# Définir le dossier de travail
WORKDIR /app

# Copier les fichiers du projet
COPY . .

# Compiler en mode release pour optimiser la performance
RUN cargo build --release

# Étape 2 : Image finale avec seulement le binaire (léger)
FROM debian:bookworm-slim

# Installer les dépendances nécessaires pour exécuter l'application
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Définir le dossier de travail
WORKDIR /app

# Copier uniquement le binaire compilé depuis l'étape de build
COPY --from=builder /app/target/release/statu_hub_server /app/statu_hub_server

# Spécifier le port exposé (facultatif, dépend de l’infra)
EXPOSE 3000

# Commande pour lancer l'application
CMD ["/app/statu_hub_server"]
