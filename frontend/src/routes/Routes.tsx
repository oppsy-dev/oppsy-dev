import { Outlet, Route, Routes } from 'react-router-dom';
import { AppShell } from '../components/AppShell/AppShell';
import { HomePage } from '../pages/HomePage/HomePage';
import { WorkspacesDashboardPage } from '../pages/WorkspacesDashboardPage/WorkspacesDashboardPage';
import { ChannelsDashboardPage } from '../pages/ChannelsDashboardPage/ChannelsDashboardPage';
import { NotFoundPage } from '../pages/NotFoundPage/NotFoundPage';
import { WorkspacePage } from '../pages/WorkspacePage/WorkspacePage';
import { OsvRecordPage } from '../pages/OsvRecordPage/OsvRecordPage';
import { ChannelPage } from '../pages/ChannelPage/ChannelPage';

export enum AppRoute {
  Home = '/',
  WorkspacesDashboard = '/workspaces',
  Workspace = '/workspaces/:workspaceId',
  TeamDashboard = '/dashboard/teams/:teamId',
  ChannelsDashboard = '/channels',
  Channel = '/channels/:channelId',
  OsvRecord = '/osv/record/:name',
}

export function AppRoutes() {
  return (
    <Routes>
      <Route path={AppRoute.Home} element={<HomePage />} />

      <Route
        element={
          <AppShell>
            <Outlet />
          </AppShell>
        }
      >
        <Route path={AppRoute.WorkspacesDashboard} element={<WorkspacesDashboardPage />} />
        <Route path={AppRoute.Workspace} element={<WorkspacePage />} />
        <Route path={AppRoute.ChannelsDashboard} element={<ChannelsDashboardPage />} />
        <Route path={AppRoute.Channel} element={<ChannelPage />} />
        <Route path={AppRoute.TeamDashboard} element={<WorkspacesDashboardPage />} />
      </Route>

      <Route path={AppRoute.OsvRecord} element={<OsvRecordPage />} />
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
