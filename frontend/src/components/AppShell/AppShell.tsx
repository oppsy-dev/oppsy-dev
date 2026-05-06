import type { ReactNode } from 'react';
import { LeftSidebar } from './LeftSidebar/LeftSidebar';
import { RightSidebar } from './RightSidebar/RightSidebar';
import styles from './AppShell.module.css';

type AppShellProps = {
  children: ReactNode;
};

export function AppShell({ children }: AppShellProps) {
  return (
    <div className={styles.shell}>
      <LeftSidebar />
      <main className={styles.main}>{children}</main>
      <RightSidebar />
    </div>
  );
}
