import "./button.css"

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  type?: "button" | "submit" | "reset";
}

export const Button: React.FC<ButtonProps> = ({ type, children, onClick }) => {
  return <button type={type} onClick={onClick} className="ui-button">{children}</button>;
};
