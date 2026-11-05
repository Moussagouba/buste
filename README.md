# Melbet Quantitative Analytics & Value Betting Platform

Une infrastructure complète haute-performance en **Rust** et **Vue.js** pour la collecte de cotes autorisée (Melbet), la modélisation probabiliste (Poisson / Bivariate), la détection mathématique de Value Bets (+EV), la gestion rigoureuse des risques (Critère de Kelly fractionné) et la simulation de Paper Trading avec suivi en temps réel du ROI, Win Rate, Drawdown et Closing Line Value (CLV).

---

## 🏗️ Architecture du Système

```
                         ┌─────────────────────┐
                         │      MELBET         │
                         │   Données / cotes   │
                         └──────────┬──────────┘
                                    │
                         collecte autorisée
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────┐
│                    RUST BACKEND                          │
│                                                         │
│  ┌──────────────┐    ┌──────────────┐                  │
│  │ Data Collector│───▶│  Normalizer  │                  │
│  └──────────────┘    └──────┬───────┘                  │
│                             │                            │
│                             ▼                            │
│                    ┌────────────────┐                    │
│                    │   PostgreSQL   │                    │
│                    └───────┬────────┘                    │
│                            │                             │
│             ┌──────────────┴──────────────┐              │
│             ▼                             ▼              │
│      ┌──────────────┐              ┌──────────────┐     │
│      │ Feature      │              │ Historical   │     │
│      │ Engine       │              │ Analyzer     │     │
│      └──────┬───────┘              └──────┬───────┘     │
│             │                             │              │
│             └──────────────┬──────────────┘              │
│                            ▼                             │
│                    ┌───────────────┐                     │
│                    │ Probability   │                     │
│                    │ Engine        │                     │
│                    └───────┬───────┘                     │
│                            ▼                             │
│                    ┌───────────────┐                     │
│                    │ Value Engine  │                     │
│                    └───────┬───────┘                     │
│                            ▼                             │
│                    ┌───────────────┐                     │
│                    │ Risk Manager  │                     │
│                    └───────┬───────┘                     │
│                            ▼                             │
│                  ┌────────────────────┐                  │
│                  │   PAPER BETTING    │                  │
│                  └─────────┬──────────┘                  │
│                            ▼                             │
│                    ┌───────────────┐                     │
│                    │ Performance   │                     │
│                    │ Analyzer      │                     │
│                    └───────────────┘                     │
└─────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │ Vue Dashboard    │
                    │                  │
                    │ Bankroll         │
                    │ ROI              │
                    │ Win rate         │
                    │ Drawdown         │
                    │ Value bets       │
                    └──────────────────┘
```

---

## 📁 Structure du Projet

```text
melbet-analytics/
│
├── Cargo.toml                  # Dépendances Tokio, Axum, SQLx, Serde, Reqwest
├── Cargo.lock
├── .env.example                # Variables d'environnement & configuration
├── README.md                   # Documentation technique
├── docker-compose.yml          # Déploiement multi-conteneur (Postgres, Rust, Vue)
├── Dockerfile                  # Build conteneurisé du backend Rust
│
├── migrations/                 # Migrations SQL versionnées (PostgreSQL)
│   ├── 001_users.sql           # Comptes analystes & rôles
│   ├── 002_events.sql          # Événements sportifs collectés (ID, équipes, statut)
│   ├── 003_markets.sql         # Marchés (1X2, Over/Under, BTTS, Handicap)
│   ├── 004_odds.sql            # Snapshots des cotes décimales & marges bookmaker
│   ├── 005_predictions.sql     # Probabilités estimées, cotes équitables & EV
│   ├── 006_paper_bets.sql      # Ordres de paris virtuels & calcul du CLV
│   └── 007_bankroll.sql        # Ledger des transactions & snapshots de capital
│
├── src/
│   ├── main.rs                 # Point d'entrée, initialisation & schedulers
│   ├── config.rs               # Gestionnaire de configuration typé
│   ├── error.rs                # Gestion centralisée des erreurs (thiserror/axum)
│   │
│   ├── api/                    # Couche API REST haute performance (Axum)
│   │   ├── mod.rs              # Définition de l'AppState
│   │   ├── routes.rs           # Routeur, middlewares CORS & tracing
│   │   ├── events.rs           # Endpoints événements & rafraîchissement
│   │   ├── predictions.rs      # Flux des signaux +EV en temps réel
│   │   ├── bets.rs             # Prise d'ordre virtuel & règlement
│   │   └── dashboard.rs        # Synthèse KPI, courbe d'équité & backtest
│   │
│   ├── models/                 # Modèles de données & DTO sérialisables
│   │   ├── mod.rs
│   │   ├── event.rs
│   │   ├── market.rs
│   │   ├── odds.rs
│   │   ├── prediction.rs
│   │   ├── bet.rs
│   │   └── bankroll.rs
│   │
│   ├── collector/              # Collecte des cotes et normalisation
│   │   ├── mod.rs
│   │   ├── melbet.rs           # Client API Melbet avec simulateur haute fidélité
│   │   ├── parser.rs           # Normaliseur des types de marchés & cotes
│   │   └── scheduler.rs        # Tâche asynchrone de scrutation périodique
│   │
│   ├── analysis/               # Modélisation quantitative & mathématique
│   │   ├── mod.rs
│   │   ├── features.rs         # Extraction des forces d'attaque/défense
│   │   ├── probability.rs      # Matrice de score de Poisson & retrait de marge
│   │   ├── value.rs            # Espérance mathématique (EV) & Edge
│   │   └── confidence.rs       # Score de confiance de liquidité & dispersion
│   │
│   ├── strategy/               # Règles d'arbitrage & génération de signaux
│   │   ├── mod.rs
│   │   ├── strategy.rs         # Configuration de la stratégie (Quarter Kelly)
│   │   ├── filters.rs          # Filtres d'éligibilité (min EV, cotes min/max)
│   │   └── signals.rs          # Détecteur des opportunités +EV
│   │
│   ├── risk/                   # Gestion des risques & préservation du capital
│   │   ├── mod.rs
│   │   ├── bankroll.rs         # Ledger comptable, débits, crédits & peak
│   │   ├── staking.rs          # Dimensionnement de mise (Critère de Kelly)
│   │   └── limits.rs           # Disjoncteur de Stop-Loss & exposition max
│   │
│   ├── simulation/             # Paper Trading & Backtesting
│   │   ├── mod.rs
│   │   ├── paper_betting.rs    # Exécution des ordres de paris virtuels
│   │   ├── settlement.rs       # Résolution automatique des résultats & P&L
│   │   └── backtest.rs         # Moteur de test historique sur 30 jours
│   │
│   ├── database/               # Persistance PostgreSQL
│   │   ├── mod.rs
│   │   └── postgres.rs         # Pool SQLx & exécution des migrations
│   │
│   └── telemetry/              # Observabilité & journalisation
│       ├── mod.rs
│       └── logging.rs          # Tracing structuré JSON / standard output
│
└── frontend/                   # Interface Utilisateur Réactive (Vue.js + Tailwind)
    ├── package.json
    ├── vite.config.js
    ├── tailwind.config.js
    ├── index.html
    ├── src/
    │   ├── main.js
    │   ├── App.vue             # Dashboard complet de trading sportif
    │   ├── assets/main.css     # Styles Tailwind & thème sombre
    │   ├── services/api.js     # Client API REST
    │   └── components/
    │       ├── DashboardOverview.vue   # KPIs (Bankroll, ROI, Win Rate, Drawdown)
    │       ├── BankrollChart.vue       # Courbe d'évolution du capital SVG
    │       ├── ValueBetsList.vue       # Tableau interactif des paris +EV
    │       ├── PaperBetsHistory.vue    # Registre des paris & CLV
    │       └── RiskConfigModal.vue     # Réglage du Kelly & seuils de risque
    └── Dockerfile
```

---

## 🧮 Fondements Mathématiques

### 1. Espérance Mathématique (Expected Value - EV)
L'espérance mathématique d'un pari représente le gain ou la perte théorique par unité misée :

$$EV = (p_{\text{modèle}} \times \text{cote}_{\text{bookmaker}}) - 1.0$$

- Un pari est qualifié de **Value Bet** si $EV \ge +0.03$ (+3% d'avantage théorique).

### 2. Modélisation de Poisson pour les Scores
Pour un match de football où l'intensité attendue de buts est $\lambda$ (équipe à domicile) et $\mu$ (équipe à l'extérieur) :

$$P(X = x, Y = y) = \frac{\lambda^x e^{-\lambda}}{x!} \times \frac{\mu^y e^{-\mu}}{y!}$$

Les probabilités réelles des marchés sont dérivées par sommation :
- **Victoire Domicile (1)** : $\sum_{x > y} P(x, y)$
- **Match Nul (X)** : $\sum_{x = y} P(x, y)$
- **Victoire Extérieur (2)** : $\sum_{x < y} P(x, y)$
- **Total Plus de 2.5 Buts** : $\sum_{x+y > 2.5} P(x, y)$

### 3. Critère de Kelly Fractionné
Le dimensionnement optimal du capital pour maximiser la croissance logarithmique du capital tout en évitant le risque de ruine :

$$f^* = \frac{b \cdot p - q}{b}$$

Où :
- $b = \text{cote} - 1$ (rapport de gain net)
- $p = p_{\text{modèle}}$ (probabilité estimée de succès)
- $q = 1 - p$ (probabilité d'échec)

Afin d'amortir la variance des séries sportives, nous appliquons un **Quarter Kelly (0.25)** avec un plafond de sécurité de 5% du capital par pari :

$$\text{Mise} = \min(\text{Bankroll} \times f^* \times 0.25, \; \text{Bankroll} \times 0.05)$$

### 4. Closing Line Value (CLV)
Le CLV mesure la capacité du modèle à battre le marché avant le coup d'envoi (mesure ultime de la compétence prédictive) :

$$\text{CLV} = \frac{\text{Cote}_{\text{prise}}}{\text{Cote}_{\text{clôture}}} - 1.0$$

---

## 🚀 Démarrage Rapide

### Option 1 : Avec Docker Compose (Recommandé)

Lancez l'ensemble des services (PostgreSQL, Backend Rust, Dashboard Vue.js) en une seule commande :

```bash
docker compose up --build -d
```

- **Dashboard Web** : [http://localhost:3000](http://localhost:3000)
- **API Rust** : [http://localhost:8080/api/dashboard/summary](http://localhost:8080/api/dashboard/summary)
- **PostgreSQL** : `localhost:5432` (identifiant: `melbet_user`, mdp: `melbet_secure_pass`)

---

### Option 2 : Exécution Locale

#### 1. Démarrer PostgreSQL
```bash
docker compose up -d postgres
```

#### 2. Démarrer le Backend Rust
```bash
cp .env.example .env
cargo run --release
```

#### 3. Démarrer le Frontend Vue.js
```bash
cd frontend
npm install
npm run dev
```

Accédez à [http://localhost:3000](http://localhost:3000).

---

## 📡 Endpoints de l'API REST

| Méthode | Route | Description |
|---|---|---|
| `GET` | `/api/dashboard/summary` | Synthèse des métriques clés (Bankroll, ROI, Win rate, Drawdown) |
| `GET` | `/api/dashboard/bankroll-history` | Série temporelle de l'évolution du capital pour le graphique |
| `POST`| `/api/dashboard/backtest` | Exécute une simulation historique sur 30 jours |
| `GET` | `/api/predictions/value-bets` | Liste des opportunités +EV actuelles détectées |
| `GET` | `/api/events` | Liste des matchs et cotes collectés depuis Melbet |
| `POST`| `/api/events/refresh` | Force un cycle de collecte et normalisation de Melbet |
| `GET` | `/api/bets` | Historique des paris virtuels avec statuts et CLV |
| `POST`| `/api/bets` | Place un pari virtuel (Paper Bet) |
| `POST`| `/api/bets/settle` | Déclenche le règlement des paris terminés |

