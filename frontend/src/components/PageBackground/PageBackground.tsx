import styles from './PageBackground.module.css';

type PageBackgroundProps = {
  is_top_glow?: boolean;
};

export function PageBackground({ is_top_glow = true }: PageBackgroundProps = {}) {
  return (
    <>
      <div className={styles.grid} aria-hidden="true" />
      {is_top_glow && <div className={styles.top_glow} aria-hidden="true" />}
    </>
  );
}
