import "./button.css";

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  type?: "button" | "submit" | "reset";
  className?: string;
}

export const Button: React.FC<ButtonProps> = ({
  className,
  type,
  children,
  onClick,
}) => {
  return (
    <button type={type} onClick={onClick} className={`ui-button ${className}`}>
      {children}
    </button>
  );
};
