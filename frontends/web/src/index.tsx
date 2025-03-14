// File: frontend/src/index.tsx
import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { AuthPageRoutes } from "./ui/pages/AuthPages/routes";
import "./index.css";

/**
 * Similar to the "compileRoutes" pattern, we just return an array of route objects.
 * Each object can have `path`, `element`, children, etc.
 */
function getAllRoutes() {
  const baseRoutes = [
    {
      path: "*",
      element: <h1>404</h1>,
    },
  ];

  // Merge your AuthPageRoutes with baseRoutes
  return [...AuthPageRoutes, ...baseRoutes];
}

// Create the router from your compiled routes
const router = createBrowserRouter(getAllRoutes());

// Render the router inside <RouterProvider>
const rootElement = document.getElementById("root");
if (rootElement) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(<RouterProvider router={router} />);
}

// If you need to export a component for Storybook:
export const App = () => <RouterProvider router={router} />;
export default App;
