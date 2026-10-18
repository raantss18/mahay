# Campagnes d'affiches

Règle (reprise de Tontolo) : **une campagne par jour**, liée aux mises à jour et à l'usage.
Chaque affiche porte **une seule idée**, des lignes courtes, l'icône de l'app, et un **QR code
vers la dernière version**, où le seul fichier à télécharger est **Mahay HUB**. Toujours en
français, anglais et malgache ; jamais de prénom ni de donnée personnelle ; pas d'émoji.

## Produire une campagne

1. Créer `press/campaigns/AAAA-MM-JJ/campaign.json` (modèle : celle du 2026-10-18) : un `theme`,
   puis par affiche un `id`, l'`app` (son icône et sa couleur) et, par langue, `kicker`, `title`
   (2 lignes), `sub` (2 lignes), `points` (3 au plus) et `caption` (texte pour les réseaux).
2. `python3 -I press/posters/build_campaign.py AAAA-MM-JJ` → PNG 1080 × 1440 par affiche et par
   langue, `overview-<langue>.png` (planche de contrôle) et `captions.md` (légendes + lien).
3. Vérifier à l'œil la planche malgache (les textes les plus longs) et scanner un QR code
   (`zbarimg`). Les termes malgaches nouveaux vont dans `docs/A_VALIDER.md`.

## Rotation d'idées

- **Mahay HUB** : « une seule app à installer », « tout se met à jour tout seul ».
- **Une app par jour** : ce qu'elle ouvre (« touchez un tableur, choisissez Tabilao »).
- **La nouveauté de la version** (avec l'affiche de version jointe à la release).
- **Confiance** : hors ligne, sans compte, fichiers chez vous, mises à jour signées.
- **Cas d'usage** : devoirs, petite entreprise, association, chantier, école.

## Campagnes faites

| Date       | Thème                                      | Affiches            |
|------------|--------------------------------------------|---------------------|
| 2026-10-18 | Installez Mahay HUB, il installe le reste  | 14 × 3 langues      |
