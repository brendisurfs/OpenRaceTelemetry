import type { FC } from "react";

interface WelcomeSquareProps {
  label: string;
}

const WelcomeSquare: FC<WelcomeSquareProps> = ({ label }) => {
  return (
    <div>
      <div>{label}</div>
      <div>Upload a recording, or connect to a device</div>
    </div>
  );
};

export default WelcomeSquare;
