import type { FC } from "react";

interface MetricCardProps {
  title: string;
  value: string | number;
}

const MetricCard: FC<MetricCardProps> = ({ title, value }) => {
  return (
    <div>
      <div>{title}</div>
      <div>{value}</div>
    </div>
  );
};

export default MetricCard;
