<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
    <!-- 1. Total Bankroll -->
    <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg relative overflow-hidden group hover:border-amber-500/50 transition">
      <div class="flex items-center justify-between text-slate-400 text-xs font-semibold uppercase tracking-wider">
        <span>Bankroll</span>
        <span class="text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded text-[10px]">Virtual</span>
      </div>
      <div class="mt-2 flex items-baseline gap-2">
        <span class="text-3xl font-extrabold text-white tracking-tight">
          €{{ summary.current_bankroll?.toFixed(2) || '1,000.00' }}
        </span>
      </div>
      <div class="mt-2 flex items-center text-xs">
        <span :class="summary.net_profit >= 0 ? 'text-emerald-400' : 'text-rose-400'" class="font-medium flex items-center gap-1">
          {{ summary.net_profit >= 0 ? '+' : '' }}€{{ summary.net_profit?.toFixed(2) || '0.00' }}
          ({{ (((summary.net_profit || 0) / (summary.initial_bankroll || 1000)) * 100).toFixed(1) }}%)
        </span>
        <span class="text-slate-500 ml-2">from €{{ summary.initial_bankroll?.toFixed(0) || '1000' }}</span>
      </div>
      <div class="absolute -right-4 -bottom-4 w-16 h-16 bg-amber-500/5 rounded-full blur-xl group-hover:bg-amber-500/10 transition"></div>
    </div>

    <!-- 2. ROI -->
    <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg relative overflow-hidden group hover:border-emerald-500/50 transition">
      <div class="flex items-center justify-between text-slate-400 text-xs font-semibold uppercase tracking-wider">
        <span>ROI (Yield)</span>
        <span class="text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded text-[10px]">+EV Target</span>
      </div>
      <div class="mt-2 flex items-baseline gap-2">
        <span class="text-3xl font-extrabold" :class="summary.roi_pct >= 0 ? 'text-emerald-400' : 'text-rose-400'">
          {{ summary.roi_pct >= 0 ? '+' : '' }}{{ summary.roi_pct?.toFixed(2) || '0.00' }}%
        </span>
      </div>
      <div class="mt-2 text-xs text-slate-400">
        Total staked: <span class="text-slate-200 font-mono">€{{ summary.total_staked?.toFixed(1) || '0.0' }}</span>
      </div>
      <div class="absolute -right-4 -bottom-4 w-16 h-16 bg-emerald-500/5 rounded-full blur-xl group-hover:bg-emerald-500/10 transition"></div>
    </div>

    <!-- 3. Win Rate -->
    <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg relative overflow-hidden group hover:border-cyan-500/50 transition">
      <div class="flex items-center justify-between text-slate-400 text-xs font-semibold uppercase tracking-wider">
        <span>Win Rate</span>
        <span class="text-cyan-400 bg-cyan-500/10 px-2 py-0.5 rounded text-[10px]">Settled</span>
      </div>
      <div class="mt-2 flex items-baseline gap-2">
        <span class="text-3xl font-extrabold text-cyan-400">
          {{ summary.win_rate_pct?.toFixed(1) || '0.0' }}%
        </span>
      </div>
      <div class="mt-2 text-xs text-slate-400">
        <span class="text-emerald-400 font-bold">{{ summary.won_bets_count || 0 }}W</span> - 
        <span class="text-rose-400 font-bold">{{ summary.lost_bets_count || 0 }}L</span> 
        ({{ summary.settled_bets_count || 0 }} total)
      </div>
      <div class="absolute -right-4 -bottom-4 w-16 h-16 bg-cyan-500/5 rounded-full blur-xl group-hover:bg-cyan-500/10 transition"></div>
    </div>

    <!-- 4. Max Drawdown -->
    <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg relative overflow-hidden group hover:border-rose-500/50 transition">
      <div class="flex items-center justify-between text-slate-400 text-xs font-semibold uppercase tracking-wider">
        <span>Drawdown</span>
        <span class="text-rose-400 bg-rose-500/10 px-2 py-0.5 rounded text-[10px]">Risk Cap 25%</span>
      </div>
      <div class="mt-2 flex items-baseline gap-2">
        <span class="text-3xl font-extrabold text-white">
          {{ summary.current_drawdown_pct?.toFixed(1) || '0.0' }}%
        </span>
      </div>
      <div class="mt-2 text-xs text-slate-400">
        Max recorded: <span class="text-rose-400 font-mono">{{ summary.max_drawdown_pct?.toFixed(1) || '0.0' }}%</span>
      </div>
      <div class="absolute -right-4 -bottom-4 w-16 h-16 bg-rose-500/5 rounded-full blur-xl group-hover:bg-rose-500/10 transition"></div>
    </div>

    <!-- 5. Value Bets Available -->
    <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg relative overflow-hidden group hover:border-indigo-500/50 transition">
      <div class="flex items-center justify-between text-slate-400 text-xs font-semibold uppercase tracking-wider">
        <span>Active Signals</span>
        <span class="text-indigo-400 bg-indigo-500/10 px-2 py-0.5 rounded text-[10px]">Real-Time</span>
      </div>
      <div class="mt-2 flex items-baseline gap-2">
        <span class="text-3xl font-extrabold text-indigo-400">
          {{ summary.active_value_bets_count || 0 }}
        </span>
        <span class="text-xs text-slate-400">opportunities</span>
      </div>
      <div class="mt-2 text-xs text-slate-400">
        Active bets in play: <span class="text-amber-400 font-mono">{{ summary.active_bets_count || 0 }}</span>
      </div>
      <div class="absolute -right-4 -bottom-4 w-16 h-16 bg-indigo-500/5 rounded-full blur-xl group-hover:bg-indigo-500/10 transition"></div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  summary: {
    type: Object,
    required: true,
  },
});
</script>

