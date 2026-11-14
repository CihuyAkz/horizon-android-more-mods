# Gateway deployment files

These examples are for an Ubuntu Linux host. Replace `YOUR-GATEWAY-HOST` with a DNS name whose A/AAAA record points to the host. Do not expose port 8788 publicly; Caddy is the HTTPS entry point.

1. Create the service account and directories:

   ```sh
   sudo useradd --system --home /var/lib/starling-gateway --shell /usr/sbin/nologin starling-gateway
   sudo install -d -o root -g starling-gateway -m 0750 /etc/starling-gateway
   sudo install -d -o root -g root -m 0755 /opt/starling-gateway
   ```

2. Clone the application repository at the gateway branch:

   ```sh
   sudo git clone --branch starling https://github.com/CihuyAkz/starling-extras /opt/starling-gateway
   ```

3. Copy `gateway.env.example` to `/etc/starling-gateway/gateway.env`. Set `GATEWAY_PUBLIC_URL` to your HTTPS origin. Keep `BUILD_VM` empty for GitHub-only builds. `BUILD_BACKEND` is an unused local URL in this mode.

4. Create a fine-grained GitHub token restricted to `CihuyAkz/mod-builds-extras` with Actions read/write and Contents read. Install it as a root-owned file:

   ```sh
   sudo install -o root -g root -m 0600 /dev/null /etc/starling-gateway/github-token
   sudoedit /etc/starling-gateway/github-token
   ```

   Paste the token into the editor. Do not put it in `gateway.env`, the APK, or the repository. `LoadCredential` supplies `CREDENTIALS_DIRECTORY` to the service automatically; do not set that variable yourself.

5. Install the service and Caddy configuration:

   ```sh
   sudo install -o root -g root -m 0644 deploy/starling-gateway.service /etc/systemd/system/starling-gateway.service
   sudo install -o root -g root -m 0644 deploy/Caddyfile.example /etc/caddy/Caddyfile
   ```

   Edit the Caddyfile to replace `YOUR-GATEWAY-HOST`, then:

   ```sh
   sudo systemctl daemon-reload
   sudo systemctl enable --now starling-gateway
   sudo systemctl reload caddy
   ```

6. Check `https://YOUR-GATEWAY-HOST/healthz` returns `{"ok": true}`. The important check is that HTTPS reaches the gateway and `GITHUB_VERSIONS=*`, `STATE_DIRECTORY`, `GATEWAY_PUBLIC_URL`, and the token are configured.

Only after this host works, replace the single line in `packaging/android/build-server.txt` on branch `starling` with the real HTTPS origin. Do not commit the placeholder.
