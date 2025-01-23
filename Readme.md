# TP Rust Image

Cette application permet d'appliquer quatre filtres simples à une image. Vous pouvez :
- La convertir en monochrome (noir et blanc).
- La réduire à une palette de couleurs limitée.
- Appliquer un effet de tramage aléatoire.
- Utiliser une matrice de Bayer pour un tramage plus structuré.

## Répartition du travail
Robin Boissay: Réalisation de la partie commande avec argh et du filtre monocrhome et palette
Nathan Boissay: Réalisation des filtres bayer et tramage aléatoire.

## Options disponibles

### 1. Monochrome (Seuillage)

Ce filtre convertit l'image en noir et blanc en fonction d'un seuil de luminosité. Si la luminosité d'un pixel est supérieure au seuil, il devient blanc, sinon il devient noir.

#### Utilisation :
- `--monochrome`
- Optionnel : `--threshold` (par défaut 0.5) : définit le seuil de luminosité pour passer en blanc ou noir.

### 2. Palette

Ce filtre réduit l'image à une palette de 8 couleurs prédéfinies : noir, blanc, rouge, vert, bleu, jaune, magenta, cyan. Chaque pixel de l'image est remplacé par la couleur la plus proche de cette palette.

#### Utilisation :
- `--palette`

### 3. Tramage aléatoire (Dithering)

Ce filtre applique un effet de tramage où chaque pixel est transformé en noir ou en blanc de manière aléatoire. Cela permet de donner une illusion de nuance dans les images avec peu de couleurs.

#### Utilisation :
- `--dithering`

### 4. Tramage par matrice de Bayer

Ce filtre applique un tramage structuré en utilisant une matrice de Bayer d'ordre n. Cette méthode permet de réduire l'effet de tramage aléatoire en appliquant des seuils de manière systématique sur l'ensemble de l'image.

#### Utilisation :
- `--bayer ` 


## Exemple de ligne de commande:
- (Monochrome): cargo run -- --image input.jpg --output output.png --operation monochrome --threshold 0.6
- (Palette): cargo run -- --operation palette --image "input.jpg" --output "output.png"
- (Dithering): cargo run -- --operation dithering --image "input.jpg" --output "output.png"
- (Bayer): cargo run -- --operation bayer --image "input.jpg" --output "output.png"
