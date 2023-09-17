import { useSyncExternalStore } from "react";
import {
  darkModeQuery,
  getColorScheme,
  setColorScheme,
} from "@/utils/color-scheme";

const subscribe = (fn: () => void) => {
  darkModeQuery.addEventListener("change", fn);
  return () => {
    if (darkModeQuery) {
      darkModeQuery.removeEventListener("change", fn);
    }
  };
};

const getSnapshot = () => getColorScheme();

export default function useColorScheme() {
  const colorScheme = useSyncExternalStore(subscribe, getSnapshot);
  return { colorScheme, setColorScheme };
}
