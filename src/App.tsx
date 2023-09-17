import { Outlet } from "react-router-dom";
import Window from "@/components/Window";
import useColorScheme from "@/hooks/useColorScheme";

export default function App() {
  const { colorScheme } = useColorScheme();

  console.log(colorScheme);

  return (
    <Window>
      <Outlet />
    </Window>
  );
}
