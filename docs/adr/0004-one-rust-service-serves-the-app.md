# One Rust service serves the app

Build SvelteKit as a static frontend and serve it alongside the API from Axum, behind the homelab's HTTPS reverse proxy. The app does not need server-side rendering, so one Rust systemd service avoids a separate Node runtime, a second service, and cross-origin configuration. Backend and frontend remain separate development projects; adding server-side rendering later would require revisiting this deployment choice.
