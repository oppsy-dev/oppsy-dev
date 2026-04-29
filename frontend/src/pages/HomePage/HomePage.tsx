import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { AppRoute } from '../../routes/Routes';

export function HomePage() {
  const navigate = useNavigate();

  useEffect(() => {
    navigate(AppRoute.WorkspacesDashboard, { replace: true });
  }, [navigate]);

  return null;
}
