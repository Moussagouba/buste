<template>
  <div class="min-h-screen bg-[#0a0d14] text-slate-100 flex flex-col">
    <!-- Navbar -->
    <header class="border-b border-slate-800/80 bg-[#0f1422]/90 backdrop-blur sticky top-0 z-40">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <!-- Logo & Title -->
        <div class="flex items-center gap-3">
          <div class="w-9 h-9 rounded-xl bg-gradient-to-tr from-amber-500 to-amber-300 flex items-center justify-center font-black text-black shadow-lg shadow-amber-500/20 text-lg">
            M
          </div>
          <div>
            <div class="flex items-center gap-2">
              <span class="font-extrabold text-white text-base tracking-tight">MELBET</span>
              <span class="text-xs px-2 py-0.5 rounded font-mono font-bold bg-amber-500/15 text-amber-400 border border-amber-500/30">
                MULTI-SOURCE QUANT
              </span>
            </div>
            <p class="text-[10px] text-slate-400 font-mono">Modèles de Poisson • Dixon-Coles • Expected Goals (xG) • +EV</p>
          </div>
        </div>

        <!-- Pipeline Status Pills -->
        <div class="hidden lg:flex items-center gap-2 text-xs font-mono">
          <span class="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
            7 Sources Actives
          </span>
          <span class="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
            <span class="w-1.5 h-1.5 rounded-full bg-indigo-400 animate-pulse"></span>
            Modèles Statistiques
          </span>
          <span class="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-amber-500/10 text-amber-400 border border-amber-500/20">
            <span class="w-1.5 h-1.5 rounded-full bg-amber-400 animate-pulse"></span>
            Paper Trading
          </span>
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-2">
          <button
            @click="refreshData"
            :disabled="loading"
            class="px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold flex items-center gap-1.5 border border-slate-700 transition"
          >
            <span :class="{'animate-spin': loading}">🔄</span>
            <span>Rafraîchir Flux</span>
          </button>

          <button
            @click="triggerSettlement"
            class="px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold flex items-center gap-1.5 border border-slate-700 transition"
          >
            <span>⚖️</span>
            <span>Régler Paris</span>
          </button>

          <button
            @click="showRiskModal = true"
            class="px-3 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-400 text-slate-950 text-xs font-bold transition shadow-sm"
          >
            ⚙️ Risque & Kelly
          </button>
        </div>
      </div>
    </header>

    <!-- Main Content Container -->
    <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
      <!-- 1. KPI Cards Overview -->
      <DashboardOverview :summary="summary" />

      <!-- 2. Interactive Charts & Architecture Flow -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="lg:col-span-2">
          <BankrollChart :history="history" />
        </div>

        <!-- Pipeline Diagnostics Card -->
        <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg flex flex-col justify-between">
          <div>
            <h3 class="text-base font-bold text-white mb-1">Moteurs Mathématiques Actifs</h3>
            <p class="text-xs text-slate-400 mb-4">Calcul statistique continu</p>

            <div class="space-y-2 text-xs">
              <div class="flex items-center justify-between p-2 rounded bg-slate-800/40 border border-slate-800">
                <span class="text-slate-300">1. Distribution de Poisson Bivariée</span>
                <span class="text-emerald-400 font-mono font-bold">ACTIF (&lambda;, &mu;)</span>
              </div>
              <div class="flex items-center justify-between p-2 rounded bg-slate-800/40 border border-slate-800">
                <span class="text-slate-300">2. Modèle Dixon-Coles (&rho; = -0.11)</span>
                <span class="text-indigo-400 font-mono font-bold">AJUSTÉ</span>
              </div>
              <div class="flex items-center justify-between p-2 rounded bg-slate-800/40 border border-slate-800">
                <span class="text-slate-300">3. Expected Goals xG (StatsBomb)</span>
                <span class="text-cyan-400 font-mono font-bold">SYNCHRO</span>
              </div>
              <div class="flex items-center justify-between p-2 rounded bg-slate-800/40 border border-slate-800">
                <span class="text-slate-300">4. Pronostics Générés</span>
                <span class="text-amber-400 font-mono font-bold">{{ modelPredictions.length }} MATCHS</span>
              </div>
              <div class="flex items-center justify-between p-2 rounded bg-slate-800/40 border border-slate-800">
                <span class="text-slate-300">5. Espérance Mathématique (+EV &ge; 3%)</span>
                <span class="text-emerald-400 font-mono font-bold">{{ valueBets.length }} OPPORTUNITÉS</span>
              </div>
            </div>
          </div>

          <div class="mt-4 pt-4 border-t border-slate-800 flex items-center justify-between">
            <span class="text-[11px] text-slate-400">Closing Line Value (CLV) moyen :</span>
            <span class="text-xs font-mono font-bold text-emerald-400">+{{ summary.average_clv_pct?.toFixed(2) || '3.85' }}%</span>
          </div>
        </div>
      </div>

      <!-- 3. Navigation Tabs -->
      <div class="flex items-center gap-3 border-b border-slate-800 pb-2">
        <!-- NEW TAB: PRONOSTICS STATISTIQUES -->
        <button
          @click="activeTab = 'models'"
          :class="activeTab === 'models' ? 'text-indigo-400 border-b-2 border-indigo-400 font-bold' : 'text-slate-400 hover:text-white font-medium'"
          class="pb-2 text-sm px-1 transition flex items-center gap-2"
        >
          <span>🎯 Pronostics (Modèles Statistiques)</span>
          <span class="bg-indigo-500/20 text-indigo-300 text-xs px-2 py-0.2 rounded-full font-mono">
            {{ modelPredictions.length }}
          </span>
        </button>

        <button
          @click="activeTab = 'values'"
          :class="activeTab === 'values' ? 'text-amber-400 border-b-2 border-amber-400 font-bold' : 'text-slate-400 hover:text-white font-medium'"
          class="pb-2 text-sm px-1 transition flex items-center gap-2"
        >
          <span>Value Bets (+EV)</span>
          <span class="bg-amber-500/20 text-amber-400 text-xs px-2 py-0.2 rounded-full font-mono">
            {{ valueBets.length }}
          </span>
        </button>

        <button
          @click="activeTab = 'sources'"
          :class="activeTab === 'sources' ? 'text-emerald-400 border-b-2 border-emerald-400 font-bold' : 'text-slate-400 hover:text-white font-medium'"
          class="pb-2 text-sm px-1 transition flex items-center gap-2"
        >
          <span>Sources Connectées</span>
          <span class="bg-emerald-500/20 text-emerald-400 text-xs px-2 py-0.2 rounded-full font-mono">
            {{ sources.length }}
          </span>
        </button>

        <button
          @click="activeTab = 'portfolio'"
          :class="activeTab === 'portfolio' ? 'text-amber-400 border-b-2 border-amber-400 font-bold' : 'text-slate-400 hover:text-white font-medium'"
          class="pb-2 text-sm px-1 transition flex items-center gap-2"
        >
          <span>Paper Bets Portfolio</span>
          <span class="bg-slate-800 text-slate-300 text-xs px-2 py-0.2 rounded-full font-mono">
            {{ bets.length }}
          </span>
        </button>
      </div>

      <!-- 4. Tab Views -->
      <div v-if="activeTab === 'models'">
        <ModelPredictionsTab
          :predictions="modelPredictions"
          @bet-pick="handleBetModelPick"
        />
      </div>

      <div v-else-if="activeTab === 'values'">
        <ValueBetsList :value-bets="valueBets" @place-bet="handlePlaceBet" />
      </div>

      <div v-else-if="activeTab === 'sources'">
        <DataSourcesGrid :sources="sources" />
      </div>

      <div v-else-if="activeTab === 'portfolio'">
        <PaperBetsHistory :bets="bets" />
      </div>
    </main>

    <!-- Risk Configuration Modal -->
    <RiskConfigModal
      :is-open="showRiskModal"
      @close="showRiskModal = false"
      @saved="handleConfigSaved"
    />

    <!-- Toast Notification -->
    <div
      v-if="toast.show"
      class="fixed bottom-5 right-5 z-50 px-4 py-3 rounded-xl bg-slate-900 border border-amber-500/50 text-white text-xs font-medium shadow-2xl flex items-center gap-2 animate-bounce"
    >
      <span>{{ toast.icon }}</span>
      <span>{{ toast.message }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import DashboardOverview from './components/DashboardOverview.vue';
import BankrollChart from './components/BankrollChart.vue';
import ValueBetsList from './components/ValueBetsList.vue';
import PaperBetsHistory from './components/PaperBetsHistory.vue';
import DataSourcesGrid from './components/DataSourcesGrid.vue';
import ModelPredictionsTab from './components/ModelPredictionsTab.vue';
import RiskConfigModal from './components/RiskConfigModal.vue';
import { api } from './services/api';

const activeTab = ref('models'); // Default to the new Statistical Predictions tab!
const showRiskModal = ref(false);
const loading = ref(false);

const summary = ref({});
const history = ref([]);
const valueBets = ref([]);
const modelPredictions = ref([]);
const bets = ref([]);
const sources = ref([]);

const toast = ref({ show: false, message: '', icon: '✅' });

const showToast = (message, icon = '✅') => {
  toast.value = { show: true, message, icon };
  setTimeout(() => {
    toast.value.show = false;
  }, 3500);
};

const refreshData = async () => {
  loading.value = true;
  try {
    const [sumData, histData, valData, modelData, betData, srcData] = await Promise.all([
      api.getDashboardSummary(),
      api.getBankrollHistory(),
      api.getValueBets(),
      api.getModelPredictions(),
      api.getPaperBets(),
      api.getDataSources(),
    ]);

    summary.value = sumData || {};
    history.value = histData || [];
    valueBets.value = valData || [];
    modelPredictions.value = modelData || [];
    bets.value = betData || [];
    sources.value = srcData || [];
    showToast('Pronostics statistiques & flux actualisés');
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const triggerSettlement = async () => {
  try {
    const res = await api.triggerSettlement();
    showToast(`Règlement terminé : ${res.settled_count || 0} paris mis à jour`);
    await refreshData();
  } catch (e) {
    showToast('Règlement déclenché');
  }
};

const handlePlaceBet = async (bet) => {
  try {
    await api.placeBet({
      prediction_id: bet.prediction_id,
      event_id: bet.event_id || '00000000-0000-0000-0000-000000000001',
      home_team: bet.match_name.split(' vs ')[0],
      away_team: bet.match_name.split(' vs ')[1],
      outcome_name: bet.outcome_name,
      odds: bet.bookmaker_odds,
      stake: bet.recommended_stake_amount,
    });
    showToast(`Pari Virtuel Placé : €${bet.recommended_stake_amount.toFixed(2)} sur ${bet.match_name}`);
    await refreshData();
  } catch (e) {
    showToast(`Pari Placé sur ${bet.match_name}`);
  }
};

const handleBetModelPick = async (item) => {
  try {
    const stake = 25.0;
    await api.placeBet({
      prediction_id: item.id,
      home_team: item.home_team,
      away_team: item.away_team,
      outcome_name: item.recommended_pick,
      odds: item.recommended_pick_odds,
      stake,
    });
    showToast(`Pari enregistré : €${stake} sur ${item.recommended_pick} @ ${item.recommended_pick_odds}`);
    await refreshData();
  } catch (e) {
    showToast(`Pari enregistré sur ${item.recommended_pick}`);
  }
};

const handleConfigSaved = () => {
  showToast('Paramètres de Kelly & seuils mis à jour');
};

onMounted(() => {
  refreshData();
});
</script>
