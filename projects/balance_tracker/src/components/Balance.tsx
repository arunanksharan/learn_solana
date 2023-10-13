import React from "react";
import Address from "./Address";

interface BalancePropsInterface {
  balance: number;
}

const Balance = ({ balance }: BalancePropsInterface) => {
  return (
    <div className="flex flex-col justify-center items-center w-full h-2/3">
      <div className="w-full h-2/3 flex flex-row justify-center items-center text-5xl font-mono font-semibold">
        {balance}
      </div>
      <div className="w-full h-1/3 flex flex-row justify-center items-center text-base font-mono font-medium">
        SOL
      </div>
    </div>
  );
};

export default Balance;
