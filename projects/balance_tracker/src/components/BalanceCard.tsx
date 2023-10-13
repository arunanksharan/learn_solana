import React from "react";
import Address from "./Address";
import Balance from "./Balance";

interface BalanceCardPropsInterface {
  address: string;
  balance: number;
}

const BalanceCard = ({ address, balance }: BalanceCardPropsInterface) => {
  return (
    <div className="bg-secondary w-1/3 flex flex-col justify-center items-center rounded-lg m-4 p-4">
      <Address address={address}></Address>
      <Balance balance={balance}></Balance>
    </div>
  );
};

export default BalanceCard;
