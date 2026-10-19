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
   `org.mahay.hub://login-callback/` (Android) et `http://localhost:43824/` (Linux ; le port est fixé à 43824).
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

Valeurs à utiliser (toutes publiques) :

| | |
|---|---|
| Page d'accueil | `https://github.com/raantss18/mahay` |
| Politique de confidentialité | `https://github.com/raantss18/mahay/blob/main/docs/PRIVACY.md` (déjà écrite, fr et en) |
| Nom de l'app | `Mahay` |
| Paquet Android du hub | `org.mahay.hub` |
| Empreinte **SHA-1** de la clé de signature | `49:FC:E8:AE:44:E6:DD:E9:F0:25:BF:11:CA:10:AA:E1:19:A3:F2:9D` |
| Empreinte SHA-256 | `1E:0A:BB:5D:8F:F4:08:14:15:DB:4E:96:09:7F:66:1E:61:87:15:C2:E6:A0:FD:48:FD:42:5D:5C:A2:AC:C7:77` |

1. console.cloud.google.com : créer un projet `Mahay`.
2. **APIs & Services ▸ Library** : activer **Google Drive API**.
3. **OAuth consent screen** (« Google Auth Platform ») : type *External*, nom `Mahay`, e-mail d'assistance,
   page d'accueil et politique de confidentialité (ci-dessus), puis les portées `openid`, `email`, `profile`
   et `https://www.googleapis.com/auth/drive.file`.
   - `drive.file` est une portée **non sensible** : pas d'audit de sécurité, seulement la vérification de la
     marque (nom, logo, domaine). En mode « Testing », les jetons expirent après 7 jours et seuls les
     testeurs listés peuvent se connecter : il faut passer en « In production » avant la 0.3.0. Google peut
     demander un domaine vérifié pour la page d'accueil : je peux activer GitHub Pages
     (`raantss18.github.io/mahay`) comme page d'accueil et de confidentialité si c'est le cas.
4. **Credentials ▸ Create credentials ▸ OAuth client ID**, trois clients :
   - **Web application**, pour Supabase : redirection
     `https://<réf-du-projet>.supabase.co/auth/v1/callback`. Son *Client ID* et son *Client secret* vont dans
     Supabase (étape 1.4) ; **le secret n'est jamais mis ailleurs**.
   - **Android**, pour Drive sur Android : paquet `org.mahay.hub` et l'empreinte SHA-1 ci-dessus. Son
     *Client ID* (public) va dans `release-keys/accounts.env` (`MAHAY_GOOGLE_ANDROID_CLIENT_ID`).
   - **Desktop app**, pour Drive sur Linux (redirection sur la boucle locale). Son *Client ID* et son secret
     (que Google dit non confidentiel pour une app installée) vont dans `accounts.env`.

## 3. Ce qu'il me faut au final

Seulement des **valeurs publiques**, jamais un secret :

| Valeur | Où la mettre |
|---|---|
| Project URL et clé anon Supabase | `release-keys/accounts.env` (`MAHAY_SUPABASE_URL`, `MAHAY_SUPABASE_KEY`) |
| Client IDs Google (Android, Desktop) | `release-keys/accounts.env` |
| Secret du client Desktop | `release-keys/accounts.env` (non confidentiel pour une app installée) |
| Secrets du client Web Google et de l'app OAuth GitHub | uniquement dans Supabase, et le gestionnaire de mots de passe |

Puis `release-keys/set-github-secrets.sh` les envoie à GitHub. Dis-moi simplement « les comptes sont créés » :
je vérifie les **noms** des secrets avec `gh secret list` et je branche `mahay-account`.
