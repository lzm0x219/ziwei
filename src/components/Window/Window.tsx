import cx from "clsx";
import Sidebar from "@/components/Sidebar";
import { createBEM } from "@/utils/css";
import "./style.css";

export interface WindowProps {
  fullscreen?: boolean;
  children?: React.ReactNode;
}

const b = createBEM("window");

export default function Window({ children }: WindowProps) {
  return (
    <main className={cx(b())}>
      <Sidebar />
      <div className={cx(b("content"))}>{children}</div>
    </main>
  );
}
