# Retours et diagnostics : mise en place dans Supabase

Décision [D28](DECISIONS.md). À faire **une fois** par le propriétaire du projet Supabase, quand vous voulez
que les apps puissent envoyer un petit message ou un rapport. Les apps n'envoient rien tant que
l'utilisateur n'a pas choisi de le faire ; la page de confidentialité ([PRIVACY.md](PRIVACY.md)) le dit.

## Ce que ça fait

- Deux tables, **en écriture seule pour les apps** : `feedback` (le petit message + des informations
  techniques facultatives) et `diagnostics` (événements anonymes : une app a planté, une mise à jour a
  échoué…). Personne ne peut les **lire** avec la clé publique : vous les lisez dans Supabase
  (Table Editor), avec votre compte.
- Chaque appareil a un identifiant **aléatoire** (`install_id`), sans lien avec un compte Google ou GitHub.
- Une limite par appareil (20 messages par heure, 200 événements par heure) arrête le plus gros des abus.
  Quelqu'un qui extrait la clé publique d'une app pourrait quand même envoyer du courrier indésirable ;
  vous pouvez alors supprimer des lignes, et nous ajouterons une vérification plus forte.

## Le script

Dans Supabase : **SQL Editor ▸ New query**, collez tout, **Run**.

```sql
-- Mahay: feedback and anonymous diagnostics (D28). Insert-only for the apps.

create table public.feedback (
  id          bigint generated always as identity primary key,
  created_at  timestamptz not null default now(),
  install_id  uuid,
  app         text not null check (app in ('hub','sary','taratasy','soratra','tabilao','seho','tsilo','endrika',
                                          'pejy','hazavana','feo','hetsika','tselatra','drafitra','texnika','naoty')),
  version     text not null check (char_length(version) <= 20),
  kind        text not null default 'idea' check (kind in ('bug','idea','question','praise')),
  message     text not null check (char_length(message) between 1 and 1000),
  contact     text check (contact is null or char_length(contact) <= 200),
  device      jsonb check (device is null or pg_column_size(device) <= 2000),
  status      text not null default 'new' check (status in ('new','read','done'))
);

create table public.diagnostics (
  id          bigint generated always as identity primary key,
  created_at  timestamptz not null default now(),
  install_id  uuid not null,
  app         text not null check (app in ('hub','sary','taratasy','soratra','tabilao','seho','tsilo','endrika',
                                          'pejy','hazavana','feo','hetsika','tselatra','drafitra','texnika','naoty')),
  version     text not null check (char_length(version) <= 20),
  event       text not null check (event in ('open','crash','native_crash','anr','low_memory','killed',
                                            'update_ok','update_failed','install_failed')),
  detail      text check (detail is null or char_length(detail) <= 500),
  device      jsonb check (device is null or pg_column_size(device) <= 2000)
);

alter table public.feedback    enable row level security;
alter table public.diagnostics enable row level security;

-- The apps may only insert. No select, update or delete policy exists: nobody can read with the public key.
revoke all on public.feedback, public.diagnostics from anon, authenticated;
grant insert on public.feedback, public.diagnostics to anon, authenticated;

create policy "send feedback"    on public.feedback    for insert to anon, authenticated with check (status = 'new');
create policy "send diagnostics" on public.diagnostics for insert to anon, authenticated with check (true);

-- At most 20 messages and 200 events per device and hour.
create function public.limit_feedback() returns trigger language plpgsql security definer set search_path = public as $$
begin
  if new.install_id is not null
     and (select count(*) from public.feedback where install_id = new.install_id and created_at > now() - interval '1 hour') >= 20 then
    raise exception 'too many messages';
  end if;
  return new;
end $$;

create function public.limit_diagnostics() returns trigger language plpgsql security definer set search_path = public as $$
begin
  if (select count(*) from public.diagnostics where install_id = new.install_id and created_at > now() - interval '1 hour') >= 200 then
    raise exception 'too many events';
  end if;
  return new;
end $$;

create trigger limit_feedback    before insert on public.feedback    for each row execute function public.limit_feedback();
create trigger limit_diagnostics before insert on public.diagnostics for each row execute function public.limit_diagnostics();

-- Faster reading in the dashboard.
create index feedback_created    on public.feedback    (created_at desc);
create index diagnostics_created on public.diagnostics (created_at desc);
create index diagnostics_event   on public.diagnostics (app, event, created_at desc);
```

## Vérifier

Avec la clé publique, on doit pouvoir **écrire** mais pas **lire** (le script de contrôle que j'écris avec les
apps le fait). Dans Supabase, vous lisez les données dans **Table Editor** ▸ `feedback` et `diagnostics`.

Note technique : l'API ne peut pas renvoyer la ligne qu'on vient d'écrire (il n'y a pas de lecture) ; les apps
envoient l'en-tête `Prefer: return=minimal`.
