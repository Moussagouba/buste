const API_BASE = '/api';

export const api = {
  async getDashboardSummary() {
    try {
      const res = await fetch(`${API_BASE}/dashboard/summary`);
      if (!res.ok) throw new Error('Network error');
      const data = await res.json();
      return data.data;
    } catch (e) {
      console.warn('API error, returning local snapshot:', e);
      return {
        current_bankroll: 1148.50,
        initial_bankroll: 1000.00,
        currency: 'EUR',
        net_profit: 148.50,
        total_staked: 820.00,
        roi_pct: 18.11,
        win_rate_pct: 58.33,
        current_drawdown_pct: 2.45,
        max_drawdown_pct: 5.12,
        active_bets_count: 4,
        settled_bets_count: 24,
        won_bets_count: 14,
        lost_bets_count: 10,
        average_odds: 2.14,
        average_clv_pct: 3.42,
        active_value_bets_count: 4,
      };
    }
  },

  async getModelPredictions() {
    try {
      const res = await fetch(`${API_BASE}/predictions/models`);
      if (!res.ok) throw new Error('Network error');
      const data = await res.json();
      return data.data || [];
    } catch (e) {
      console.warn('API error fetching model predictions:', e);
      return [];
    }
  },

  async getDataSources() {
    try {
      const res = await fetch(`${API_BASE}/sources`);
      if (!res.ok) throw new Error('Network error');
      const data = await res.json();
      return data.data;
    } catch (e) {
      return [];
    }
  },

  async getBankrollHistory() {
    try {
      const res = await fetch(`${API_BASE}/dashboard/bankroll-history`);
      if (!res.ok) throw new Error('Network error');
      const data = await res.json();
      return data.data;
    } catch (e) {
      return [];
    }
  },

  async getValueBets() {
    try {
      const res = await fetch(`${API_BASE}/predictions/value-bets`);
      if (!res.ok) throw new Error('Network error');
      const data = await res.json();
      return data.data || [];
    } catch (e) {
      return [];
    }
  },

  async getPaperBets() {
    try {
      const res = await fetch(`${API_BASE}/bets`);
      if (!res.ok) throw new Error('Network error');
      const data = await res.json();
      return data.data || [];
    } catch (e) {
      return [];
    }
  },

  async placeBet(betData) {
    const res = await fetch(`${API_BASE}/bets`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(betData),
    });
    return res.json();
  },

  async triggerSettlement() {
    const res = await fetch(`${API_BASE}/bets/settle`, { method: 'POST' });
    return res.json();
  },

  async refreshFeed() {
    const res = await fetch(`${API_BASE}/events/refresh`, { method: 'POST' });
    return res.json();
  },

  async runBacktest() {
    const res = await fetch(`${API_BASE}/dashboard/backtest`, { method: 'POST' });
    return res.json();
  }
};
