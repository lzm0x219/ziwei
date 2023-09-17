import cx from "clsx";
import { createBEM } from "@/utils/css";
import Icon from "@/components/Icon";
import Access from "@/components/Access";
import "./style.css";

const b = createBEM("card");

export interface CardProps {
  title?: React.ReactNode;
  desc?: React.ReactNode;
  other?: React.ReactNode;
  close?: boolean;
  onClick?: () => void;
  onClose?: () => void;
}

export default function Card({
  title,
  desc,
  other,
  close = false,
  onClick,
  onClose,
}: CardProps) {
  return (
    <div
      className={cx(b())}
      onClick={(e) => {
        e.stopPropagation();
        onClick?.();
      }}
    >
      <div className={cx(b("title"))}>{title}</div>
      <div className={cx(b("content"))}>
        <span className={cx(b("content_desc"))}>{desc}</span>
        <span className={cx(b("content_other"))}>{other}</span>
      </div>
      <Access accessible={close}>
        <Icon name="Delete" className={cx(b("close"))} onClick={onClose} />
      </Access>
    </div>
  );
}
