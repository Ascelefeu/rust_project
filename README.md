# Règles du jeu

## Objectif

Deux joueurs s’affrontent avec leurs decks de Gwynt.  
L’objectif est de gagner des manches en ayant plus de points que l’adversaire sur le plateau, jusqu’à obtenir 2 manches gagnées (ou jusqu’à la fin de la troisième manche).

***

## Mise en place

- Chaque joueur saisit son nom au début de la partie.  
- Deux decks sont chargés depuis un fichier CSV (par exemple Northern Realms et Nilfgaard).  
- Les decks sont attribués aléatoirement aux joueurs (un deck par joueur).  
- Chaque joueur pioche 7 cartes de son deck pour constituer sa main de départ.

***

## Types de cartes

Chaque carte possède :

- Un **nom**.  
- Une **puissance** (nombre entier).  
- Une **ligne** de prédilection :  
  - **Mêlée**  
  - **Tir**  
  - **Siège**  
- Un **type**, affiché avec un emoji dans l’UI :

| Type        | Emoji | Effet de base                                                         |
|------------|-------|------------------------------------------------------------------------|
| Unité      | ⚔️    | Se pose sur la ligne indiquée du plateau de son propriétaire          |
| Espion     | 🕵️    | Se pose sur la ligne indiquée du plateau adverse, puis pioche 2 cartes |

> D’autres types (météo, buff, héros, etc.) pourront être ajoutés plus tard, mais ne sont pas encore implémentés.

***

## Plateau et lignes

Chaque joueur a un plateau composé de **trois lignes** :

- **Mêlée**  
- **Tir**  
- **Siège**

Quand une carte est jouée :

- Une **unité** ⚔️ va sur la ligne correspondante du plateau du joueur qui la joue.  
- Un **espion** 🕵️ va sur la ligne correspondante du plateau **adverse** et le joueur qui l’a jouée pioche 2 cartes.

La **puissance totale** d’un joueur est la somme des puissances de toutes les cartes présentes sur ses trois lignes.

***

## Tour de jeu

- Le joueur 1 commence la première manche.  
- Les joueurs jouent ensuite à tour de rôle.  
- À son tour, un joueur peut :

1. **Jouer une carte**  
   - Choisir une carte dans sa main.  
   - La carte est retirée de sa main et placée sur la ligne indiquée du plateau (chez soi pour une unité, chez l’adversaire pour un espion).  
   - Si c’est un espion, le joueur pioche immédiatement 2 cartes (si le deck le permet).

2. **Passer**  
   - Le joueur indique qu’il ne jouera plus de carte pour cette manche.  
   - Son état passe à « passé » et il ne pourra plus jouer tant que la manche en cours n’est pas terminée.

Une fois qu’un joueur a passé, il ne peut plus effectuer d’actions pendant la manche.

***

## Fin de manche

Une manche se termine lorsque :

- **Les deux joueurs ont passé**, ou  
- **Les deux joueurs n’ont plus de cartes en main**.

À la fin de la manche :

1. On calcule la puissance totale de chaque joueur (somme des cartes sur leurs trois lignes).  
2. Le joueur ayant le total le plus élevé **gagne la manche** et ajoute 1 à son compteur de manches gagnées.  
3. En cas d’égalité, aucun joueur ne gagne la manche.

Ensuite :

- Les plateaux des deux joueurs sont vidés (les cartes jouées sont défaussées).  
- Les statuts « passé » sont réinitialisés.  
- Les cartes encore en main sont **conservées** pour les manches suivantes (elles ne sont pas défaussées).

***

## Pioche entre les manches

Pour simuler la gestion des ressources sur plusieurs manches :

- Après la **manche 1** : chaque joueur pioche **2 cartes** de son deck (si possible).  
- Après la **manche 2** : chaque joueur pioche **1 carte** de son deck (si possible).  
- Si un deck ne contient plus assez de cartes, le joueur pioche seulement ce qui reste.

***

## Fin de partie

La partie se termine lorsque :

- Un joueur a gagné **2 manches**, ou  
- La **troisième manche** est terminée.

Le vainqueur est le joueur qui a **le plus de manches gagnées**.  
En cas d’égalité de manches gagnées, la partie se termine sur une **égalité**.

***

## Différences avec le Gwynt original

Cette version est un **prototype simplifié** :

- Les cartes sont limitées à des **unités** et des **espions**.  
- Il n’y a pas encore de météo, de buffs, de héros ni de capacités complexes.  
- Les decks sont définis dans un fichier CSV (nom, puissance, type, ligne) pour faciliter la modification et les tests.