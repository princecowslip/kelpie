import { ThemeProvider } from "../theme/ThemeProvider";
import { AppRouter } from "./router";

export function App() {
  return (
    <ThemeProvider>
      <AppRouter />
    </ThemeProvider>
  );
}
