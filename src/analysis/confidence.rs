pub fn calculate_confidence_score(
    sport: &str,
    league: &str,
    hours_until_start: f64,
    bookmaker_overround: f64,
    odds: f64,
) -> f64 {
    let mut score = 0.70; // Base confidence

    // 1. Tier 1 leagues have higher market liquidity and information accuracy
    let l = league.to_lowercase();
    if l.contains("premier league") || l.contains("champions league") || l.contains("nba") {
        score += 0.15;
    } else if l.contains("la liga") || l.contains("serie a") || l.contains("bundesliga") {
        score += 0.10;
    }

    // 2. Proximity to event: 2-24 hours prior gives optimal price discovery
    if hours_until_start >= 1.0 && hours_until_start <= 48.0 {
        score += 0.05;
    } else if hours_until_start < 1.0 {
        // Line-ups confirmed, but high volatility
        score += 0.02;
    } else {
        // Far in advance, line-up uncertainty
        score -= 0.08;
    }

    // 3. Lower bookmaker margin means tighter, sharper markets
    if bookmaker_overround < 0.04 {
        score += 0.08;
    } else if bookmaker_overround > 0.08 {
        score -= 0.08;
    }

    // 4. Extreme longshots or heavy favorites have higher variance
    if odds > 4.5 || odds < 1.25 {
        score -= 0.05;
    }

    score.clamp(0.10, 0.99)
}

