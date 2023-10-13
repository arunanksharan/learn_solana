import { useState } from "react";
import React from "react";
import Button from "./Button";

interface SearchFormPropsInterface {
  searchHandler: (address: string) => void;
}

const SearchForm = ({ searchHandler }: SearchFormPropsInterface) => {
  const [address, setAddress] = useState<string>("");

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    searchHandler(address);
    setAddress("");
  };

  const handleAddressInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setAddress(e.target.value);
  };
  return (
    <form
      onSubmit={handleSubmit}
      className="flex flex-col justify-center items-center w-3/4"
    >
      <input
        id="public-key"
        type="text"
        value={address}
        placeholder="Enter address or ID"
        className="px-4 py-2 w-full border rounded-md m-10 text-black font-mono font-medium"
        onChange={handleAddressInputChange}
      ></input>
      <Button></Button>
    </form>
  );
};

export default SearchForm;
