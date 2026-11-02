# Recording API and accounts

Anonymous recording and sharing work without Google configuration. Google sign-in returns 503 until both `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` are configured on the server. Keep the secret in the server environment; never put it in web or desktop build variables.

Create a Google OAuth client of type **Web application**, enable the `openid` and `email` scopes, and register the exact callback URL:

```
http://127.0.0.1:8082/api/auth/google/callback
```

Add your production callback separately. While the consent screen is in Testing, add each test account to the allowed test users. The desktop uses the system browser and the same server client.

Local server settings:

```
PUBLIC_BASE_URL=http://127.0.0.1:8000
API_PUBLIC_BASE_URL=http://127.0.0.1:8082
COOKIE_SECURE=false
SESSION_TTL_DAYS=90
```

`PUBLIC_BASE_URL` is the exact website origin used by credentialed CORS and cookie mutation checks. Use `127.0.0.1` consistently in local URLs. The website's compile-time `TUTORLOLV2_API_URL` must point to this API, including `/api`. In production, use HTTPS, leave `COOKIE_SECURE` enabled (the default), and host website and API on the same registrable domain.

Run the server with `cargo run -p tutorlolv2_server`. Startup applies migrations, including account/session tables. The sign-in flow sends PKCE S256 and validates issuer, audience, expiry and nonce after a direct confidential exchange with Google's token endpoint. `GOOGLE_TOKEN_ENDPOINT_TRUSTED_OVERRIDE` is exclusively for trusted integration-test providers; leave it unset in deployment.

Desktop sign-in claims only previously anonymous recordings on that device. Account history includes claimed games from other devices; signing out unlinks future recordings while preserving previously claimed games. Native credentials stay in Rust and use Windows DPAPI storage. Website credentials use an HttpOnly cookie. Uploads and live controls continue to use device authentication.

Tests use a mock token provider. Real Google sign-in, the configured redirect URI, and Google accepting PKCE must also be checked with an allowed Google account before release.
