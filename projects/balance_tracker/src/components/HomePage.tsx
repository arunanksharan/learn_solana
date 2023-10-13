import React from "react";
import Title from "@/components/Title";
import SearchForm from "@/components/SearchForm";
import BalanceList from "@/components/BalanceList";
import type { NextPage } from "next";
import { useState, useEffect } from "react";
import * as web3 from "@solana/web3.js";

export type AddressBalanceMapping = {
  address: string;
  balance: number;
};

const HomePage: React.FC = () => {
  const [balance, setBalance] = useState<number>(0);
  const [address, setAddress] = useState<string>("");
  const [addressBalanceMapping, setAddressBalanceMapping] = useState<
    AddressBalanceMapping[]
  >([]);
  const [invalidAddress, setInvalidAddress] = useState<boolean>(false);
  const [addressError, setAddressError] = useState<string | null>(null);

  // Console Log addressBalanceMapping
  useEffect(() => {
    console.log(`Inside useeffect: ${JSON.stringify(addressBalanceMapping)}`);
  }, [addressBalanceMapping]);

  const addressSubmittedHandler = async (address: string) => {
    try {
      setAddress(address);
      const publicKey = new web3.PublicKey(address);
      const connection = new web3.Connection(web3.clusterApiUrl("devnet"));
      const balance = await connection.getBalance(publicKey);
      setBalance(balance / web3.LAMPORTS_PER_SOL);
      console.log(balance);
      setAddressBalanceMapping((prevState) => {
        return [...prevState, { address, balance }];
      });

      setAddressError(null);
    } catch (error: any) {
      console.log(error);
      setAddressError(error.message);
    }
  };

  return (
    <main className="flex flex-col justify-around items-center h-screen w-screen">
      <Title></Title>
      <SearchForm searchHandler={addressSubmittedHandler}></SearchForm>
      {addressError && <p className="text-red-500">{addressError}</p>}
      {/* <p>{`Address: ${address}`}</p>
      <p>{`Balance: ${balance}`}</p> */}
      <BalanceList addressBalanceList={addressBalanceMapping}></BalanceList>
    </main>
  );
};

export default HomePage;
