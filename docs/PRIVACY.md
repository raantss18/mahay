# Privacy policy / Politique de confidentialité

*Last updated 2026-10-19. The account and sync parts apply from Mahay 0.3.0; until then they
describe what is planned.*

## English

**Mahay** is a free suite of apps that work on your device, offline. The author is
RANIRIAMANJAKA Antsa; questions and requests go to <https://github.com/raantss18/mahay/discussions>.

- **No account is needed.** Every app works without one. There is **no advertising, no analytics
  and no tracking**, and no app sends your documents anywhere on its own.
- **Updates.** Mahay checks for and downloads new versions from GitHub
  (`github.com/raantss18/mahay`). GitHub sees your IP address, as it does for any download. Updates
  are verified (a signed list of checksums) before they are installed.
- **Optional account (sync).** If you choose "Continue with Google" or "Continue with GitHub", Mahay
  uses **Supabase** to sign you in. Supabase keeps your account identifier, the e-mail address and
  the display name your provider shares, and a few small items Mahay stores for you (settings, a list
  of recent files, which apps are installed). These items are **encrypted on your device** before
  they are sent.
- **Your files.** If you turn sync on, Mahay encrypts your files **on your device**, with a
  passphrase that never leaves it, and stores them in **your own** Google Drive, in a folder named
  "Mahay" (Mahay only sees the files it created: the `drive.file` permission), or on a Nextcloud or
  WebDAV server **you** choose. Neither Google, Supabase nor the author can read them. **If you lose
  the passphrase, nobody can recover the files.**
- **Google user data.** Mahay's use and transfer of information received from Google APIs adheres to
  the [Google API Services User Data Policy](https://developers.google.com/terms/api-services-user-data-policy),
  including the Limited Use requirements. Google data is used only to sign you in and to store and
  read the files you sync; it is never sold, shared with others, used for advertising or read by people.
- **Deleting your data.** "Sign out and delete my data" in Mahay HUB removes your items from Supabase
  and the "Mahay" folder it created in Drive. You can also revoke Mahay's access in your Google
  account settings and delete your Supabase account items by asking in the discussions above.
- **Children.** Mahay does not knowingly collect data from anyone, and has no features aimed at children
  that need an account.
- **Changes.** This page is in the repository (`docs/PRIVACY.md`); its history shows every change.

## Français

**Mahay** est une suite d'applications gratuites qui fonctionnent sur votre appareil, sans connexion.
L'auteur est RANIRIAMANJAKA Antsa ; questions et demandes : <https://github.com/raantss18/mahay/discussions>.

- **Aucun compte n'est nécessaire.** Chaque app fonctionne sans. **Pas de publicité, pas de mesure
  d'audience, pas de pistage**, et aucune app n'envoie vos documents ailleurs d'elle-même.
- **Mises à jour.** Mahay vérifie et télécharge les nouvelles versions depuis GitHub
  (`github.com/raantss18/mahay`) ; GitHub voit votre adresse IP, comme pour tout téléchargement. Les mises
  à jour sont vérifiées (liste d'empreintes signée) avant d'être installées.
- **Compte facultatif (synchronisation).** Si vous choisissez « Continuer avec Google » ou « avec GitHub »,
  Mahay utilise **Supabase** pour vous connecter. Supabase conserve votre identifiant de compte, l'adresse
  e-mail et le nom que votre fournisseur partage, et quelques petits éléments que Mahay garde pour vous
  (réglages, liste de fichiers récents, apps installées), **chiffrés sur votre appareil** avant l'envoi.
- **Vos fichiers.** Si vous activez la synchronisation, Mahay les chiffre **sur votre appareil**, avec une
  phrase secrète qui ne le quitte jamais, et les range dans **votre propre** Google Drive, dans un dossier
  « Mahay » (Mahay ne voit que les fichiers qu'il a créés : autorisation `drive.file`), ou sur un serveur
  Nextcloud ou WebDAV **que vous choisissez**. Ni Google, ni Supabase, ni l'auteur ne peuvent les lire.
  **Si vous perdez la phrase secrète, personne ne peut récupérer les fichiers.**
- **Données Google.** L'utilisation et le transfert par Mahay des informations reçues des API Google
  respectent la [Google API Services User Data Policy](https://developers.google.com/terms/api-services-user-data-policy),
  y compris les exigences d'usage limité. Les données Google servent uniquement à vous connecter et à
  enregistrer et lire les fichiers que vous synchronisez ; elles ne sont ni vendues, ni partagées, ni
  utilisées pour de la publicité, ni lues par des personnes.
- **Supprimer vos données.** « Se déconnecter et supprimer mes données » dans Mahay HUB supprime vos éléments
  chez Supabase et le dossier « Mahay » créé dans votre Drive. Vous pouvez aussi retirer l'accès de Mahay dans
  les paramètres de votre compte Google.
- **Modifications.** Cette page est dans le dépôt (`docs/PRIVACY.md`) ; son historique montre chaque changement.
