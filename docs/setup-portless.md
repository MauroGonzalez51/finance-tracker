# Portless Setup

## Steps

1. Install `nss`

```bash
sudo pacman -S nss
```

2. Force `portless` to re-ingress certificates

```bash
sudo portless trust
```

3. Update certificates local database

```bash
sudo trust extract-compat
```

