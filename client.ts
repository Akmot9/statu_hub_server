const socket = new WebSocket("ws://127.0.0.1:3000/ws");

// Gérer la connexion WebSocket
socket.onopen = () => {
    console.log("✅ Connecté au serveur WebSocket !");
};

// Réception des messages du serveur
socket.onmessage = (event) => {
    console.log("📢 Mise à jour reçue :", event.data);
};

// Gérer les erreurs
socket.onerror = (error) => {
    console.error("❌ Erreur WebSocket :", error);
};

// Gérer la fermeture de la connexion
socket.onclose = () => {
    console.log("🔌 Connexion fermée.");
};
