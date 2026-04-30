import { Cvss3P1 } from 'ae-cvss-calculator';
import styles from './Cvss3Card.module.css';
import { scoreToSeverity, Severity } from '../../../../api/osvRecord';
import { ScoreBar } from '../ScoreBar/ScoreBar';
import { SubScoreChip } from '../SubScoreChip/SubScoreChip';
import { CvssMetricsCard, type CvssMetric } from '../CvssMetricsCard/CvssMetricsCard';

const CVSS_V3_GUIDE_URL = 'https://www.first.org/cvss/v3.0/specification-document';

// ── Style maps ────────────────────────────────────────────────────────────────

const SCORE_CLASS: Record<Severity, string> = {
  CRITICAL: styles.scoreCritical,
  HIGH: styles.scoreHigh,
  MEDIUM: styles.scoreMedium,
  LOW: styles.scoreLow,
  NONE: styles.scoreNone,
};

// ── Component ─────────────────────────────────────────────────────────────────

type Cvss3CardProps = { score: string };

export function Cvss3Card({ score }: Cvss3CardProps) {
  const cvss = new Cvss3P1(score);
  const scores = cvss.calculateScores();
  const overall = scores.overall;
  const severity = scoreToSeverity(overall);

  const subScores: Array<[string, number]> = (
    [
      ['base', scores.base],
      ['impact', scores.impact],
      ['exploitability', scores.exploitability],
      ['temporal', scores.temporal],
      ['environmental', scores.environmental],
      ['modified impact', scores.modifiedImpact],
    ] as Array<[string, number | undefined]>
  ).filter((s): s is [string, number] => s[1] !== undefined && s[1] !== null);

  const components = cvss.getComponents();
  const registeredComponents = cvss.getRegisteredComponents();

  const metricCategories: Array<{ title: string; metrics: CvssMetric[] }> = [];
  for (const [category, categoryComponents] of registeredComponents) {
    const metrics: CvssMetric[] = [];
    for (const component of categoryComponents) {
      const value = components.get(component);
      if (value !== undefined) {
        metrics.push({
          title: component.name,
          metricDescription: component.description,
          value: value.name,
          description: value.description,
        });
      }
    }
    if (metrics.length > 0) {
      metricCategories.push({ title: category.name, metrics });
    }
  }

  return (
    <div className={styles.card}>
      <a
        href={CVSS_V3_GUIDE_URL}
        target="_blank"
        rel="noopener noreferrer"
        className={styles.versionLabel}
      >
        CVSS v3.0
      </a>
      <span className={styles.vectorBadge}>{score}</span>

      <div className={styles.scoreBlock}>
        <span className={[styles.scoreNumber, SCORE_CLASS[severity]].join(' ')}>
          {overall.toFixed(1)}
        </span>
        <ScoreBar overall={overall} />
      </div>

      <hr className={styles.divider} />

      <div className={styles.subScores}>
        {subScores.map(([subLabel, value]) => (
          <SubScoreChip key={subLabel} label={subLabel} value={value} />
        ))}
      </div>

      <hr className={styles.divider} />

      <div className={styles.metricCards}>
        {metricCategories.map(({ title, metrics }) => (
          <CvssMetricsCard key={title} title={title} metrics={metrics} />
        ))}
      </div>
    </div>
  );
}
