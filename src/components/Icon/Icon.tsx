type IconName =
  | "Add"
  | "Delete"
  | "Dragon"
  | "Github"
  | "Logo"
  | "Mask"
  | "Setting";

const components = import.meta.glob<React.FC<React.SVGProps<SVGSVGElement>>>(
  "./*.svg",
  {
    eager: true,
    import: "ReactComponent",
  },
);

const icons = Object.entries(components).reduce<
  Map<IconName, React.FC<React.SVGProps<SVGSVGElement>>>
>((result, [path, component]) => {
  result.set(path.slice(6, path.length - 4) as IconName, component);
  return result;
}, new Map());

export interface IconProps extends React.SVGProps<SVGSVGElement> {
  name: IconName;
}

export default function Icon({ name, ...props }: IconProps) {
  const Component = icons.get(name)!;
  return <Component {...props} />;
}
