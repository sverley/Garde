# Chemins exclus du plancher de validation (§4.4, #20).
#
# Tout ce qui n'est pas exclu impose une validation : un oubli fait jouer plus,
# jamais moins (annexe C).
#
# Un motif est un glob à la manière de `case`. `*` franchit un `/`, donc `src/*`
# couvre tout le sous-arbre.

# Produit — §6, premier membre : ajouter ne demande rien de plus.
src/*
Cargo.toml
Cargo.lock
.gitignore

# Documentation simple — ni catalogue, ni garde (§4.5).
README.md
docs/application.md
docs/plan-de-construction.md
