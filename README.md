# **🚀 Statu Hub Server**
Un serveur léger en **Rust (Axum) avec Redis** pour suivre les statuts des utilisateurs **(connecté / déconnecté)** en **temps réel** via **WebSockets**.

### **📌 Fonctionnalités**
✅ API HTTP pour mettre à jour et récupérer le statut d'un utilisateur.  
✅ WebSockets pour recevoir les mises à jour en **temps réel**.  
✅ Stockage dans **Redis** pour la persistance des statuts.  

---

## **1️⃣ Prérequis**
### **🔹 Installer Rust et Cargo**
Si Rust n'est pas installé, installe-le avec :
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Vérifie l’installation :
```sh
rustc --version
cargo --version
```

### **🔹 Installer Redis**
Si Redis n'est pas installé :
- **Sur Debian/Ubuntu** :
  ```sh
  sudo apt update && sudo apt install redis-server
  ```
- **Sur macOS** :
  ```sh
  brew install redis
  ```

Vérifie que Redis tourne :
```sh
redis-server --version
```

---

## **2️⃣ Lancer le serveur Rust**
Clone le projet et entre dans le dossier :
```sh
git clone https://github.com/ton-projet/statu_hub_server.git
cd statu_hub_server
```

Installe les dépendances Rust :
```sh
cargo build
```

Lance le serveur :
```sh
cargo run
```

Tu devrais voir :
```
Serveur démarré sur http://127.0.0.1:3000
```

---

## **3️⃣ Tester l'API**
### **🔹 Mettre à jour le statut d'un utilisateur**
```sh
curl -X POST http://127.0.0.1:3000/status \
     -H "Content-Type: application/json" \
     -d '{"user_id": "john", "status": "connecté"}'
```

### **🔹 Récupérer le statut d'un utilisateur**
```sh
curl http://127.0.0.1:3000/status/john
```

---

## **4️⃣ Lancer le client WebSocket avec Deno**
### **🔹 Installer Deno**
Si ce n'est pas encore fait :
```sh
curl -fsSL https://deno.land/x/install/install.sh | sh
```
Ajoute Deno au `PATH` :
```sh
export PATH="$HOME/.deno/bin:$PATH"
```
Vérifie l'installation :
```sh
deno --version
```

### **🔹 Créer le client Deno**
Crée un fichier **`client.ts`** et ajoute ce code :
```ts
const socket = new WebSocket("ws://127.0.0.1:3000/ws");

// Événement d'ouverture
socket.onopen = () => {
    console.log("✅ Connecté au WebSocket !");
};

// Réception des messages
socket.onmessage = (event) => {
    console.log("📢 Mise à jour reçue :", event.data);
};

// Gestion des erreurs
socket.onerror = (error) => {
    console.error("❌ Erreur WebSocket :", error);
};

// Fermeture de la connexion
socket.onclose = () => {
    console.log("🔌 Connexion fermée.");
};
```

Lance le client avec :
```sh
deno run --allow-net client.ts
```

---

## **5️⃣ Tester en temps réel**
1. **Démarre le serveur Rust** : `cargo run`
2. **Lance le client Deno** : `deno run --allow-net client.ts`
3. **Met à jour un statut avec `curl`** :
   ```sh
   curl -X POST http://127.0.0.1:3000/status \
        -H "Content-Type: application/json" \
        -d '{"user_id": "john", "status": "connecté"}'
   ```
4. **Tu devrais voir ceci dans le client Deno** :
   ```
   📢 Mise à jour reçue : {"user_id": "john", "status": "connecté"}
   ```

---

## **6️⃣ Déploiement avec Docker (optionnel)**
### **🔹 Construire l’image**
```sh
docker build -t statu_hub_server .
```
### **🔹 Lancer le conteneur**
Si Redis tourne en local :
```sh
docker run -p 3000:3000 statu_hub_server
```

Si tu veux lancer Redis en même temps :
```sh
docker-compose up --build
```

---

## **🚀 Conclusion**
✅ **Serveur Axum + Redis en Rust**  
✅ **Client WebSocket en Deno**  
✅ **Mise à jour des statuts en temps réel**  

Tu veux ajouter **l’authentification**, un **dashboard**, ou améliorer le système ? Dis-moi ! 🚀🔥