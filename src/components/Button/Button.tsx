import cx from "clsx";
import { createBEM } from "@/utils/css";
import "./style.css";

export interface ButtonProps
  extends React.DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {
  icon?: React.ReactNode;
  variant?: ButtonVariant;
  color?: ButtonColor;
  size?: ButtonSize;
  shadow?: boolean;
  children?: React.ReactNode;
}

export type ButtonVariant = "contained" | "soft" | "text";

export type ButtonColor = "primary" | "danger" | "default";

export type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";

const b = createBEM("button");

export default function Button({
  children,
  icon,
  className,
  variant = "contained",
  color = "default",
  size = "md",
  shadow = false,
  disabled = false,
  ...props
}: ButtonProps) {
  return (
    <button
      className={cx([
        b([variant, color, size, { shadow, disabled }]),
        className,
      ])}
      {...props}
    >
      {icon}
      {children}
    </button>
  );
}
