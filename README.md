# Morse

**Morse** is a lightning-fast, lightweight chat application built entirely in Rust.

Designed for speed, simplicity, and privacy, Morse provides completely ephemeral chat rooms where no messages are stored, and everything happens in real-time.

## 🚀 Features

- **Privacy-Focused:** Accounts are completely optional, and does not require your email or any other third-party.
- **Ephemeral Chat Rooms:** All messages only transit through the server and are never stored, making your conversations completely private.
- **Lightweight:** Minimal resource usage without sacrificing functionality.
- **Fast:** Leveraging Rust’s speed and WebSocket real-time capabilities.

## 🛠️ Tech Stack

- **Backend:** Rust with [Warp framework](https://github.com/seanmonstar/warp)
- **Frontend:** Plain HTML/CSS/JavaScript
- **Deployment:** Docker, Nginx
- **Storage:** MySQL (for user accounts), Redis (for chat rooms)

## 🏗️ Installation

### Prerequisites

- [Docker](https://www.docker.com/get-started/)
- A domain name and an `api` subdomain (Can be set in /etc/hosts for testing)
- A TLS/SSL Certificate (Can be self-signed for testing)

1. Clone the repository:
   ```bash
   git clone https://github.com/Backendt/Morse.git
   cd Morse
   ```
2. Edit the environment configuration:
   ```bash
   cp .env.exemple .env
   vim .env
   ```
3. Add your TLS/SSL certificate:
   ```bash
   cp /path/to/your/certificate.pem morse-web/tls/morse.cert
   cp /path/to/your/privkey.pem morse-web/tls/morse.rsa
   ```
4. Run the application:
   ```bash
   docker compose up
   ```
5. Morse will be up and running on port 443 !

## 📚 Usage

1. Create an account or log-in as anonymous.
2. Create a room
3. Share the room link with your friends or colleagues.
4. Start chatting instantly with no traces left behind!

## 🤝 Contributing

Contributions are welcome ! To get started:

1. Fork the repository.
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "My feature"
   ```
4. Push to your branch:
   ```bash
   git push origin feature-name
   ```
5. Open a pull request on GitHub.

## 📜 License

This project is licensed under the GPL-3.0 License. See the [LICENSE](LICENSE) file for details.
