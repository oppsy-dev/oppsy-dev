import styles from './CvssMetricsCard.module.css';

export type CvssMetric = {
  title: string;
  metricDescription: string;
  value: string;
  description: string;
};

type CvssMetricsCardProps = {
  title: string;
  metrics: CvssMetric[];
};

export function CvssMetricsCard({ title, metrics }: CvssMetricsCardProps) {
  return (
    <div className={styles.card}>
      <span className={styles.title}>{title}</span>
      <div className={styles.metricList}>
        {metrics.map((metric) => (
          <div key={metric.title} className={styles.row}>
            <div className={styles.rowLeft}>
              <span className={styles.metricTitle}>{metric.title}</span>
              <span className={styles.tooltipAnchor} aria-label={metric.metricDescription}>
                <span className={styles.questionMark}>?</span>
                <span className={[styles.tooltip, styles.tooltipRight].join(' ')} role="tooltip">
                  {metric.metricDescription}
                </span>
              </span>
            </div>
            <div className={styles.rowRight}>
              <span className={styles.value}>{metric.value}</span>
              <span className={styles.tooltipAnchor} aria-label={metric.description}>
                <span className={styles.questionMark}>?</span>
                <span className={styles.tooltip} role="tooltip">
                  {metric.description}
                </span>
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
