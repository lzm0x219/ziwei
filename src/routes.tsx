/* eslint-disable react-refresh/only-export-components */
import { Suspense, lazy } from "react";
import type { RouteObject } from "react-router-dom";
import Loading from "@/components/Loading";
import App from "@/App";

export const Index = lazy(() => import("@/pages/index"));
export const Customer = lazy(() => import("@/pages/customer"));
export const Setting = lazy(() => import("@/pages/setting"));
export const Notfound = lazy(() => import("@/pages/404"));

export const routes: RouteObject[] = [
  {
    path: "/",
    element: <App />,
    children: [
      {
        path: "/",
        element: (
          <Suspense fallback={<Loading />}>
            <Index />
          </Suspense>
        ),
      },
      {
        path: "/customer",
        element: (
          <Suspense fallback={<Loading />}>
            <Customer />
          </Suspense>
        ),
      },
      {
        path: "/setting",
        element: (
          <Suspense fallback={<Loading />}>
            <Setting />
          </Suspense>
        ),
      },
    ],
  },
  {
    path: "*",
    element: <Notfound />,
  },
];
