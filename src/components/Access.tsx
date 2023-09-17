import { Fragment } from "react";

export interface AccessProps {
  accessible: boolean;
  element?: React.ReactNode;
  fallback?: React.ReactNode;
  children?: React.ReactNode;
}

export default function Access({
  accessible,
  element,
  fallback,
  children,
}: AccessProps) {
  return <Fragment>{accessible ? element ?? children : fallback}</Fragment>;
}
