import cx from "clsx";
import { useNavigate, useLocation } from "react-router-dom";
import { createBEM } from "@/utils/css";
import Button from "@/components/Button/Button";
import Card from "@/components/Card";
import Icon from "@/components/Icon";
import "./style.css";

const b = createBEM("sidebar");

export default function Sidebar() {
  const navigate = useNavigate();
  const { pathname } = useLocation();

  return (
    <aside className={cx(b())}>
      <SidebarHeader
        title="象数观止"
        desc="最专业的北派紫微斗数排盘工具"
        logo={<Icon name="Logo" className={cx(b("header__logo"))} />}
      />
      <SidebarToolbar
        options={[
          {
            icon: <Icon name="Mask" />,
            label: "命例",
            onClick() {
              navigate("/customer");
            },
          },
          {
            icon: <Icon name="Mask" />,
            label: "待定",
          },
        ]}
      />
      <SidebarNav
        onClick={() => {
          if (pathname !== "/") {
            navigate("/");
          }
        }}
      />
      <SidebarFooter
        options={[
          {
            icon: <Icon name="Setting" />,
            onClick() {
              navigate("/setting");
            },
          },
          {
            icon: <Icon name="Github" />,
            onClick() {
              window.open(`https://github.com/ziweijs/insight`, "_blank");
            },
          },
        ]}
        extra={
          <Button icon={<Icon name="Add" className="mr-[5px]" />}>
            快速排盘
          </Button>
        }
      />
    </aside>
  );
}

export interface SidebarHeaderProps {
  title?: string;
  desc?: string;
  logo?: React.ReactNode;
}

export function SidebarHeader({ title, desc, logo }: SidebarHeaderProps) {
  return (
    <header className={cx(b("header"))}>
      <div>
        <h1 className={cx(b("header__title"))}>{title}</h1>
        <span className={cx(b("header__desc"))}>{desc}</span>
      </div>
      {logo}
    </header>
  );
}

export interface SidebarToolbarProps {
  options: {
    label: string;
    icon?: React.ReactNode;
    onClick?: () => void;
  }[];
}

export function SidebarToolbar({ options }: SidebarToolbarProps) {
  return (
    <div className={b("toolbar")}>
      {options.map((item) => (
        <Button
          key={item.label}
          className={b("toolbar__action")}
          icon={<div className="mr-[5px]">{item.icon}</div>}
          onClick={item.onClick}
        >
          {item.label}
        </Button>
      ))}
    </div>
  );
}

export interface SidebarNavProps {
  onClick?: () => void;
}

export function SidebarNav({ onClick }: SidebarNavProps) {
  return (
    <nav className={cx(b("nav"))} onClick={onClick}>
      <Card title="匿名" desc="1999年12月30日" close />
      <Card title="匿名" desc="1999年12月30日" close />
      <Card title="匿名" desc="1999年12月30日" close />
    </nav>
  );
}

export interface SidebarFooterProps {
  options: {
    icon?: React.ReactNode;
    label?: React.ReactNode;
    onClick?: () => void;
  }[];
  extra?: React.ReactNode;
}

export function SidebarFooter({ options, extra }: SidebarFooterProps) {
  return (
    <footer className={cx(b("footer"))}>
      <div className={cx(b("footer__actions"))}>
        {options.map(({ label, ...item }, index) => (
          <Button key={`footer__action.${index}`} {...item}>
            {label}
          </Button>
        ))}
      </div>
      {extra}
    </footer>
  );
}
