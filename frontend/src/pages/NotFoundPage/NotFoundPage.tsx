import { Link } from 'react-router-dom';
import { AppRoute } from '../../routes/Routes';
import { PageBackground } from '../../components/PageBackground/PageBackground';
import styles from './NotFoundPage.module.css';

export function NotFoundPage() {
  return (
    <main className={styles.container}>
      <PageBackground />

      <img src="/logo.svg" className={styles.logoBg} alt="" aria-hidden="true" />
      <div className={styles.code} aria-hidden="true">
        404
      </div>
      <p className={styles.oopsy}>Oopsy</p>
      <h1 className={styles.heading}>Page not found</h1>
      <p className={styles.description}>
        The page you're looking for doesn't exist or has been moved.
      </p>
      <Link to={AppRoute.WorkspacesDashboard} className={styles.homeLink}>
        Go home
      </Link>
    </main>
  );
}
