import React from "react";
import Balance from "./Balance";
import BalanceCard from "./BalanceCard";
import { AddressBalanceMapping } from "./HomePage";

interface AddressBalanceListPropsInterface {
  addressBalanceList: AddressBalanceMapping[];
}

const BalanceList = ({
  addressBalanceList,
}: AddressBalanceListPropsInterface) => {
  return (
    <div className="flex flex-wrap justify-center w-full h-1/5">
      {addressBalanceList.map((element, idx: number) => (
        <BalanceCard
          key={idx}
          address={element.address}
          balance={element.balance}
        ></BalanceCard>
      ))}
    </div>
  );
};

export default BalanceList;
