### Serving Your App

Run the following commands in the root of your project to start developing with the default platform:

```bash
diesel setup
diesel migration run 
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

