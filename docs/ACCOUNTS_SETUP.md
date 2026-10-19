# Créer les comptes pour la synchronisation (0.3.0)

À faire **une seule fois** par le propriétaire du dépôt. Deux services, tous deux gratuits pour
commencer, sur le modèle de Tontolo : **Supabase** (connexion « Continuer avec Google / GitHub » et
petites données) et **Google Cloud** (le Drive de l'utilisateur, avec la portée `drive.file` : Mahay
ne voit que les fichiers qu'il a créés). Conception : [SYNC.md](SYNC.md), décision
[D20](DECISIONS.md).

> **Ne m'envoie jamais un secret dans la conversation.** Les valeurs vont dans `release-keys/`
> (ignoré par git) et dans les secrets GitHub ; je ne lis que leurs noms. Les écrans de ces
> services changent souvent : suis l'intention de chaque étape plutôt que les intitulés exacts.

## 1. Supabase

1. Créer un compte sur supabase.com (le plus simple : « Continue with GitHub »).
2. **New project** : nom `mahay`, plan gratuit, région la plus proche (Europe ou Asie), un mot de
   passe de base de données (à garder dans `release-keys/SECRETS.md`).
3. **Authentication ▸ Providers ▸ GitHub** : activer.
   - Sur GitHub : Settings ▸ Developer settings ▸ OAuth Apps ▸ **New OAuth App**.
     Homepage : `https://github.com/raantss18/mahay`.
     Authorization callback URL : `https://<réf-du-projet>.supabase.co/auth/v1/callback`
     (Supabase l'affiche dans le fournisseur GitHub).
   - Recopier le *Client ID* et générer un *Client secret* dans Supabase.
4. **Authentication ▸ Providers ▸ Google** : activer (identifiants créés à l'étape 2, ci-dessous).
5. **Authentication ▸ URL Configuration ▸ Redirect URLs** : ajouter
   `org.mahay.hub://login-callback/` (Android) et `http://localhost:43824/` (Linux, port à
   confirmer à l'implémentation).
6. **SQL Editor** : exécuter (les petites données de chaque utilisateur, chiffrées côté appareil ;
   chacun ne voit que ses lignes) :

   ```sql
   create table sync_items (
     user_id uuid not null default auth.uid() references auth.users on delete cascade,
     kind text not null,
     item_id text not null,
     blob text not null,
     updated_at timestamptz not null default now(),
     primary key (user_id, kind, item_id)
   );
   alter table sync_items enable row level security;
   create policy "own rows" on sync_items for all
     using (user_id = auth.uid()) with check (user_id = auth.uid());
   ```
7. **Project Settings ▸ API** : noter **Project URL** et la clé publique « anon / publishable »
   (ce n'est pas un secret, mais elle va dans `release-keys/` avec les autres valeurs de build).

## 2. Google Cloud (connexion Google et Drive)

1. console.cloud.google.com : créer un projet `Mahay`.
2. **APIs & Services ▸ Library** : activer **Google Drive API**.
3. **OAuth consent screen** (« Google Auth Platform ») : type *External*, nom de l'app `Mahay`,
   e-mail d'assistance, page d'accueil `https://github.com/raantss18/mahay`, une page de
   confidentialité (je la rédige dans le dépôt), puis les portées `openid`, `email`, `profile` et
   `https://www.googleapis.com/auth/drive.file`.
   - `drive.file` est une portée **non sensible** : pas d'audit de sécurité, seulement la
     vérification de la marque (nom, logo, domaine). Tant que l'application est en mode
     « Testing », les jetons expirent après 7 jours et seuls les testeurs listés peuvent se
     connecter : il faudra la passer en « In production » avant la 0.3.0.
4. **Credentials ▸ Create credentials ▸ OAuth client ID** :
   - un client *Web application* pour Supabase (redirection
     `https://<réf-du-projet>.supabase.co/auth/v1/callback`) : son *Client ID* et son *Client secret*
     vont dans Supabase (étape 1.4) ;
   - les clients pour Drive sur l'appareil (Android et Linux) : le type exact et la redirection
     seront confirmés à l'implémentation ; je te dirai alors lesquels créer.

## 3. Ce qu'il me faut au final

Seulement des **noms** et des emplacements, jamais les secrets :

| Valeur | Où la mettre |
|---|---|
| Project URL et clé anon Supabase | `release-keys/` et secrets GitHub `MAHAY_SUPABASE_URL`, `MAHAY_SUPABASE_KEY` |
| Client ID Google pour Drive | `release-keys/` et secret GitHub `MAHAY_GOOGLE_CLIENT_ID` |
| Secrets clients (GitHub, Google) | uniquement dans Supabase, jamais dans le dépôt |

Quand c'est fait, dis-moi simplement « les comptes sont créés » : je vérifie les noms des secrets
avec `gh secret list` et je branche `mahay-account`.
