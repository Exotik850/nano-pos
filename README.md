# nano-pos

nano-pos is a small point-of-sale website built with Rust, Dioxus, and Diesel.

## Goals

- Provide a minimal POS system for small businesses.
- Keep the codebase easy to extend with user-created types and custom fields.
- Add a plugin system in the future for more flexible extensions.

The current model includes users, contacts, products, estimates, and invoices.

## Development

```bash
diesel setup
diesel migration run
dx serve --platform web
```

The project is licensed under the [GPLv3 License](LICENSE).