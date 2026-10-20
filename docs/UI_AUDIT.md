# Interface audit (D29): first pass

What each app shows on a phone, and what to remove, replace or hide. **First pass, 2026-10-20**: the
0.2.4 apps (x86_64 emulator, 1080×2340 at 420 dpi, start screen only). Nothing here was removed yet; each
line says what was seen and what to do. A line marked *check* needs someone to read the code before
removing anything. Rule (D29): hide or move rare professional features, remove only what needs a service
that does not exist; keep upstream merges possible (D24).

## Everywhere

| Seen | Decision |
|---|---|
| About links did not open on Android | Fixed (0.2.5). |
| "Discussions" links answered 404 | Discussions enabled on the repository. |
| No way to send feedback | Added to About in every app (D28). |
| Community links repeated up to three times (top bar, side list, home card) in Tselatra, Hetsika, Taratasy | Keep one place per app (home card or About). *check* each app's wording. |
| Debug readouts in the status bar (`render 0.0 ms · ui 1.0`) in Tsilo | Hide unless a developer switch is on. |

## Phone layout (shared with the layouts session, D23)

Seen unusable on 411 dp: **Hetsika** (menus drawn over each other, panels cut), **Pejy** (welcome text one
letter per line, three columns), **Tsilo** (same), **Hazavana** (three panels, search bar overlapping its
own icons), **Feo** (menu bar cut), **Tselatra** (home page cut at the right), **Drafitra** (start page
cut), **Endrika** (home cut at the right). Fine or close: **Tabilao**, **Seho**, **Soratra**, **Taratasy**.
The cure is the layout work (D23) plus a Simple workspace for the apps people use most (D27). Sary has it
(0.2.5); Hetsika is next, because its phone layout is the worst.

## Per app

- **Sary** (Simple workspace done): six tasks. Pro keeps everything; a small "Simple" button is added at its
  bottom right. To do: a real "Auto" (from the histogram), filter thumbnails, export straight from Simple.
- **Hetsika**: unusable layout on a phone (above). **Simple workspace built (0.2.5):** Add, Edit, Text, Speed,
  Sound, Export. To do: the monitor still shows Pro's transport (Fit, 1/2, markers, in/out): replace it by a
  play button and a time bar; thumbnails in the story strip; check the export screen on a phone.
- **Hazavana**: top bar shows a cloud icon, a "chat" icon (feedback?) and a share icon next to search and
  help. *check* what each opens; remove those that need a service; the Mahay feedback form replaces "chat".
- **Endrika**: home has "Online" (a server the user must set in Preferences, so it is empty by default),
  "Join with a code" and a "Share with a code" step. Live sharing on the local network is a working,
  local-first feature: **keep**; hide "Online" until a server is set, and say so in one line.
- **Seho**: a left list with **Account** and **Options**. *check*: Account has nothing to sign in to until
  0.3.0; hide it until then. "Join the Mahay community on GitHub" is fine.
- **Taratasy**: home card "Join the Mahay community" with four buttons (discussions, web page, GitHub,
  Mahay website). Keep two: discussions and GitHub. The recommended-tools cards are good.
- **Soratra**: the file menu lists Info, Recover, Save, Save As, Print, Export (greyed with no document):
  fine. "Community discussions" is fine.
- **Tabilao**: no change needed on the start screen; the ribbon is already phone-friendly.
- **Pejy, Tsilo, Feo, Tselatra, Drafitra**: nothing to remove before the layout is fixed; then audit the
  menus screen by screen.

## Not yet done

Opening documents in each app (menus, dialogs, context menus), tablet and dark theme, Linux. This file
grows as each is looked at; every removal gets a line here with its reason.
