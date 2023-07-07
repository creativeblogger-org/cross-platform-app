# cross-platform-app
Application cross-platforme (desktop et mobile) pour Creative Blogger.

# Sommaire
- [Application desktop](#application-desktop)
- [Application mobile](#application-mobile)
    - [Application Android](#application-android)
    - [Application iOS](#application-ios)

Vous devez avoir Rust installé sur votre OS : vous pouvez trouvez les informations [ici](https://www.rust-lang.org/tools/install).

Tout d'abord, exécutez cette commande (laissez-là s'exécuter si vous comptez modifier du code) :
```
npx tailwindcss -i input.css -o public/tailwind.css --watch
```

# Application desktop
[Installez les dépendances listées ici selon votre OS](https://dioxuslabs.com/docs/0.3/guide/en/getting_started/desktop.html#platform-specific-dependencies)

Exécutez la commande suivante :
```
cargo run
```

# Application mobile
Exécutez la commande suivante :
```
cargo install --git https://github.com/BrainiumLLC/cargo-mobile
```
## Application Android

**Installez Android SDK and NDK**

Pour lancer l'application :
```
cargo android run
```

Pour build l'APK :
```
cargo android build
```

## Application iOS
<span style="color:red; font-weight:bold">Vous devez posséder un Mac</span>

**Installez XCode**

Pour lancer l'application :
```
cargo apple run
```

Pour build l'application :
```
cargo apple build
```