import React from "react";

interface AddressPropsInterface {
  address: string;
}

const Address = ({ address }: AddressPropsInterface) => {
  return (
    <div className="flex flex-row justify-center items-center w-full h-1/3 text-base font-mono">
      {address.slice(0, 10)}
    </div>
  );
};

export default Address;
