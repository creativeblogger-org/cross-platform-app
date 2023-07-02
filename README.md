# cross-platform-app
Application cross-platforme pour Creative Blogger.

# Lancer l'application

Vous devez avoir Rust installé sur votre OS : vous pouvez trouvez les informations [ici](https://www.rust-lang.org/tools/install).

Tout d'abord, exécutez cette commande (laissez-là s'exécuter si vous comptez modifier du code) :
```
tailwindcss -i input.css -o public/tailwind.css --watch
```

## Application desktop
[Installez les dépendances listées ici selon votre OS](https://dioxuslabs.com/docs/0.3/guide/en/getting_started/desktop.html#platform-specific-dependencies)

Exécutez la commande suivante :
```
cargo run
```

## Application web
Exécutez les commandes suivantes :
```
cargo install dioxus-cli
```
```
rustup target add wasm32-unknown-unknown
```
```
dioxus serve
```

## Application mobile
*Non disponible pour le moment*